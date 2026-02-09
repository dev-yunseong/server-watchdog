mod telegram;

use async_trait::async_trait;
use tokio::sync::broadcast::Receiver;
use crate::client::ClientKind::Telegram;
use crate::client::telegram::dto::SendMessageDto;
use crate::client::telegram::TelegramClient;
use crate::core::worker::Worker;
use crate::core::config::ClientConfig;

#[async_trait]
pub trait Client : Worker + Clone {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool;
    fn subscribe(&mut self) -> Receiver<(String, String)>;
}

impl ClientKind {
    pub fn from(config: ClientConfig) -> Option<Self> {
        match config.kind.as_str() {
            "telegram" => {
                let token = config.token?;
                Some(ClientKind::Telegram(TelegramClient::new(config.name, token)))
            }
            _ => None,
        }
    }
}

#[async_trait]
impl Client for ClientKind {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool {
        match self {
            Telegram(c) => c.send_message(chat_id, data).await
        }
    }


    fn subscribe(&mut self) -> Receiver<(String, String)> {
        match self {
            Telegram(c) => c.subscribe()
        }
    }
}

#[async_trait]
impl Worker for ClientKind {

    async fn on_tick(&mut self) -> bool {
        match self {
            Telegram(c) => c.on_tick().await
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Telegram(c) => c.get_name()
        }
    }

    fn interval(&self) -> i32 {
        match self {
            Telegram(c) => c.interval()
        }
    }
}

#[derive(Clone)]
pub enum ClientKind {
    Telegram(TelegramClient)
}