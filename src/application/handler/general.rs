use async_trait::async_trait;
use derive_new::new;
use log::{debug, trace};
use crate::application::client::MessageGateway;
use crate::application::config::{AuthUseCase, EventConfigUseCase, EventSubscribeUseCase};
use crate::application::handler::command::{Command, Run};
use crate::application::handler::MessageHandler;
use crate::application::server::ServerManager;
use crate::domain::client::Message;

pub const INVALID_COMMAND_MESSAGE: &str = r#"Invalid or unknown command.

Available commands:
- /logs <server_name> <lines>
  Fetches the last <lines> of logs from the specified server.
  Example: /logs main 100

- /health (server_name)
  (server_name): optional. If provided, returns the health status of the specified server."#;

use std::sync::Arc;

#[derive(new)]
pub struct GeneralHandler {
    pub message_gateway: Arc<dyn MessageGateway>,
    pub server_manager: Arc<dyn ServerManager>,
    pub auth_use_case: Box<dyn AuthUseCase>,
    pub event_subscribe_use_case: Arc<dyn EventSubscribeUseCase>,
    pub event_config_use_case: Arc<dyn EventConfigUseCase>,
}

#[async_trait]
impl MessageHandler for GeneralHandler {
    async fn handle(&mut self, message: Message) {
        match message.data.split_whitespace().collect::<Vec<_>>()[..] {
            ["/register", password] => {
                let response = if !self.auth_use_case.password_required() {
                        String::from("Password is not required")
                } else if self.auth_use_case.validate_password(password.to_string()).await {
                    match self.auth_use_case.register(message.client_name.clone(), message.chat_id.clone()).await {
                        Ok(_) => String::from("Successfully registered."),
                        Err(e) => format!("Fail to register: {e}")
                    }

                } else {
                    String::from("Invalid password. Usage: /register <password>")
                };
                self.message_gateway.send_message(
                    message.client_name.as_str(),
                    message.chat_id.as_str(),
                    response.as_str()
                )
                    .await
            },
            _ => {
                let auth_id = self.auth_use_case
                    .authenticate(message.client_name.clone(), message.chat_id.clone())
                    .await;
                if let Some(id) = auth_id {
                    self._handle(String::from(id), message).await
                } else {
                    self.message_gateway.send_message(
                        message.client_name.as_str(),
                        message.chat_id.as_str(),
                        "Registration required. Usage: /register <password>"
                    )
                        .await
                }
            }
        }
    }
}
impl GeneralHandler {

    async fn _handle(&mut self, id: String, message: Message) {
        trace!("GeneralHandler::handle");
        debug!("handling message: {:?}", &message);

        let command = Command::parse(message.data.as_str());
        debug!("parsed command: {:?}", &command);

        let response = command.run(self, id, &message).await;
        debug!("response: {:?}", &response);

        let response = response.unwrap_or_else(|e| format!("[Err] {e}"));

        self.message_gateway
            .send_message(
                message.client_name.as_str(),
                message.chat_id.as_str(),
                response.as_str()
            )
            .await;
    }
}