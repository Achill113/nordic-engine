use log::debug;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{exit_event, keyboard_event, mouse_event};

use super::render::Render;

pub struct Window {
    event_loop: Option<EventLoop<()>>,
    pub window: winit::window::Window,
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Nordic Engine")
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop: Some(event_loop),
            window,
        }
    }

    pub fn run(mut self, mut render: Render) {
        let win_id = self.window.id();

        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == win_id => {
                        if !render.input(event) {
                            match event {
                                exit_event!() => {
                                    debug!("Exit");
                                    *control_flow = ControlFlow::Exit;
                                }
                                mouse_event!() => {
                                    debug!("Mouse Event");
                                    debug!("{:?}", event);
                                }
                                keyboard_event!() => {
                                    debug!("Keyboard Event");
                                    debug!("{:?}", event);
                                }
                                _ => {}
                            }
                        }
                    }
                    Event::RedrawRequested(window_id) if window_id == win_id => {
                        debug!("Redraw");
                        render.update();
                        match render.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => render.resize(render.size),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    Event::MainEventsCleared => {
                        self.window.request_redraw();
                    }
                    _ => {}
                }
            });
    }
}
