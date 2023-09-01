use dotenv::dotenv;
use log::info;
use nordic_engine::core::app::App;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    env_logger::init();

    info!("Starting application");

    let app = App::new();

    app.run().await;
}
