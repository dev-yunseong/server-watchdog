use async_trait::async_trait;
use crate::domain::client::Message;

#[async_trait]
pub trait MessageHandler {
    async fn handle(&self, message: Message);
}