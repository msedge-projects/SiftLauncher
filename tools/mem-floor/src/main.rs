//! Framework floor: egui + winit + softbuffer, text laid out, nothing drawn.
use std::num::NonZeroU32;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    ctx: egui::Context,
    window: Option<Rc<Window>>,
    _surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    frames: u32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(LogicalSize::new(1440.0, 900.0))
                        .with_title("floor"),
                )
                .expect("window"),
        );
        let context = softbuffer::Context::new(window.clone()).expect("ctx");
        self._surface = Some(softbuffer::Surface::new(&context, window.clone()).expect("surface"));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let raw = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 900.0))),
                    ..Default::default()
                };
                let output = self.ctx.run_ui(raw, |ui| {
                    for (i, size) in [12.0_f32, 13.0, 14.0, 16.0, 18.0, 22.0].iter().enumerate() {
                        let family = format!("inter-{}", [400, 500, 600, 700, 800][i % 5]);
                        ui.label(
                            egui::RichText::new("Library  Instance 3  Fabric 26.3  Never played  Search  Settings  Mods")
                                .font(egui::FontId::new(*size, egui::FontFamily::Name(family.into()))),
                        );
                    }
                });
                let _ = self.ctx.tessellate(output.shapes, output.pixels_per_point);
                self.frames += 1;
                if self.frames < 3 {
                    if let Some(window) = self.window.as_ref() { window.request_redraw(); }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}
}

fn main() {
    let ctx = egui::Context::default();
    let mut fonts = egui::FontDefinitions::default();
    for (family, file) in [
        ("inter-400", include_bytes!("../../../assets/fonts/Inter-400.ttf").as_slice()),
        ("inter-500", include_bytes!("../../../assets/fonts/Inter-500.ttf").as_slice()),
        ("inter-600", include_bytes!("../../../assets/fonts/Inter-600.ttf").as_slice()),
        ("inter-700", include_bytes!("../../../assets/fonts/Inter-700.ttf").as_slice()),
        ("inter-800", include_bytes!("../../../assets/fonts/Inter-800.ttf").as_slice()),
    ] {
        fonts.font_data.insert(family.to_owned(), std::sync::Arc::new(egui::FontData::from_static(file)));
        fonts.families.insert(egui::FontFamily::Name(family.into()), vec![family.to_owned()]);
    }
    ctx.set_fonts(fonts);
    let event_loop = EventLoop::new().expect("loop");
    let mut app = App { ctx, window: None, _surface: None, frames: 0 };
    event_loop.run_app(&mut app).expect("run");
}