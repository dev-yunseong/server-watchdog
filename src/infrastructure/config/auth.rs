use std::error::Error;
use async_trait::async_trait;
use crate::application::config::AuthUseCase;
use crate::domain::chat::{Chat, ChatList, ChatMap};
use crate::domain::config::Config;
use crate::infrastructure::common::file_accessor::{get_chat_list_file_accessor, get_config_file_accessor, FileAccessor};

pub struct AuthAdapter {
    password: Option<String>,
    chat_map: Option<ChatMap>,
    config_file_accessor: FileAccessor<Config>,
    chat_list_file_accessor: FileAccessor<ChatList>
}

impl AuthAdapter {
    pub fn new() -> Self {
        Self {
            password: None,
            chat_map: None,
            config_file_accessor: get_config_file_accessor(),
            chat_list_file_accessor: get_chat_list_file_accessor()
        }
    }

    pub async fn init(&mut self) {
        let config = self.config_file_accessor.read().await
            .unwrap();
        self.password = config.password;
    }

    async fn get_chat_map(&mut self) -> Result<&ChatMap, Box<dyn Error + Send + Sync>> {
        if self.chat_map.is_none() {
            let list = self.chat_list_file_accessor.read().await?;
            self.chat_map = Some(ChatMap::from(list));
        }

        Ok(self.chat_map.as_ref().unwrap())
    }
}

#[async_trait]
impl AuthUseCase for AuthAdapter {
    async fn set_password(&self, password: Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.password = password;
        self.config_file_accessor.write(config).await?;
        Ok(())
    }

    async fn validate_password(&mut self, password: String) -> bool {
        let config_password = self.password.as_ref().expect("Password is not defined").as_str();
        config_password.eq(password.as_str())
    }

    async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut chat_list = self.chat_list_file_accessor.read()
            .await?;
        
        // Check if the entry already exists to make registration idempotent
        let already_exists = chat_list.chats.iter().any(|chat| {
            chat.client_name == client_name && chat.identity == identity
        });
        
        if !already_exists {
            chat_list.chats.push(Chat::new(client_name, identity));
            self.chat_list_file_accessor.write(chat_list).await?;
            self.chat_map = None;
        }
        
        Ok(())
    }

    async fn authenticate(&mut self, client_name: String, identity: String) -> Option<String> {
        let chat_map = match self.get_chat_map().await {
            Ok(value) => value,
            Err(_) => return None
        };

        let id = chat_map.get_id(client_name.as_str(), identity.as_str())?;
        Some(String::from(id))
    }

    fn password_required(&self) -> bool {
        self.password.is_some()
    }
}