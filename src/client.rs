use async_trait::async_trait;
use crate::client::ClientKind::Telegram;
use crate::client::telegram_client::TelegramClient;
use crate::core::Worker;
use crate::core::registrar::config::ClientConfig;

mod telegram_client;

#[async_trait]
pub  trait Client : Worker {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool;
    fn set_callback(&mut self, callback: impl Fn(&str, &str) + 'static + Send + Sync);
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

    fn set_callback(&mut self, callback: impl Fn(&str, &str) + 'static + Send + Sync) {
        match self {
            Telegram(c) => c.set_callback(callback)
        }
    }
}

#[async_trait]
impl Worker for ClientKind {

    async fn on_tick(&mut self) {
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

pub  enum ClientKind {
    Telegram(TelegramClient)
}