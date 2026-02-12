use std::error::Error;
use async_trait::async_trait;
use crate::domain::config::ClientConfig;

#[async_trait]
pub trait ClientConfigUseCase {
    async fn add_client(&self, client_config: ClientConfig) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn list_client(&self) -> Result<Vec<ClientConfig>, Box<dyn Error + Send + Sync>>;
}