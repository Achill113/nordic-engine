use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget, EventLoopBuilder},
    window::{WindowBuilder, WindowId},
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the window loop.
    /// 
    /// This should be the last thing called on App.
    pub fn run(&self) -> () {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Nordic Engine").build(&event_loop).unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                _ => {}
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_run_should_run_event_loop() {
        let app = App::new();

        app.run();
    }
}
