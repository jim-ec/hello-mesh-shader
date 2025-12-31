mod render;

use std::{cell::OnceCell, sync::Arc};

use render::Renderer;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: OnceCell<Arc<Window>>,
    renderer: OnceCell<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title(env!("CARGO_PKG_NAME")))
                .unwrap(),
        );
        self.window.set(window.clone()).unwrap();

        let renderer = Renderer::new(window);
        self.renderer
            .set(futures::executor::block_on(renderer))
            .unwrap();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer.take();
        self.window.take();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.renderer.get_mut().unwrap().resize(size);
                self.window.get().unwrap().request_redraw();
            }

            WindowEvent::RedrawRequested => {
                let renderer = self.renderer.get_mut().unwrap();
                renderer.render();
                self.window.get().unwrap().request_redraw();
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        repeat: false,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }

            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
