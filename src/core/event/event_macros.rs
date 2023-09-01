#[macro_export]
macro_rules! mouse_event {
    () => {
        WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorMoved { .. }
            | WindowEvent::CursorLeft { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::MouseWheel { .. }
    };
}

#[macro_export]
macro_rules! exit_event {
    () => {
        WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            }
    };
}

#[macro_export]
macro_rules! keyboard_event {
  () => {
    WindowEvent::KeyboardInput { .. }
  }
}