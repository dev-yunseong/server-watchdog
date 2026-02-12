use std::error::Error;
use async_trait::async_trait;
use crate::domain::config::{ClientConfig, ServerConfig};

#[async_trait]
pub trait AuthUseCase : Send + Sync {
    async fn set_password(&self, password: String);
    async fn validate_password(&mut self, password: String) -> bool;
    async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error>>;
    async fn authenticate(&mut self, client_name: String, identity: String) -> bool;
}

#[async_trait]
pub trait ServerConfigUseCase {
    async fn add_server(&self, server_config: ServerConfig);
    async fn list_server(&self) -> Vec<ServerConfig>;
}

#[async_trait]
pub trait ClientConfigUseCase {
    async fn add_client(&self, client_config: ClientConfig);
    async fn list_client(&self) -> Vec<ClientConfig>;
}