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
    /// A frame is only run when something happened or an animation asked for one: an idle window draws nothing, which is
    /// this project's lightness rule and the reason a fan does not spin.
    dirty: bool,
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
        dirty: true,
        next_frame: None,
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

        let mut animating = false;
        let shell = &mut self.shell;
        let output = self.ctx.run_ui(raw, |ui| {
            let ctx = ui.ctx().clone();
            let screen = ui.max_rect();
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| {
                    animating = shell.ui(ui);
                    if shell.picker.is_some() {
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

        surface
            .resize(
                NonZeroU32::new(size.width.max(1)).unwrap(),
                NonZeroU32::new(size.height.max(1)).unwrap(),
            )
            .ok();
        if let Ok(mut buffer) = surface.buffer_mut() {
            // **Into the presentable buffer, not a pixmap that is then copied.** That is the 4.94 MiB this backend saves
            // over the spike, and the reason the launcher can sit under the gate where a pixmap-based path cannot.
            self.painter
                .paint(&primitives, width, height, &mut buffer);
            let _ = buffer.present();
        }

        if std::env::var("SIFT_SOFT_MEM").is_ok() {
            eprintln!(
                "textures {} kB, textures {}",
                self.painter.texture_bytes() / 1024,
                self.painter.len()
            );
        }
        if std::env::var("SIFT_SOFT_TRACE").is_ok() {
            eprintln!("frame animating={animating} delay_zero={}", output.viewport_output.get(&egui::ViewportId::ROOT).map(|v| v.repaint_delay.is_zero()).unwrap_or(false));
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
            WindowEvent::Resized(_) => self.dirty = true,
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
