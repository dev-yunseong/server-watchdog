mod server;
mod client;
mod common;

use clap::{Parser, Subcommand};
use crate::core::app;
use crate::core::app::App;
use crate::core::cli::client::ClientCommands;
use crate::core::cli::server::ServerCommands;

#[derive(Parser)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Server {
        #[command(subcommand)]
        command: ServerCommands
    },
    Client {
        #[command(subcommand)]
        command: ClientCommands
    },
    Run
}

impl Commands {
    pub async fn run(&self) {
        match self {
            Commands::Server { command } => command.run().await,
            Commands::Client { command } => command.run().await,
            Commands::Run => {
                App::init().await;
                App::global();
                println!("=== Run ===");
                tokio::signal::ctrl_c().await.unwrap();
            }
        }
    }
}
