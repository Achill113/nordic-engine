use log::debug;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{exit_event, keyboard_event, mouse_event};

pub struct Window {
    event_loop: Option<EventLoop<()>>,
    window: winit::window::Window,
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
            window: window,
        }
    }

    pub fn run(mut self) -> () {
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
                    } => {
                        if window_id == win_id {
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
                    _ => {}
                }
            });
    }
}
