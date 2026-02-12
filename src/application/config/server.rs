use std::error::Error;
use async_trait::async_trait;
use crate::domain::config::ServerConfig;

#[async_trait]
pub trait ServerConfigUseCase {
    async fn add_server(&self, server_config: ServerConfig) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn list_server(&self) -> Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>>;
}
