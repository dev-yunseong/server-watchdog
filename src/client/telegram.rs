pub mod dto;

use std::sync::Arc;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crate::client::Client;
use rust_api_client::api::ApiClient;
use dto::SendMessageDto;
use log::{debug, error, warn};
use tokio::sync::broadcast::{self, Receiver, Sender};
use crate::client::telegram::dto::{Message, TelegramResponse, Update};
use crate::core::worker::Worker;

#[derive(Clone)]
pub struct TelegramClient {
    name: String,
    api_client: Arc<ApiClient>,
    offset: i64,
    tx: Option<Sender<(String, String)>>
}

impl TelegramClient {
    pub fn new(name: String, token: String) -> Self {
        Self {
            name,
            api_client: Arc::new(ApiClient::new(format!("https://api.telegram.org/bot{token}"))),
            offset: 0,
            tx: None
        }
    }

    async fn get_update(&mut self) -> Result<Vec<Update>> {
        let offset = self.offset.to_string();
        match self.api_client.get_json::<TelegramResponse<Vec<Update>>>("getUpdates", None, Some(&[("offset", offset.as_str())])).await {
            Ok(updates) => {

                debug!("[TelegramClient] Ok: Successfully get update");
                if !updates.ok {
                    return Err(anyhow!("[TelegramClient] status: {} {}", updates.error_code.unwrap(), updates.description.unwrap()));
                }

                if let Some(new_offset) = updates.result.iter().map(|update: &Update|{update.update_id}).max() {
                    self.offset = new_offset + 1;
                }

                Ok(updates.result)
            },
            Err(e) => {
                Err(anyhow!("[TelegramClient] Err: {}", e))
            }
        }
    }
}

#[async_trait]
impl Client for TelegramClient {
    async fn send_message_direct(&self, send_message_dto: SendMessageDto) -> bool {
        let response = self.api_client
            .post_json::<SendMessageDto, TelegramResponse<Message>> (
                "sendMessage",
                &send_message_dto, None, None).await;

        if response.is_err() {
            error!("[Err]: {}", response.err().unwrap().to_string());
            return false
        }

        true
    }

    fn subscribe(&mut self) -> Receiver<(String, String)> {
        match &self.tx {
            Some(tx) => tx.subscribe(),
            None => {
                let (tx, rx) = broadcast::channel::<(String, String)>(16);
                self.tx = Some(tx);
                rx
            }
        }
    }
}

#[async_trait]
impl Worker for TelegramClient {
    async fn on_tick(&mut self) -> bool {
        let updates = match self.get_update().await {
            Ok(updates) => {
                updates
            },
            Err(e) => {
                error!("[TelegramClient] Err: {e}");
                return false;
            }
        };

        for update in updates {
            let message = if let Some(msg) = update.message {
                (msg.chat.id.to_string(), msg.text.unwrap_or("".to_string()))
            } else if let Some(cb) = update.callback_query {
                let chat_id = match cb.message {
                    Some(message) => message.chat.id.to_string(),
                    None => continue
                };
                let text = cb.data.clone().unwrap_or_default();
                (chat_id, text)
            } else {
                continue;
            };

            if let Some(tx) = &self.tx {
                if let Err(e) = tx.send(message) {
                    warn!("[TelegramClient] Err: {}", e);
                }
            }
        }

        true
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn interval(&self) -> i32 {
        5
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_update() {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let mut telegram_client = TelegramClient::new("test_client".to_string(), token);
        let response = telegram_client.get_update().await;

        assert!(response.is_ok());
        println!("{:?}", response.unwrap());

        let response = telegram_client.get_update().await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn send_message() {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let telegram_client = TelegramClient::new("test_client".to_string(), token);
         telegram_client.send_message(env::var("CHAT_ID").unwrap().as_str(), "test message").await;
    }
}
