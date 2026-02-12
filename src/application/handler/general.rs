use async_trait::async_trait;
use derive_new::new;
use log::{debug, trace};
use crate::application::client::MessageGateway;
use crate::application::config::AuthUseCase;
use crate::application::handler::command::Command;
use crate::application::handler::MessageHandler;
use crate::application::server::ServerManager;
use crate::domain::client::Message;

const INVALID_COMMAND_MESSAGE: &str = r#"Invalid or unknown command.

Available commands:
- /logs <server_name> <lines>
  Fetches the last <lines> of logs from the specified server.
  Example: /logs main 100

- /health (server_name)
  (server_name): optional. If provided, returns the health status of the specified server."#;

#[derive(new)]
pub struct GeneralHandler {
    message_gateway: Box<dyn MessageGateway>,
    server_manager: Box<dyn ServerManager>,
    auth_use_case: Box<dyn AuthUseCase>
}

#[async_trait]
impl MessageHandler for GeneralHandler {
    async fn handle(&mut self, message: Message) {
        match message.data.split_whitespace().collect::<Vec<_>>()[..] {
            ["/register", password] => {
                let response = if self.auth_use_case.validate_password(password.to_string()).await {
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
                if (self.auth_use_case.authenticate(message.client_name.clone(), message.chat_id.clone()).await) {
                    self._handle(message).await
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

    async fn _handle(&mut self, message: Message) {
        trace!("GeneralHandler::handle");
        debug!("handling message: {:?}", &message);

        let command = Command::parse(message.data);
        debug!("parsed command: {:?}", &command);

        let response = match command {
            Command::Logs(name, n) => {
                self.server_manager.logs(name.as_str(), n).await
                    .unwrap_or(String::from("Logs are not available."))
            },
            Command::HealthCheck(name) => {
                let health = self.server_manager.healthcheck(name.as_str()).await;
                format!("===\nServer: {name}\n Health: {health}")
            },
            Command::HealthCheckAll => {
                self.server_manager.healthcheck_all()
                    .await
                    .iter().map(|result|{format!("===\nServer: {}\nHealth: {}", result.0, result.1)})
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            Command::Nothing => String::from(INVALID_COMMAND_MESSAGE)
        };
        debug!("response: {}", &response);

        self.message_gateway
            .send_message(
                message.client_name.as_str(),
                message.chat_id.as_str(),
                response.as_str()
            )
            .await;
    }
}
