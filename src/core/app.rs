use std::path::Path;

use super::{render::Render, window::Window, texture};
use image::{io::Reader as ImageReader, DynamicImage};
use log::debug;

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
        let path = Path::new("C:\\Users\\achil\\Projects\\nordic-engine\\assets\\happy-tree.png");
        debug!("{:?}", path);

        let image_result = ImageReader::open(path).unwrap().decode();

        let mut image: Option<DynamicImage> = None;
        if let Ok(img) = image_result {
            image = Some(img);
        }

        let render = Render::new(&self.window, image.as_ref()).await;

        self.window.run(render);
    }
}
