pub mod dto;

use std::sync::Arc;
use std::time::Duration;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dto::SendMessageDto;
use log::{debug, error, trace, warn};
use tokio::sync::mpsc::{Sender};
use crate::application::worker::Worker;
use crate::domain;
use crate::infrastructure::client::common::Client;
use crate::infrastructure::client::telegram::dto::{GetUpdateDto, Message, TelegramResponse, Update};
use crate::infrastructure::common::api_client::ApiClient;

#[derive(Clone)]
pub struct TelegramClient {
    name: String,
    api_client: Arc<ApiClient>,
    offset: i64,
    tx: Option<Sender<domain::client::Message>>
}

impl TelegramClient {
    pub fn new(name: String, token: String) -> Self {
        trace!("TelegramClient::new(name: {}, token: ...)", &name);
        Self {
            name,
            api_client: Arc::new(ApiClient::new(format!("https://api.telegram.org/bot{token}"))),
            offset: 0,
            tx: None
        }
    }

    async fn get_update(&mut self) -> Result<Vec<Update>> {
        trace!("TelegramClient::get_update");
        let offset = self.offset;
        let dto = GetUpdateDto::new(offset);
        debug!("get_update request: {:?}", &dto);
        match self.api_client.post_json::<GetUpdateDto, TelegramResponse<Vec<Update>>>("getUpdates", &dto, None, None).await {
            Ok(updates) => {
                debug!("[TelegramClient] Ok: Successfully get update");
                if !updates.ok {
                    return Err(anyhow!("[TelegramClient] status: {} {}", updates.error_code.unwrap(), updates.description.unwrap()));
                }
                debug!("get_update response: {:?}", &updates.result);

                if let Some(new_offset) = updates.result.iter().map(|update: &Update|{update.update_id}).max() {
                    self.offset = new_offset + 1;
                    debug!("new offset: {}", &self.offset);
                }

                Ok(updates.result)
            },
            Err(e) => {
                Err(anyhow!("[TelegramClient] Err: {}", e))
            }
        }
    }

    async fn send_message_direct(&self, send_message_dto: SendMessageDto) -> bool {
        trace!("TelegramClient::send_message_direct");
        debug!("send_message_direct request: {:?}", &send_message_dto);
        let response = self.api_client
            .post_json::<SendMessageDto, TelegramResponse<Message>> (
                "sendMessage",
                &send_message_dto, None, None).await;

        if response.is_err() {
            error!("[Err]: {}", response.err().unwrap().to_string());
            return false
        }
        debug!("send_message_direct response: {:?}", &response);

        true
    }
}

#[async_trait]
impl Client for TelegramClient {

    async fn send_message(&self, chat_id: &str, data: &str) -> bool {
        trace!("Client::send_message(chat_id: {}, data: ...)", chat_id);
        self.send_message_direct(SendMessageDto::new(chat_id, data, None)).await
    }

    fn subscribe(&mut self, tx: Sender<domain::client::Message>) {
        trace!("Client::subscribe");
        self.tx = Some(tx);
    }
}

#[async_trait]
impl Worker for TelegramClient {
    async fn on_tick(&mut self) -> bool {
        trace!("Worker::on_tick for {}", &self.name);
        let updates = match self.get_update().await {
            Ok(updates) => {
                updates
            },
            Err(e) => {
                error!("[TelegramClient] Err: {e}");
                tokio::time::sleep(Duration::from_secs(5)).await;
                return true;
            }
        };
        debug!("{} updates received", updates.len());

        for update in updates {
            let (chat_id, data) = if let Some(msg) = update.message {
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
            let message = domain::client::Message::new(
                self.get_name().to_string(), chat_id, data
            );
            debug!("created message: {:?}", &message);

            if let Some(tx) = &self.tx {
                if let Err(e) = tx.send(message).await {
                    warn!("[TelegramClient] Err: {}", e);
                }
            }
        }
        true
    }

    fn get_name(&self) -> &str {
        trace!("Worker::get_name for {}", &self.name);
        self.name.as_str()
    }

    fn interval(&self) -> i32 {
        trace!("Worker::interval for {}", &self.name);
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
