use clap::{Parser, Subcommand};
use log::{debug, trace};
use crate::application::client::ClientLoader;
use crate::application::handler::{GeneralHandler, MessageHandler};
use crate::infrastructure::cli::client::ClientCommands;
use crate::infrastructure::cli::event::EventCommands;
use crate::infrastructure::cli::password::PasswordCommands;
use crate::infrastructure::cli::server::ServerCommands;
use crate::infrastructure::client::{ClientManager, MessageAdapter};
use crate::infrastructure::config::{ClientConfigAdapter, EventConfigAdapter, ServerConfigAdapter};
use crate::infrastructure::config::auth::AuthAdapter;
use crate::infrastructure::server::{ConfigServerRepository, GeneralServerManager};

#[derive(Parser)]
#[derive(Debug)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
#[derive(Debug)]
pub enum Commands {
    Server {
        #[command(subcommand)]
        command: ServerCommands
    },
    Client {
        #[command(subcommand)]
        command: ClientCommands
    },
    Password {
        #[command(subcommand)]
        command: PasswordCommands
    },
    Event {
        #[command(subcommand)]
        command: EventCommands
    },
    Run
}

impl Commands {
    pub async fn run(&self) {
        trace!("command start: {:?}", &self);
        match self {
            Commands::Password { command } => {
                let mut auth_adapter = AuthAdapter::new();
                auth_adapter.init().await;
                let auth_config = Box::new(auth_adapter);
                command.run(auth_config).await
            }
            Commands::Server { command } => {
                debug!("server command");
                let server_config = ServerConfigAdapter::new();
                let server_config = Box::new(server_config);
                command.run(server_config).await
            },
            Commands::Client { command } => {
                debug!("client command");
                let client_config = ClientConfigAdapter::new();
                let client_config = Box::new(client_config);
                command.run(client_config).await
            },
            Commands::Event { command } => {
                debug!("event command");
                let event_config = EventConfigAdapter::new();
                let event_config = Box::new(event_config);
                command.run(event_config).await
            },
            Commands::Run => {
                debug!("run command");
                let mut client_loader = ClientManager::new();
                let _ = client_loader.load_clients().await;
                let mut rx = client_loader.run().await;

                let mut auth_adapter = AuthAdapter::new();
                auth_adapter.init().await;

                let mut server_repository = ConfigServerRepository::new();
                server_repository.load().await;

                let event_subscribe = EventConfigAdapter::new();
                
                let mut handler = GeneralHandler::new(
                    Box::new(MessageAdapter::new(Box::new(client_loader))),
                    Box::new(GeneralServerManager::new(Box::new(server_repository))),
                    Box::new(auth_adapter),
                    Box::new(event_subscribe)
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
                println!("=== Shutdown ===");
            }
        }
        trace!("command end");
    }
}
