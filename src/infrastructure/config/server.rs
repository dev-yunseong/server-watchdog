use std::error::Error;
use async_trait::async_trait;
use crate::application::config::ServerConfigUseCase;
use crate::domain::config::{Config, ServerConfig};
use crate::infrastructure::common::file_accessor::{get_config_file_accessor, FileAccessor};

pub struct ServerConfigAdapter {
    config_file_accessor: FileAccessor<Config>
}

impl ServerConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_file_accessor: get_config_file_accessor()
        }
    }
}

#[async_trait]
impl ServerConfigUseCase for ServerConfigAdapter {

    async fn add_server(&self, server_config: ServerConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.servers.push(server_config);
        self.config_file_accessor.write(config).await?;
        Ok(())
    }

    async fn list_server(&self) -> Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.servers)
    }
}