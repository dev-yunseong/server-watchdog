use server_manager::core::app::App;

#[tokio::main]
async fn main() {
    App::init().await;
    let app = App::global();
    tokio::signal::ctrl_c().await.unwrap();
}
