use super::{render::Render, window::Window};

pub struct App {
    window: Window,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let window = Window::new();

        Self { window }
    }

    pub async fn run(self) {
        let render = Render::new(&self.window).await;

        self.window.run(render);
    }
}
