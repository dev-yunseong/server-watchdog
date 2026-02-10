use clap::Subcommand;
use crate::application::config::ServerConfigUseCase;
use crate::domain::config::ServerConfig;
use crate::domain::server::Server;
use crate::infrastructure::cli::util::{read_int, read_string, read_string_option, read_string_option_allow_whitespace};
use crate::infrastructure::config::ServerConfigAdapter;

#[derive(Subcommand)]
pub enum ServerCommands {
    Add,
    List
}

impl ServerCommands {
    pub async fn run(&self, server_config_adapter: Box<dyn ServerConfigUseCase>) {
        match self {
            ServerCommands::Add => {
                println!("--- Add Server ---");
                let name = read_string("name").await;
                let proto = read_string("protocol").await;
                let host = read_string("host").await;
                let port = read_int("port").await;
                let health_check_path = read_string_option("health check path").await;
                let kill_path = read_string_option("kill path").await;
                let log_command = read_string_option_allow_whitespace("log command").await;
                
                let config = ServerConfig::new(name, proto, host, port as i16, health_check_path, kill_path, log_command);
                server_config_adapter.add_server(config).await;
            },
            ServerCommands::List => {
                let server_config_adapter = ServerConfigAdapter {};
                let servers = server_config_adapter.list_server().await;

                println!("--- Server List ---");

                if servers.is_empty() {
                    println!("Empty Server");
                } else {
                    for server in servers {
                        let server = Server::from(server);

                        let command = match server.log_command.as_ref() {
                            Some(command) => command.join(" "),
                            None => "None".to_string()
                        };

                        println!(
                            "=========\nName: {}\nBASE URL: {}\nKill URL: {}\nHealth Check URL: {}\nLog command: {}\n\n",
                            server.name,
                            server.get_url(),
                            server.get_kill_url().unwrap_or("None".to_string()),
                            server.get_health_check_url().unwrap_or("None".to_string()),
                            command
                        );
                    }
                }
            }
        }
    }
}