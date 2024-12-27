use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window    
};
use crate::draw::initialize::InitilizeWgpu;


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
                wasm_bindgen_futures::spawn_local(
                    async move {
                        let init = InitilizeWgpu::new(
                            &window
                        ).await;
                        let output = init.surface.get_current_texture().unwrap();
                        let view = output.texture.create_view(
                            &wgpu::TextureViewDescriptor::default()
                        );
                        let mut encoder = init.device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("Command Encoder")
                            }
                        );

                        {
                            let render_pass = encoder.begin_render_pass(
                                &wgpu::RenderPassDescriptor { 
                                    label: Some("Render Pass"),
                                    color_attachments: &[
                                        Some(wgpu::RenderPassColorAttachment {
                                            view: &view,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                                    r: 0.0,
                                                    g: 0.0,
                                                    b: 0.0,
                                                    a: 0.0
                                                }),
                                                store: wgpu::StoreOp::Store
                                            },
                                        })
                                    ],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None
                                }
                            );
                            init.queue.submit(
                                std::iter::once(encoder.finish())
                            );
                            output.present();
                        }
                    }
                );
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