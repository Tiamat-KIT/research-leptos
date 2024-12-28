use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window    
};


#[derive(Default)]
pub struct WgpuDrawApp {
    window: Option<Window>,
}

impl ApplicationHandler for WgpuDrawApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }
    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}


pub fn drawing() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = WgpuDrawApp::default();
    event_loop.run_app(&mut app);
}