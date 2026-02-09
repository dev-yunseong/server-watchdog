use async_trait::async_trait;
use log::error;
use tokio::sync::broadcast::Receiver;
use crate::client::ClientKind::Telegram;
use crate::client::telegram_client::dto::SendMessageDto;
use crate::client::telegram_client::TelegramClient;
use crate::core::worker::Worker;
use crate::core::config::ClientConfig;

mod telegram_client;

#[async_trait]
pub trait Client : Worker + Clone {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool {
        self.send_message_direct(SendMessageDto::new(chat_id, data, None)).await
    }
    async fn send_message_direct(&self, send_message_dto: SendMessageDto) -> bool;
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
    async fn send_message_direct(&self, send_message_dto: SendMessageDto) -> bool {
        match self {
            Telegram(c) => c.send_message_direct(send_message_dto).await
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