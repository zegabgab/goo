use winit::{self, application::ApplicationHandler, dpi::{LogicalSize, PhysicalSize}, event::WindowEvent, event_loop::ActiveEventLoop, platform::x11::WindowAttributesExtX11, window::{Window, WindowAttributes, WindowId}};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_atrributes = WindowAttributes::default()
        .with_title("finestra sinestra")
        .with_inner_size(LogicalSize::new(0.5, 0.5))
        .with_name("AN APP HAHA", "an app instance");
        self.window = Some(event_loop.create_window(window_atrributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping {}", self.window.as_ref().map(Window::title).unwrap_or(String::from("[no title found]")));
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                println!("redraw redraw");
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Couldn't construct event loop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Error running app");
}
