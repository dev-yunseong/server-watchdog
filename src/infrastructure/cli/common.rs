use clap::{Parser, Subcommand};
use crate::application::client::ClientLoader;
use crate::application::handler::MessageHandler;
use crate::infrastructure::cli::client::ClientCommands;
use crate::infrastructure::cli::server::ServerCommands;
use crate::infrastructure::client::{ClientManager, MessageAdapter};
use crate::infrastructure::config::{ClientConfigAdapter, ServerConfigAdapter};
use crate::infrastructure::handler::{EchoHandler};

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
            Commands::Server { command } => {
                let server_config = ServerConfigAdapter {};
                let server_config = Box::new(server_config);
                command.run(server_config).await
            },
            Commands::Client { command } => {
                let client_config = ClientConfigAdapter {};
                let client_config = Box::new(client_config);
                command.run(client_config).await
            },
            Commands::Run => {
                let mut client_loader = ClientManager::new();
                client_loader.load_clients().await;
                let mut rx = client_loader.run().await;
                let handler = EchoHandler::new(
                    Box::new(MessageAdapter::new(Box::new(client_loader)))
                );
                tokio::spawn(async move {
                    loop {
                        if let Some(message) = rx.recv().await {
                            handler.handle(message).await;
                        }
                    }
                });
                println!("=== Run ===");
                tokio::signal::ctrl_c().await.unwrap();
            }
        }
    }
}
