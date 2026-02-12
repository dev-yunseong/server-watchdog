use std::error::Error;
use async_trait::async_trait;
use crate::domain::config::EventConfig;

#[async_trait]
pub trait EventConfigUseCase {
    async fn add_event(&self, event_config: EventConfig) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn list_event(&self) -> Result<Vec<EventConfig>, Box<dyn Error + Send + Sync>>;
    async fn remove_event(&self, name: String) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait EventSubscribeUseCase: Send + Sync {
    async fn subscribe(&self, chat_id: String, event_name: String) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn list_subscribed_event(&self, chat_id: String) -> Result<Vec<EventConfig>, Box<dyn Error + Send + Sync>>;
    async fn unsubscribe(&self, chat_id: String, event_name: String) -> Result<(), Box<dyn Error + Send + Sync>>;
}