use crate::client::ClientKind;
use std::collections::HashMap;
use std::time::Duration;
use async_trait::async_trait;
use tokio::task::JoinHandle;
use crate::core::config;
use crate::core::config::ClientConfig;

#[async_trait]
pub trait Worker: Send {
    async fn on_tick(&mut self) -> bool;
    fn get_name(&self) -> &str;
    fn interval(&self) -> i32;
}

pub struct WorkerRegistry {
    handles: HashMap<String, JoinHandle<()>>
}

impl WorkerRegistry {

    pub fn new() -> Self {
        Self {
            handles: HashMap::new()
        }
    }

    fn stop(&mut self, key: &str) {
        let handle = match self.handles.get_mut(key) {
            Some(handle) => handle,
            None => return
        };
        handle.abort();
        self.handles.remove(key);
    }

    pub fn register_batch(&mut self, client_configs: Vec<ClientConfig>) {
        for client_config in client_configs.into_iter() {
            let client = ClientKind::from(client_config);
            let client = match client {
                Some(client) => client,
                None => continue,
            };
            self.register(Box::new(client));
        }
    }

    pub fn register(&mut self, mut worker: Box<dyn Worker>) {
        let key = worker.get_name().to_string();
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(worker.interval() as u64));

            loop {
                interval.tick().await;
                if !worker.on_tick().await {
                    break;
                }
            }
        });
        self.handles.insert(key, handle);
    }

    pub async fn load(&mut self) {
        let config = config::read().await;
        self.register_batch(config.clients);
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use log::error;
    use crate::client::{Client, ClientKind};
    use crate::core::*;
    use crate::core::config;
    use crate::core::config::{ClientConfig, Config};
    use crate::core::worker::WorkerRegistry;

    #[tokio::test]
    async fn load() {
        config::init().await;

        let mut config = Config::new();
        config.clients.push(ClientConfig::new_telegram("name", "token"));
        let last_client_num = config.clients.len();
        config::write(config).await;

        let mut registry = WorkerRegistry::new();
        registry.load().await;

        let client_num = registry.handles.len();

        config::remove().await;

        assert_eq!(last_client_num, client_num);
    }

    #[tokio::test]
    async fn run_work() {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let mut registry = WorkerRegistry::new();
        let mut client = ClientKind::from(
            ClientConfig::new_telegram("name", token.as_str())).unwrap();

        let mut rx = client.subscribe();
        let client_for_callback = client.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(message) => {
                        let chat_id_owned = message.0;
                        let text_owned = message.1;
                        client_for_callback.send_message(chat_id_owned.as_str(), format!("echo {chat_id_owned}: {text_owned}").as_str()).await;
                    },
                    Err(e) => {
                        error!("[Err]: {}", e);
                        break;
                    }
                }
            }
        });
        registry.register(Box::new(client));
        tokio::signal::ctrl_c().await.unwrap();
    }
}