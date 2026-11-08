//! The window: winit events in, egui frames out, drawn by `Painter` straight into the presentable buffer.

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};

use egui;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::ModifiersState;
use winit::window::{Window, WindowId};

use super::Painter;
use crate::shell::Shell;

/// What the window knows between frames.
struct Soft {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    painter: Painter,
    shell: Shell,
    ctx: egui::Context,
    scale: f32,
    modifiers: ModifiersState,
    pointer: egui::Pos2,
    /// Events collected since the last frame.
    pending: Vec<egui::Event>,
    /// Paths winit reported as dropped, handed to the shell at the start of the next frame.
    dropped: Vec<std::path::PathBuf>,
    /// When an *injected* drop gives up waiting for an instance page. A real one arrives whenever the reader drops it.
    drop_deadline: Option<std::time::Instant>,
    /// A frame is only run when something happened or an animation asked for one: an idle window draws nothing, which is
    /// this project's lightness rule and the reason a fan does not spin.
    dirty: bool,
    /// The last frame's rasteriser and present times, for `SIFT_SOFT_TRACE`.
    painted: std::time::Duration,
    presented: std::time::Duration,
    /// Frame timing, in the same format the OpenGL path writes: the lightness gate is a p95, and a backend that never
    /// records one cannot be held to it.
    log: crate::frame_log::FrameLog,
    next_frame: Option<Instant>,
}

/// Run the launcher with the software backend.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let ctx = egui::Context::default();
    crate::fonts::install(&ctx);
    crate::serverart::install(&ctx);
    let dark = crate::theme::dark();
    let mut app = Soft {
        window: None,
        surface: None,
        painter: Painter::new(dark.BG),
        shell: Shell::default(),
        ctx,
        scale: 1.0,
        modifiers: ModifiersState::empty(),
        pointer: egui::Pos2::ZERO,
        pending: Vec::new(),
        // **A drop, injected.** Xvfb has no file manager to drag from, so the same event winit would deliver is queued
        // from `SIFT_DROP_PATHS` (colon-separated) at start-up. Everything downstream -- `Shell::dropped`, the copy, the
        // toast -- is the path an OS drop takes.
        dropped: std::env::var_os("SIFT_DROP_PATHS")
            .map(|value| {
                value
                    .to_string_lossy()
                    .split(':')
                    .filter(|part| !part.is_empty())
                    .map(std::path::PathBuf::from)
                    .collect()
            })
            .unwrap_or_default(),
        drop_deadline: std::env::var_os("SIFT_DROP_PATHS").map(|_| std::time::Instant::now() + std::time::Duration::from_secs(30)),
        dirty: true,
        painted: std::time::Duration::ZERO,
        presented: std::time::Duration::ZERO,
        next_frame: None,
        log: crate::frame_log::FrameLog::default(),
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}

