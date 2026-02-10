use async_trait::async_trait;
use derive_new::new;
use crate::application::client::MessageGateway;
use crate::application::handler::MessageHandler;
use crate::domain::client::Message;

#[derive(new)]
pub struct EchoHandler {
    message_gateway: Box<dyn MessageGateway>,
}

#[async_trait]
impl MessageHandler for EchoHandler {
    async fn handle(&self, message: Message) {
        self.message_gateway
            .send_message(
                message.client_name.as_str(),
                message.chat_id.as_str(),
                message.data.as_str()
            ).await;
    }
}