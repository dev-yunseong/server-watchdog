use clap::{command, Subcommand};
use crate::client::ClientKind;
use crate::core::app::App;
use crate::core::cli::common::{read_string, read_string_option};
use crate::core::config::{add_client, read, ClientConfig};
use crate::core::worker::Worker;
use crate::watchdog::server::Server;

#[derive(Subcommand)]
pub enum ClientCommands {
    Add,
    List
}

impl ClientCommands {
    pub async fn run(&self) {
        match self {
            ClientCommands::Add => {
                let name = read_string("Name").await;
                let kind = read_string("kind (ex: telegram)").await;
                let token = read_string_option("Token").await;

                add_client(ClientConfig::new_telegram(name.as_str(), token.unwrap().as_str())).await;
            },
            ClientCommands::List => {
                let config = read().await;
                println!("--- Client List ---");
                if config.clients.is_empty() {
                    println!("Empty Client");
                } else {
                    for client in config.clients {
                        println!(
                            "=========\nName: {}\nKind: {}\nToken: {}\n\n",
                            client.name,
                            client.kind,
                            client.token.unwrap_or("None".to_string())
                        );
                    }
                }
            }
        }
    }
}