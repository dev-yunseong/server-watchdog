use std::error::Error;
use async_trait::async_trait;
use crate::application::config::ClientConfigUseCase;
use crate::domain::config::{ClientConfig, Config};
use crate::infrastructure::common::file_accessor::{get_config_file_accessor, FileAccessor};

pub struct ClientConfigAdapter {
    config_file_accessor: FileAccessor<Config>
}

impl ClientConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_file_accessor: get_config_file_accessor()
        }
    }
}

#[async_trait]
impl ClientConfigUseCase for ClientConfigAdapter {
    async fn add_client(&self, client_config: ClientConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.clients.push(client_config);
        self.config_file_accessor.write(config).await?;
        Ok(())
    }

    async fn list_client(&self) -> Result<Vec<ClientConfig>, Box<dyn Error + Send + Sync>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.clients)
    }
}
