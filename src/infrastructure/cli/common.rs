use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use clap::{Parser, Subcommand};
use log::{debug, trace};
use tokio::sync::mpsc;
use crate::application::client::ClientLoader;
use crate::application::event::checker::{GeneralEventChecker, HealthEventChecker, LogEventChecker};
use crate::application::event::receiver::EventManager;
use crate::application::handler::{GeneralHandler, MessageHandler};
use crate::application::worker::WorkerRunner;
use crate::domain::chat::ChatList;
use crate::domain::config::{Config, EventSubscribeList};
use crate::domain::file_accessor::FileAccessor;
use crate::infrastructure::cli::client::ClientCommands;
use crate::infrastructure::cli::event::EventCommands;
use crate::infrastructure::cli::password::PasswordCommands;
use crate::infrastructure::cli::server::ServerCommands;
use crate::infrastructure::client::{ClientManager, MessageAdapter};
use crate::infrastructure::common::file_accessor::{get_chat_list_file_accessor, get_config_file_accessor, get_event_subscribe_file_accessor};
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
        let config_file_accessor: Arc<dyn FileAccessor<Config> + Send + Sync> = Arc::new(get_config_file_accessor());
        match self {
            Commands::Password { command } => {
                let chat_list_file_accessor: Arc<dyn FileAccessor<ChatList> + Send + Sync> = Arc::new(get_chat_list_file_accessor());
                let mut auth_adapter = AuthAdapter::new(config_file_accessor.clone(), chat_list_file_accessor);
                auth_adapter.init().await;
                let auth_config = Box::new(auth_adapter);
                command.run(auth_config).await
            }
            Commands::Server { command } => {
                debug!("server command");
                let server_config = ServerConfigAdapter::new(
                    config_file_accessor.clone()
                );
                let server_config = Box::new(server_config);
                command.run(server_config).await
            },
            Commands::Client { command } => {
                debug!("client command");
                let client_config = ClientConfigAdapter::new(
                    config_file_accessor.clone()
                );
                let client_config = Box::new(client_config);
                command.run(client_config).await
            },
            Commands::Event { command } => {
                debug!("event command");
                let subscribe_file_accessor: Arc<dyn FileAccessor<EventSubscribeList> + Send + Sync> = Arc::new(get_event_subscribe_file_accessor());

                let event_config = EventConfigAdapter::new(
                    config_file_accessor.clone(),
                    subscribe_file_accessor
                );
                let event_config = Box::new(event_config);
                command.run(event_config).await
            },
            Commands::Run => {
                debug!("run command");
                let chat_list_file_accessor: Arc<dyn FileAccessor<ChatList> + Send + Sync> = Arc::new(get_chat_list_file_accessor());
                let subscribe_file_accessor: Arc<dyn FileAccessor<EventSubscribeList> + Send + Sync> = Arc::new(get_event_subscribe_file_accessor());

                let worker_runner = Arc::new(Mutex::new(WorkerRunner::new()));

                let mut client_manager = ClientManager::new(
                    worker_runner.clone(),
                    Arc::new(Mutex::new(HashMap::new())),
                    config_file_accessor.clone()
                );
                let _ = client_manager.load_clients().await;

                let message_gateway = Arc::new(MessageAdapter::new(Arc::new(client_manager.clone())));
                let mut rx = client_manager.run().await;

                let mut auth_adapter = AuthAdapter::new(config_file_accessor.clone(), chat_list_file_accessor.clone());
                auth_adapter.init().await;

                let mut server_repository = ConfigServerRepository::new(
                    config_file_accessor.clone()
                );
                server_repository.load().await;

                let event_subscribe = EventConfigAdapter::new(
                    config_file_accessor.clone(),
                    subscribe_file_accessor.clone()
                );

                let server_manager = Arc::new(GeneralServerManager::new(Box::new(server_repository)));

                let mut handler = GeneralHandler::new(
                    message_gateway.clone(),
                    server_manager.clone(),
                    Box::new(auth_adapter),
                    Box::new(event_subscribe)
                );

                let (tx, rx_event) = mpsc::channel(32);
                let event_manager = EventManager::new(
                    rx_event,
                    message_gateway.clone(),
                    chat_list_file_accessor,
                    subscribe_file_accessor
                );

                {
                    worker_runner.lock().unwrap().run(Box::new(event_manager));
                }

                let event_checker = GeneralEventChecker::new(
                    config_file_accessor.clone(),
                    server_manager.clone(),
                    tx,
                    Box::new(HealthEventChecker::new()),
                    Box::new(LogEventChecker::new())
                );

                event_checker.init().await;

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