impl Soft {
    /// Run one egui frame and paint it. This mirrors `main.rs`'s `eframe::App::ui` exactly -- same panel, same modals in
    /// the same order -- because two backends that draw different things are two products.
    fn frame(&mut self) {
        let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut()) else {
            return;
        };
        let size = window.inner_size();
        if std::env::var_os("SIFT_SCALE_PROBE").is_some() {
            eprintln!(
                "frame inner={:?} scale={} ppp={}",
                size,
                self.scale,
                self.ctx.pixels_per_point(),
            );
        }
        let (width, height) = (size.width as usize, size.height as usize);
        if width == 0 || height == 0 {
            return;
        }

        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32 / self.scale, height as f32 / self.scale),
            )),
            ..Default::default()
        };
        // The events collected by the window handlers since the last frame ride along with the input.
        let events = std::mem::take(&mut self.pending);
        let raw = egui::RawInput { events, ..raw };

        self.log.begin();
        let mut animating = false;
        let shell = &mut self.shell;
        let output = self.ctx.run_ui(raw, |ui| {
            let ctx = ui.ctx().clone();
            let screen = ui.max_rect();
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| {
                    // **Before the frame, the drop.** What a dropped file means depends on what is open, and
                    // `Shell::dropped` answers that; the toast it sets is drawn by this frame.
                    // An injected drop waits briefly for an instance page (so the copy path can be driven); a real one
                    // is delivered at once, and with no instance open it says so.
                    let drop_ready = match self.drop_deadline {
                        Some(at) => shell.files_open() || std::time::Instant::now() >= at,
                        None => true,
                    };
                    if drop_ready && !self.dropped.is_empty() {
                        let paths = std::mem::take(&mut self.dropped);
                        shell.dropped(paths);
                    }
                    animating = shell.ui(ui);
                    if shell.picker.is_some() {
                        ctx.request_repaint();
                    }
                    // **Promoted before the guard.** `Action::Create` (the welcome screen's button and its `n`) carries no
                    // clock; this frame has one, and the dialog only draws once `create_open` is true -- so the request is
                    // turned into state here, one line before the check it feeds. The GL path in `main.rs` had this first and
                    // it did nothing: the software backend is what this build runs, and it draws its own dialog.
                    if shell.create_open_pending {
                        shell.create_open = true;
                        shell.create_opened = ctx.input(|input| input.time);
                        shell.create_open_pending = false;
                        ctx.request_repaint();
                    }
                    if shell.create_open {
                        shell.draw_create(&ctx, screen, ui);
                    }
                    if shell.edit.is_some() {
                        shell.draw_edit(&ctx, screen, ui);
                        ctx.request_repaint();
                    }
                    if shell.picker.is_some() {
                        shell.draw_picker(&ctx, screen, ui);
                    }
                });
            if shell.create_open {
                ctx.request_repaint();
            }
        });

        self.painter.apply(&output.textures_delta);
        let primitives = self.ctx.tessellate(output.shapes, output.pixels_per_point);

        let resized = surface.resize(
            NonZeroU32::new(size.width.max(1)).unwrap(),
            NonZeroU32::new(size.height.max(1)).unwrap(),
        );
        if std::env::var_os("SIFT_SCALE_PROBE").is_some() {
            match &resized {
                Ok(()) => eprintln!("resize ok {width}x{height}"),
                Err(error) => eprintln!("resize FAILED {width}x{height}: {error}"),
            }
        }
        let _ = resized;
        if let Ok(mut buffer) = surface.buffer_mut() {
            if std::env::var_os("SIFT_SCALE_PROBE").is_some() {
                eprintln!("buffer len={} expect={}", buffer.len(), width * height);
            }
            // **Into the presentable buffer, not a pixmap that is then copied.** That is the 4.94 MiB this backend saves
            // over the spike, and the reason the launcher can sit under the gate where a pixmap-based path cannot.
            let paint_started = Instant::now();
            self.painter
                .paint(&primitives, width, height, &mut buffer);
            let painted = paint_started.elapsed();
            let present_started = Instant::now();
            let presented_result = buffer.present();
            if std::env::var_os("SIFT_SCALE_PROBE").is_some() {
                match &presented_result {
                    Ok(()) => eprintln!("present ok {width}x{height}"),
                    Err(error) => eprintln!("present FAILED {width}x{height}: {error:?}"),
                }
            }
            let _ = presented_result;
            // **The two halves, timed separately.** The gate is 16 ms; without this split a frame is one opaque number, and
            // the profile it produced says paint is 21 ms of a 22 ms frame while the present is 0.01 ms.
            self.painted = painted;
            self.presented = present_started.elapsed();
        }
        // **After the paint, not before it.** The first wiring closed the timer here earlier and reported a p95 of 9.68 ms
        // that quietly excluded the rasteriser -- the expensive half. The gate is about the whole frame.
        self.log.end();

        if std::env::var("SIFT_SOFT_MEM").is_ok() {
            eprintln!(
                "textures {} kB, textures {}",
                self.painter.texture_bytes() / 1024,
                self.painter.len()
            );
        }
        if std::env::var("SIFT_SOFT_TRACE").is_ok() {
            eprintln!(
                "frame animating={animating} delay_zero={} paint_ms={:.2} present_ms={:.2}",
                output.viewport_output.get(&egui::ViewportId::ROOT).map(|v| v.repaint_delay.is_zero()).unwrap_or(false),
                self.painted.as_secs_f64() * 1000.0,
                self.presented.as_secs_f64() * 1000.0
            );
        }
        self.dirty = false;
        let delay = output
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .map(|viewport| viewport.repaint_delay)
            .unwrap_or(Duration::MAX);
        // **Never schedule the next frame for "now".** An animating page asked for an immediate repaint every time, and with
        // no vsync behind a software rasteriser that means re-drawing as fast as the machine can -- measured at 95% of a core
        // for an idle launcher, on a box already at load 9. One frame per 16 ms is 60 fps: the most a reader can see, and the
        // difference between a launcher and a space heater.
        const FRAME_FLOOR: Duration = Duration::from_millis(16);
        let now = Instant::now();
        self.next_frame = if animating || delay.is_zero() {
            Some(now + FRAME_FLOOR)
        } else if delay == Duration::MAX {
            None
        } else {
            Some(now + delay.max(FRAME_FLOOR))
        };
    }
}

