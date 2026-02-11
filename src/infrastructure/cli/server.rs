use clap::Subcommand;
use log::{debug, trace};
use crate::application::config::ServerConfigUseCase;
use crate::domain::config::ServerConfig;
use crate::domain::server::Server;
use crate::infrastructure::cli::util::{read_int, read_string, read_string_option, FormatChecker};
use crate::infrastructure::config::ServerConfigAdapter;

#[derive(Subcommand)]
#[derive(Debug)]
pub enum ServerCommands {
    Add,
    List
}

impl ServerCommands {
    pub async fn run(&self, server_config_adapter: Box<dyn ServerConfigUseCase>) {
        trace!("server command start: {:?}", &self);
        match self {
            ServerCommands::Add => {
                debug!("add server");
                println!("--- Add Server ---");
                let name = read_string("name", FormatChecker::Name).await;
                let base_url = read_string_option("base url", FormatChecker::BaseUrl).await;
                let docker_container_name = read_string_option("docker container name", FormatChecker::NotAllowWhitespace).await;
                let health_check_path = read_string_option("health check path", FormatChecker::NotAllowWhitespace).await;
                let kill_path = read_string_option("kill path", FormatChecker::NotAllowWhitespace).await;
                let log_command = read_string_option("log command", FormatChecker::None).await;

                let config = ServerConfig::new(name, base_url, docker_container_name, health_check_path, kill_path, log_command);
                debug!("new server config: {:?}", &config);
                server_config_adapter.add_server(config).await;
            },
            ServerCommands::List => {
                debug!("list server");
                let server_config_adapter = ServerConfigAdapter {};
                let servers = server_config_adapter.list_server().await;
                debug!("servers: {:?}", &servers);

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
                            "=========\nName: {}\nBASE URL: {}\nDocker Container Name: {}\nKill URL: {}\nHealth Check URL: {}\nLog command: {}\n\n",
                            server.name,
                            server.base_url.as_deref().unwrap_or("None"),
                            server.docker_container_name.as_deref().unwrap_or("None"),
                            server.get_kill_url().as_deref().unwrap_or("None"),
                            server.get_health_check_url().as_deref().unwrap_or("None"),
                            command
                        );
                    }
                }
            }
        }
        trace!("server command end");
    }
}