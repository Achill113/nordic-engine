use super::window::Window;

pub struct App {
  window: Option<Window>,
}

impl App {
  pub fn new() -> Self {
    let window = Window::new();

    Self {
      window: Some(window)
    }
  }

  pub fn window(mut self) -> Window {
    self.window.take().unwrap()
  }

  pub async fn run(self) -> () {
    self.window().run();
  }
}