use async_trait::async_trait;
use log::{error, info};
use reqwest::Client;
use crate::watchdog::server::Server;
use crate::watchdog::WatchdogKind::{Http};

pub mod server;

#[async_trait]
trait Watchdog {
    async fn health_check(&self) -> Health;
    fn kill(&self);
}

pub enum Health {
    Running, Dead, Drowning, Unknown
}

enum WatchdogKind {
    Http(HttpWatchdog)
}

#[async_trait]
impl Watchdog for WatchdogKind {
    async fn health_check(&self) -> Health {
        match self {
            Http(checker) => checker.health_check().await
        }
    }

    fn kill(&self) {

    }
}

pub struct HttpWatchdog {
    server: Server,
    client: Client
}

impl HttpWatchdog {
    fn new(server: Server) -> Self{
        Self {
            server,
            client: Client::new()
        }
    }
}

#[async_trait]
impl Watchdog for HttpWatchdog {
    async fn health_check(&self) -> Health {
        let health_check_url = match self.server.get_health_check_url() {
            Some(value) => value,
            None => return Health::Unknown
        };
        
        let response = self.client
            .get(health_check_url)
            .send()
            .await;

        match response {
            Ok(response) => Health::Running,
            Err(e) => Health::Dead
        }
    }

    fn kill(&self) {
        let kill_url = match self.server.get_kill_url() {
            Some(value) => value,
            None => return
        };

        let client = self.client.clone();
        tokio::spawn(
            async move {
                if let Err(e) = client.get(kill_url).send().await {
                    error!("[HttpWatchdog] Err: Kill request failed {}", e);
                } else {
                    info!("[HttpWatchdog] Info: Kill signal sent successfully");
                }
            }
        );
    }
}
