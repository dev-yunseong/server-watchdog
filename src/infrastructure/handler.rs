mod command;

use async_trait::async_trait;
use derive_new::new;
use log::{debug, trace};
use crate::application::client::MessageGateway;
use crate::application::handler::MessageHandler;
use crate::application::server::ServerManager;
use crate::domain::client::Message;
use crate::infrastructure::handler::command::Command;

const INVALID_COMMAND_MESSAGE: &str = r#"Invalid or unknown command.

Available commands:
- /logs <server_name> <lines>
  Fetches the last <lines> of logs from the specified server.
  Example: /logs main 100"#;

#[derive(new)]
pub struct EchoHandler {
    message_gateway: Box<dyn MessageGateway>,
}

#[async_trait]
impl MessageHandler for EchoHandler {
    async fn handle(&self, message: Message) {
        trace!("EchoHandler::handle");
        debug!("handling message: {:?}", &message);
        self.message_gateway
            .send_message(
                message.client_name.as_str(),
                message.chat_id.as_str(),
                message.data.as_str()
            ).await;
    }
}

#[derive(new)]
pub struct GeneralHandler {
    message_gateway: Box<dyn MessageGateway>,
    server_manager: Box<dyn ServerManager>,
}

#[async_trait]
impl MessageHandler for GeneralHandler {
    async fn handle(&self, message: Message) {
        trace!("GeneralHandler::handle");
        debug!("handling message: {:?}", &message);

        let command = Command::parse(message.data);
        debug!("parsed command: {:?}", &command);

        let response = match command {
            Command::Logs(name, n) => {
                self.server_manager.logs(name.as_str(), n).await
                    .unwrap_or(String::from("Logs are not available."))
            },
            Command::Nothing => String::from(INVALID_COMMAND_MESSAGE)
        };
        debug!("response: {}", &response);

        self.message_gateway
            .send_message(
                message.client_name.as_str(),
                message.chat_id.as_str(),
                response.as_str()
            ).await;
    }
}