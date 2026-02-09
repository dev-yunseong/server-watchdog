use log::{debug, info};
use tokio::sync::broadcast::Receiver;
use crate::client::{Client, ClientKind};
use crate::core::worker::Worker;

pub async fn attach_handle(mut client: ClientKind, mut rx: Receiver<(String, String)>) {
    tokio::spawn(async move {
        info!("[CommandHandler] client({}) is attached", client.get_name());
        loop {
             if let Ok(message) = rx.recv().await {
                 handle(message, &mut client).await;
            }
        }
    });
}

async fn handle(message: (String, String), client: &mut ClientKind) {
    debug!("[CommandHandler] handle chat_id: {}, message: {}", message.0, message.1);
    client.send_message(message.0.as_str(), message.1.as_str()).await;
}