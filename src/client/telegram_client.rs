mod dto;

use anyhow::{anyhow, Result};
use crate::client::Client;
use rust_api_client::api::ApiClient;
use dto::SendMessageDto;
use log::{debug, error};
use crate::client::telegram_client::dto::{Message, TelegramResponse, Update};
use crate::core::Worker;

pub struct TelegramClient {
    api_client: ApiClient,
    offset: i64,
    callback: Option<Box<dyn Fn(&str)>>
}

impl TelegramClient {
    pub fn new(token: &str) -> Self {
        Self {
            api_client: ApiClient::new(format!("https://api.telegram.org/bot{token}")),
            callback: None,
            offset: 0
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

impl Client for TelegramClient {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool {
        let response = self.api_client
            .post_json::<SendMessageDto, Message> (
                "sendMessage",
                &SendMessageDto::new(chat_id, data), None, None).await;

        if response.is_err() {
            error!("[Err]: {}", response.err().unwrap().to_string());
            return false
        }

        true
    }

    fn set_callback(&mut self, callback: impl Fn(&str) + 'static) {
        self.callback = Some(Box::new(callback))
    }
}

impl Worker for TelegramClient {
    async fn on_tick(&mut self) {
        let updates = match self.get_update().await {
            Ok(updates) => {
                updates
            },
            Err(e) => {
                error!("[TelegramClient] Err: {e}");
                return;
            }
        };

        for update in updates {
            if update.message.is_none() {continue}
            let message = update.message.unwrap();
            if message.text.is_none() { continue }
            let text = message.text.unwrap();

            if let Some(cb) = self.callback.as_ref() {
                cb(text.as_str());
            }
        }
    }

    fn interval_ms() -> i32 {
        5000
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
        let mut telegram_client = TelegramClient::new(token.as_str());
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
        let telegram_client = TelegramClient::new(token.as_str());
         telegram_client.send_message(env::var("CHAT_ID").unwrap().as_str(), "test message").await;
    }
}
