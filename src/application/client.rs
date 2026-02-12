use std::error::Error;
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use crate::domain::client::Message;
use crate::infrastructure::client::Client;

#[async_trait]
pub trait MessageGateway : Send + Sync {
    async fn send_message(&self, client_name: &str, chat_id: &str, message: &str);
}


#[async_trait]
pub trait ClientLoader : Send + Sync {
    async fn load_clients(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn find(&self, name: &str) -> Option<&Box<dyn Client>>;
    async fn run(&mut self)-> Receiver<Message>;
}