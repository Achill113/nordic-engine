use nordic_engine::core::window::Window;

#[tokio::main]
async fn main() {
    let window = Window::new();

    window.run().await;
}
