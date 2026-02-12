use std::error::Error;
use std::path::PathBuf;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::fs;
use crate::application::config::AuthUseCase;
use crate::domain::chat::{Chat, ChatList, ChatMap};
use crate::infrastructure::config;
use crate::infrastructure::config::common::get_directory_path;

pub struct AuthAdapter {
    password: Option<String>,
    file_name: String,
    chat_map: Option<ChatMap>
}

impl AuthAdapter {
    pub fn new() -> Self {
        Self {
            password: None,
            file_name: String::from("chat_list.json"),
            chat_map: None,
        }
    }

    fn get_file_path(&self) -> Option<PathBuf> {
        let mut path = get_directory_path()?;
        path.push(self.file_name.as_str());
        Some(path)
    }

    async fn read(&self) -> Result<ChatList, Box<dyn Error>> {
        let file_path = match self.get_file_path() {
            Some(value) => value,
            None => return Err(anyhow!("Fail to find file path").into())
        };

        let raw_json = fs::read_to_string(file_path).await?;
        let chat_list = serde_json::from_str::<ChatList>(raw_json.as_str())?;
        Ok(chat_list)
    }

    async fn write(&self, chat_list: ChatList) {
        let raw_json = serde_json::to_string(&chat_list)
            .expect("Fail to Serialize chat list");
        fs::write(
            self.get_file_path()
                .expect("Fail to find file path"),
            raw_json).await
            .expect("Fail to write chat list");
    }
}

#[async_trait]
impl AuthUseCase for AuthAdapter {
    async fn set_password(&self, password: String) {
        let mut config = config::read().await;
        config.password = password;
        config::write(config).await;
    }

    async fn validate_password(&mut self, password: String) -> bool {
        match &self.password {
            Some(password) => password.eq(password.as_str()),
            None => {
                let config = config::read().await;
                self.password = Some(config.password);
                self.password.as_ref().unwrap().as_str().eq(password.as_str())
            }
        }
    }

    async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error>> {
        let mut chat_list = self.read()
            .await?;
        chat_list.chats
            .push(Chat::new(client_name, identity));
        self.write(chat_list).await;
        self.chat_map = None;
        Ok(())
    }

    async fn authenticate(&mut self, client_name: String, identity: String) -> bool {
        let chat_map = match &self.chat_map {
            Some(chat_map) => chat_map,
            None => {
                let list = match self.read().await {
                    Ok(list) => list,
                    Err(_) => return false
                };
                self.chat_map = Some(ChatMap::from(list));
                self.chat_map.as_ref().unwrap()
            }
        };

        chat_map.contains(client_name.as_str(), identity.as_str())
    }
}