mod child_process_stream;
mod http_server_client;
mod std_log_reader;
mod docker;
pub mod util;

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use tokio_stream::Stream;
use crate::application::server::{ServerManager, ServerRepository};
use crate::domain::config::Config;
use crate::domain::file_accessor::FileAccessor;
use crate::domain::server::{health::Health, Server};
use crate::domain::server::health::HealthCheckMethod;
use crate::infrastructure::server::docker::DockerHealthChecker;
use crate::infrastructure::server::http_server_client::HttpServerClient;
use crate::infrastructure::server::std_log_reader::StdLogReader;

#[derive(new)]
pub struct ConfigServerRepository {
    #[new(default)]
    servers: HashMap<String, Server>,
    config_file_accessor: Arc<dyn FileAccessor<Config> + Send + Sync>
}

impl ConfigServerRepository {

    pub async fn load(&mut self) {
        let config = self.config_file_accessor.read().await.unwrap();
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
    std_log_reader: StdLogReader,
    docker_health_checker: DockerHealthChecker,
}

impl GeneralServerManager {
    pub fn new(server_repository: Box<dyn ServerRepository>) -> Self {
        Self {
            server_repository,
            http_server_client: HttpServerClient::new(),
            std_log_reader: StdLogReader::new(),
            docker_health_checker: DockerHealthChecker::new(),
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
            None => return Health::Unknown(format!("Fail to found server: '{}'", name))
        };

        match server.health_check_method {
            HealthCheckMethod::Http(_) => {
                self.http_server_client.healthcheck(server).await
            },
            HealthCheckMethod::Docker => {
                self.docker_health_checker.healthcheck(server).await
            },
            HealthCheckMethod::None => Health::Unknown(String::from("Health check is not available"))
        }
    }

    async fn healthcheck_all(&self) -> Vec<(&str, Health)> {
        let mut result = Vec::new();

        for server  in self.server_repository.find_all() {
            let health = self.healthcheck(server.name.as_str()).await;
            result.push((server.name.as_str(), health));
        }
        result
    }

    async fn logs(&self, name: &str, n: i32) -> Option<String> {
        let server = self.server_repository.find(name)?;
        self.std_log_reader.read(server, n).await
    }

    async fn logs_stream(&self, name: &str) -> Option<Box<dyn Stream<Item=String> + Send>> {
        let server = self.server_repository.find(name)?;
        self.std_log_reader.read_follow(server).await
    }
}