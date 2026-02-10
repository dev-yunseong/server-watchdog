use log::{error, info};
use reqwest::Client;
use crate::domain::server::{Health, Server};

pub struct HttpServerClient {
    client: Client
}

impl HttpServerClient {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }
}

impl HttpServerClient {

    pub async fn kill(&self, server: &Server) -> bool {

        let kill_url = match server.get_kill_url() {
            Some(value) => value,
            None => return false
        };

        let client = self.client.clone();

        if let Err(e) = client.get(kill_url).send().await {
            error!("[HttpWatchdog] Err: Kill request failed {}", e);
            false
        } else {
            info!("[HttpWatchdog] Info: Kill signal sent successfully");
            true
        }
    }

    pub async fn healthcheck(&self, server: &Server) -> Health {

        let health_check_url = match server.get_health_check_url() {
            Some(value) => value,
            None => return Health::Unknown
        };

        let response = self.client
            .get(health_check_url)
            .send()
            .await;

        match response {
            Ok(_) => Health::Running,
            Err(_) => Health::Dead
        }
    }
}
