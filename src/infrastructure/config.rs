mod common;
pub mod auth;

use async_trait::async_trait;

use crate::application::config::{ClientConfigUseCase, ServerConfigUseCase};
use crate::domain::config::{ClientConfig, ServerConfig};
pub use crate::infrastructure::config::common::{init, read, write};

pub struct ClientConfigAdapter;

#[async_trait]
impl ClientConfigUseCase for ClientConfigAdapter {
    async fn add_client(&self, client_config: ClientConfig) {
        init().await;
        let mut config = read().await;
        config.clients.push(client_config);
        write(config).await;
    }

    async fn list_client(&self) -> Vec<ClientConfig> {
        let config = read().await;
        config.clients
    }
}

pub struct ServerConfigAdapter {

}

#[async_trait]
impl ServerConfigUseCase for ServerConfigAdapter {

    async fn add_server(&self, server_config: ServerConfig) {
        init().await;
        let mut config = read().await;
        config.servers.push(server_config);
        write(config).await;
    }

    async fn list_server(&self) -> Vec<ServerConfig> {
        let config = read().await;
        config.servers
    }
}