impl ApplicationHandler for Soft {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(LogicalSize::new(
                            crate::geometry::WINDOW.0,
                            crate::geometry::WINDOW.1,
                        ))
                        .with_min_inner_size(LogicalSize::new(960.0, 540.0))
                        .with_title("Sift Launcher")
                        .with_decorations(false),
                )
                .expect("a window"),
        );
        self.scale = window.scale_factor() as f32;
        self.ctx.set_pixels_per_point(self.scale);
        let context = softbuffer::Context::new(window.clone()).expect("a present context");
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).expect("a surface"));
        self.window = Some(window);
        self.dirty = true;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if std::env::var_os("SIFT_RESIZE_PROBE").is_some() {
                    eprintln!("resized to {size:?}");
                }
                self.dirty = true;
            }
            // **The frame happens here, not in `about_to_wait`.** It used to happen in both: the loop asked for a redraw,
            // took the redraw as a change, drew, asked again. Against a software rasteriser that ran the box at 95% of a core
            // for an idle window -- load average 9 on six CPUs -- and no amount of scheduling a later frame could break it,
            // because `dirty` was set by the very event the redraw produced.
            WindowEvent::RedrawRequested => {
                self.frame();
                self.dirty = false;
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale = scale_factor as f32;
                self.ctx.set_pixels_per_point(self.scale);
                self.dirty = true;
            }
            WindowEvent::ModifiersChanged(state) => {
                self.modifiers = state.state();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = super::point(position, self.scale);
                self.pending.push(egui::Event::PointerMoved(self.pointer));
                self.dirty = true;
            }
            // **A file dragged in from the desktop.** winit reports one per file; they are collected and handed to the shell
            // on the next frame, which decides what they were dropped on (`Shell::dropped`). Xvfb has no file manager to drag
            // *from*, so `SIFT_DROP_PATHS` injects the same event -- the copy, the toast and the folder are then driven the
            // same way an OS drop drives them.
            WindowEvent::DroppedFile(path) => {
                self.dropped.push(path);
                self.dirty = true;
            }
            WindowEvent::CursorLeft { .. } => {
                self.pending.push(egui::Event::PointerGone);
                self.dirty = true;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    MouseButton::Left => egui::PointerButton::Primary,
                    MouseButton::Right => egui::PointerButton::Secondary,
                    MouseButton::Middle => egui::PointerButton::Middle,
                    _ => return,
                };
                self.pending.push(egui::Event::PointerButton {
                    pos: self.pointer,
                    button,
                    pressed: state == ElementState::Pressed,
                    modifiers: modifiers(self.modifiers),
                });
                self.dirty = true;
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (unit, delta) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        (egui::MouseWheelUnit::Line, egui::vec2(x, y))
                    }
                    MouseScrollDelta::PixelDelta(pos) => (
                        egui::MouseWheelUnit::Point,
                        egui::vec2(pos.x as f32 / self.scale, pos.y as f32 / self.scale),
                    ),
                };
                self.pending.push(egui::Event::MouseWheel {
                    unit,
                    delta,
                    phase: egui::TouchPhase::Move,
                    modifiers: modifiers(self.modifiers),
                });
                self.dirty = true;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(text) = event.text.as_ref() {
                    if event.state == ElementState::Pressed {
                        self.pending.push(egui::Event::Text(text.to_string()));
                    }
                }
                if let Some(key) = super::map_key(&event) {
                    self.pending.push(egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: event.state == ElementState::Pressed,
                        repeat: event.repeat,
                        modifiers: modifiers(self.modifiers),
                    });
                }
                self.dirty = true;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let due = match self.next_frame {
            Some(at) if at <= Instant::now() => true,
            _ => false,
        };
        if self.dirty || due {
            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
            event_loop.set_control_flow(match self.next_frame {
                Some(at) => ControlFlow::WaitUntil(at),
                None => ControlFlow::Wait,
            });
        } else {
            event_loop.set_control_flow(match self.next_frame {
                Some(at) => ControlFlow::WaitUntil(at),
                None => ControlFlow::Wait,
            });
        }
    }
}

fn modifiers(state: ModifiersState) -> egui::Modifiers {
    egui::Modifiers {
        alt: state.alt_key(),
        ctrl: state.control_key(),
        shift: state.shift_key(),
        mac_cmd: false,
        command: state.control_key(),
    }
}
