mod http_server_client;
mod std_log_reader;
pub mod util;

use std::collections::HashMap;
use async_trait::async_trait;
use crate::application::server::{ServerManager, ServerRepository};
use crate::domain::server::{health::Health, Server};
use crate::infrastructure::config;
use crate::infrastructure::server::http_server_client::HttpServerClient;
use crate::infrastructure::server::std_log_reader::StdLogReader;

pub struct ConfigServerRepository {
    servers: HashMap<String, Server>
}

impl ConfigServerRepository {
    
    pub fn new() -> Self {
        Self {
            servers: HashMap::new()
        }
    }
    
    pub async fn load(&mut self) {
        
        let config = config::read().await;
        let servers: Vec<Server> = config.servers
            .into_iter()
            .map(|config| { Server::from(config) })
            .collect();

        for server in servers {
            self.servers
                .insert(
                    server.name.to_string(), 
                    server);
        }
    }
}

impl ServerRepository for ConfigServerRepository {
    
    fn find(&self, name: &str) -> Option<&Server> {
        self.servers.get(name)
    }

    fn find_all(&self) -> Vec<&Server> {
        self.servers.values().collect()
    }
}

pub struct GeneralServerManager {
    server_repository: Box<dyn ServerRepository>,
    http_server_client: HttpServerClient,
    std_log_reader: StdLogReader
}

impl GeneralServerManager {
    pub fn new(server_repository: Box<dyn ServerRepository>) -> Self {
        Self {
            server_repository,
            http_server_client: HttpServerClient::new(),
            std_log_reader: StdLogReader::new()
        }
    }
}

#[async_trait]
impl ServerManager for GeneralServerManager {
    async fn kill(&self, name: &str) -> bool {
        let server = match self.server_repository.find(name) {
            Some(s) => s,
            None => return false
        };
        
        self.http_server_client.kill(server).await
    }

    async fn healthcheck(&self, name: &str) -> Health {
        let server = match self.server_repository.find(name) {
            Some(s) => s,
            None => return Health::Unknown
        };

        self.http_server_client.healthcheck(server).await
    }

    async fn logs(&self, name: &str, n: i32) -> Option<String> {
        let server = self.server_repository.find(name)?;
        self.std_log_reader.read(server, n).await
    }
}
