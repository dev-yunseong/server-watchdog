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
                if !self.auth_use_case.password_required() ||
                    self.auth_use_case.authenticate(message.client_name.clone(), message.chat_id.clone()).await {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::error::Error;
    use crate::domain::server::health::Health;

    #[derive(Clone)]
    struct MockMessageGateway {
        messages: Arc<Mutex<Vec<(String, String, String)>>>,
    }

    impl MockMessageGateway {
        fn new() -> Self {
            Self {
                messages: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_messages(&self) -> Vec<(String, String, String)> {
            self.messages.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl MessageGateway for MockMessageGateway {
        async fn send_message(&self, client_name: &str, chat_id: &str, message: &str) {
            self.messages
                .lock()
                .unwrap()
                .push((client_name.to_string(), chat_id.to_string(), message.to_string()));
        }
    }

    struct MockServerManager;

    #[async_trait]
    impl ServerManager for MockServerManager {
        async fn kill(&self, _name: &str) -> bool {
            false
        }

        async fn healthcheck(&self, _name: &str) -> Health {
            Health::Unknown("Unknown".to_string())
        }

        async fn healthcheck_all(&self) -> Vec<(&str, Health)> {
            vec![]
        }

        async fn logs(&self, _name: &str, _n: i32) -> Option<String> {
            None
        }
    }

    #[derive(Clone)]
    struct MockAuthUseCase {
        password: Option<String>,
        registered_users: Arc<Mutex<Vec<(String, String)>>>,
    }

    impl MockAuthUseCase {
        fn new(password: Option<String>) -> Self {
            Self {
                password,
                registered_users: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_registered_users(&self) -> Vec<(String, String)> {
            self.registered_users.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl AuthUseCase for MockAuthUseCase {
        async fn set_password(&self, _password: Option<String>) {}

        async fn validate_password(&mut self, password: String) -> bool {
            match &self.password {
                Some(pwd) => pwd == &password,
                None => false,
            }
        }

        async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error>> {
            self.registered_users
                .lock()
                .unwrap()
                .push((client_name, identity));
            Ok(())
        }

        async fn authenticate(&mut self, client_name: String, identity: String) -> bool {
            self.registered_users
                .lock()
                .unwrap()
                .contains(&(client_name, identity))
        }

        fn password_required(&self) -> bool {
            self.password.is_some()
        }
    }

    #[tokio::test]
    async fn test_register_password_not_required() {
        let gateway = MockMessageGateway::new();
        let auth = MockAuthUseCase::new(None);
        let mut handler = GeneralHandler::new(
            Box::new(gateway.clone()),
            Box::new(MockServerManager),
            Box::new(auth),
        );

        let message = Message::new(
            "telegram".to_string(),
            "user123".to_string(),
            "/register testpass".to_string(),
        );

        handler.handle(message).await;

        let messages = gateway.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].2, "Password is not required");
    }

    #[tokio::test]
    async fn test_register_valid_password() {
        let gateway = MockMessageGateway::new();
        let auth = MockAuthUseCase::new(Some("correctpass".to_string()));
        let mut handler = GeneralHandler::new(
            Box::new(gateway.clone()),
            Box::new(MockServerManager),
            Box::new(auth.clone()),
        );

        let message = Message::new(
            "telegram".to_string(),
            "user123".to_string(),
            "/register correctpass".to_string(),
        );

        handler.handle(message).await;

        let messages = gateway.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].2, "Successfully registered.");

        let registered = auth.get_registered_users();
        assert_eq!(registered.len(), 1);
        assert_eq!(registered[0], ("telegram".to_string(), "user123".to_string()));
    }

    #[tokio::test]
    async fn test_register_invalid_password() {
        let gateway = MockMessageGateway::new();
        let auth = MockAuthUseCase::new(Some("correctpass".to_string()));
        let mut handler = GeneralHandler::new(
            Box::new(gateway.clone()),
            Box::new(MockServerManager),
            Box::new(auth),
        );

        let message = Message::new(
            "telegram".to_string(),
            "user123".to_string(),
            "/register wrongpass".to_string(),
        );

        handler.handle(message).await;

        let messages = gateway.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].2, "Invalid password. Usage: /register <password>");
    }

    #[tokio::test]
    async fn test_authenticate_registered_user() {
        let gateway = MockMessageGateway::new();
        let mut auth = MockAuthUseCase::new(Some("correctpass".to_string()));
        
        // Pre-register user
        auth.register("telegram".to_string(), "user123".to_string()).await.unwrap();

        let mut handler = GeneralHandler::new(
            Box::new(gateway.clone()),
            Box::new(MockServerManager),
            Box::new(auth),
        );

        let message = Message::new(
            "telegram".to_string(),
            "user123".to_string(),
            "/health".to_string(),
        );

        handler.handle(message).await;

        let messages = gateway.get_messages();
        assert_eq!(messages.len(), 1);
        // Should get a valid response, not a registration prompt
        assert!(!messages[0].2.contains("Registration required"));
    }

    #[tokio::test]
    async fn test_authenticate_unregistered_user() {
        let gateway = MockMessageGateway::new();
        let auth = MockAuthUseCase::new(Some("correctpass".to_string()));
        let mut handler = GeneralHandler::new(
            Box::new(gateway.clone()),
            Box::new(MockServerManager),
            Box::new(auth),
        );

        let message = Message::new(
            "telegram".to_string(),
            "user123".to_string(),
            "/health".to_string(),
        );

        handler.handle(message).await;

        let messages = gateway.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].2, "Registration required. Usage: /register <password>");
    }

    #[tokio::test]
    async fn test_no_password_allows_all_users() {
        let gateway = MockMessageGateway::new();
        let auth = MockAuthUseCase::new(None);
        let mut handler = GeneralHandler::new(
            Box::new(gateway.clone()),
            Box::new(MockServerManager),
            Box::new(auth),
        );

        let message = Message::new(
            "telegram".to_string(),
            "user123".to_string(),
            "/health".to_string(),
        );

        handler.handle(message).await;

        let messages = gateway.get_messages();
        assert_eq!(messages.len(), 1);
        // Should get a valid response without registration
        assert!(!messages[0].2.contains("Registration required"));
    }
}
