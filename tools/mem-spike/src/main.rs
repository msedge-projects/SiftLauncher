use std::num::NonZeroU32;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(LogicalSize::new(1440.0, 900.0))
                        .with_title("spike"),
                )
                .expect("window"),
        );
        let context = softbuffer::Context::new(window.clone()).expect("ctx");
        let surface = softbuffer::Surface::new(&context, window.clone()).expect("surface");
        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut()) else {
                    return;
                };
                let size = window.inner_size();
                let (w, h) = (NonZeroU32::new(size.width.max(1)).unwrap(), NonZeroU32::new(size.height.max(1)).unwrap());
                surface.resize(w, h).expect("resize");
                let mut buffer = surface.buffer_mut().expect("buffer");
                let mut pixmap = tiny_skia::Pixmap::new(size.width.max(1), size.height.max(1)).expect("pixmap");
                pixmap.fill(tiny_skia::Color::from_rgba8(24, 24, 30, 255));
                let mut paint = tiny_skia::Paint::default();
                paint.set_color_rgba8(27, 217, 107, 255);
                let rect = tiny_skia::Rect::from_xywh(40.0, 40.0, 300.0, 120.0).unwrap();
                pixmap.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
                for (i, px) in pixmap.pixels().iter().enumerate() {
                    let c = px.demultiply();
                    buffer[i] = ((c.alpha() as u32) << 24) | ((c.red() as u32) << 16) | ((c.green() as u32) << 8) | c.blue() as u32;
                }
                buffer.present().expect("present");
            }
            WindowEvent::Resized(_) => {
                if let Some(window) = self.window.as_ref() { window.request_redraw(); }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() { window.request_redraw(); }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("loop");
    let mut app = App { window: None, surface: None };
    event_loop.run_app(&mut app).expect("run");
}
