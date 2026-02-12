pub mod command;
mod general;

use async_trait::async_trait;

pub use general::*;
use crate::domain::client::Message;

#[async_trait]
pub trait MessageHandler : Send + Sync {
    async fn handle(&mut self, message: Message);
}