use clap::Parser;
use server_manager::core::app::App;
use server_manager::core::cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    App::init().await;

    cli.command.run().await;
}
