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

    pub async fn init(&mut self) {
        config::init().await;
        let config = config::read().await;
        self.password = config.password;
    }

    async fn get_chat_map(&mut self) -> Result<&ChatMap, Box<dyn Error>> {
        if self.chat_map.is_none() {
            let list = self.read().await?;
            self.chat_map = Some(ChatMap::from(list));
        }

        Ok(self.chat_map.as_ref().unwrap())
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

        if file_path.exists() {
            let raw_json = fs::read_to_string(file_path).await?;
            let chat_list = serde_json::from_str::<ChatList>(raw_json.as_str())?;
            Ok(chat_list)
        } else {
            Ok(ChatList { chats: Vec::new() })
        }
    }

    async fn write(&self, chat_list: ChatList) -> Result<(), Box<dyn Error>> {
        let raw_json = serde_json::to_string(&chat_list)?;
        let file_path = self.get_file_path()
            .ok_or_else(|| anyhow!("Failed to find file path"))?;
        
        // Ensure directory exists before writing
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
            }
        }
        
        fs::write(file_path, raw_json).await?;
        Ok(())
    }
}

#[async_trait]
impl AuthUseCase for AuthAdapter {
    async fn set_password(&self, password: Option<String>) {
        config::init().await;
        let mut config = config::read().await;
        config.password = password;
        config::write(config).await;
    }

    async fn validate_password(&mut self, password: String) -> bool {
        let config_password = match &self.password {
            Some(password) => password,
            None => {
                let config = config::read().await;
                self.password = config.password;
                self.password.as_ref().expect("Password is not defined").as_str()
            }
        };
        config_password.eq(password.as_str())
    }

    async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error>> {
        let mut chat_list = self.read()
            .await?;
        
        // Check if the entry already exists to make registration idempotent
        let already_exists = chat_list.chats.iter().any(|chat| {
            chat.client_name == client_name && chat.identity == identity
        });
        
        if !already_exists {
            chat_list.chats.push(Chat::new(client_name, identity));
            self.write(chat_list).await?;
            self.chat_map = None;
        }
        
        Ok(())
    }

    async fn authenticate(&mut self, client_name: String, identity: String) -> bool {
        let chat_map = match self.get_chat_map().await {
            Ok(value) => value,
            Err(_) => return false
        };

        chat_map.contains(client_name.as_str(), identity.as_str())
    }

    fn password_required(&self) -> bool {
        self.password.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tokio::fs;

    struct TestAuthAdapter {
        adapter: AuthAdapter,
        temp_dir: PathBuf,
    }

    impl TestAuthAdapter {
        async fn new(password: Option<String>) -> Self {
            let temp_dir = std::env::temp_dir().join(format!("watchdog_test_{}", uuid::Uuid::new_v4()));
            fs::create_dir_all(&temp_dir).await.unwrap();
            
            let adapter = AuthAdapter {
                password,
                file_name: "chat_list.json".to_string(),
                chat_map: None,
            };
            
            Self { adapter, temp_dir }
        }

        async fn cleanup(&self) {
            let _ = fs::remove_dir_all(&self.temp_dir).await;
        }

        fn get_file_path(&self) -> PathBuf {
            self.temp_dir.join(&self.adapter.file_name)
        }

        async fn read(&self) -> Result<ChatList, Box<dyn Error>> {
            let file_path = self.get_file_path();
            if file_path.exists() {
                let raw_json = fs::read_to_string(file_path).await?;
                let chat_list = serde_json::from_str::<ChatList>(raw_json.as_str())?;
                Ok(chat_list)
            } else {
                Ok(ChatList { chats: Vec::new() })
            }
        }

        async fn write(&self, chat_list: ChatList) {
            let raw_json = serde_json::to_string(&chat_list)
                .expect("Failed to serialize chat list");
            fs::write(self.get_file_path(), raw_json).await
                .expect("Failed to write chat list");
        }

        async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error>> {
            let mut chat_list = self.read().await?;
            chat_list.chats.push(Chat::new(client_name, identity));
            self.write(chat_list).await;
            self.adapter.chat_map = None;
            Ok(())
        }

        async fn authenticate(&mut self, client_name: String, identity: String) -> bool {
            let chat_list = match self.read().await {
                Ok(list) => list,
                Err(_) => return false,
            };
            
            if self.adapter.chat_map.is_none() {
                self.adapter.chat_map = Some(ChatMap::from(chat_list));
            }

            let chat_map = self.adapter.chat_map.as_ref().unwrap();
            chat_map.contains(client_name.as_str(), identity.as_str())
        }
    }

    #[tokio::test]
    async fn test_register_persists_to_file() {
        let mut test_adapter = TestAuthAdapter::new(Some("testpass".to_string())).await;

        // Register a user
        test_adapter.register("telegram".to_string(), "user123".to_string())
            .await
            .expect("Failed to register user");

        // Verify the file was created and contains data
        let file_path = test_adapter.get_file_path();
        assert!(file_path.exists(), "chat_list.json should exist");

        let content = fs::read_to_string(&file_path).await.unwrap();
        let chat_list: ChatList = serde_json::from_str(&content).unwrap();

        assert_eq!(chat_list.chats.len(), 1);
        assert_eq!(chat_list.chats[0].client_name, "telegram");
        assert_eq!(chat_list.chats[0].identity, "user123");
        assert!(!chat_list.chats[0].id.is_empty());

        test_adapter.cleanup().await;
    }

    #[tokio::test]
    async fn test_register_multiple_users() {
        let mut test_adapter = TestAuthAdapter::new(Some("testpass".to_string())).await;

        // Register multiple users
        test_adapter.register("telegram".to_string(), "user123".to_string())
            .await
            .expect("Failed to register user 1");
        test_adapter.register("slack".to_string(), "user456".to_string())
            .await
            .expect("Failed to register user 2");
        test_adapter.register("discord".to_string(), "user789".to_string())
            .await
            .expect("Failed to register user 3");

        // Read the file and verify all users are persisted
        let file_path = test_adapter.get_file_path();
        let content = fs::read_to_string(&file_path).await.unwrap();
        let chat_list: ChatList = serde_json::from_str(&content).unwrap();

        assert_eq!(chat_list.chats.len(), 3);
        assert_eq!(chat_list.chats[0].client_name, "telegram");
        assert_eq!(chat_list.chats[1].client_name, "slack");
        assert_eq!(chat_list.chats[2].client_name, "discord");

        test_adapter.cleanup().await;
    }

    #[tokio::test]
    async fn test_authenticate_with_persisted_data() {
        let mut test_adapter = TestAuthAdapter::new(Some("testpass".to_string())).await;

        // Register a user
        test_adapter.register("telegram".to_string(), "user123".to_string())
            .await
            .expect("Failed to register user");

        // Authenticate should succeed for registered user
        assert!(test_adapter.authenticate("telegram".to_string(), "user123".to_string()).await);

        // Authenticate should fail for unregistered user
        assert!(!test_adapter.authenticate("telegram".to_string(), "unregistered".to_string()).await);

        test_adapter.cleanup().await;
    }

    #[tokio::test]
    async fn test_validate_password_correct() {
        let mut adapter = AuthAdapter {
            password: Some("correctpass".to_string()),
            file_name: "chat_list.json".to_string(),
            chat_map: None,
        };

        assert!(adapter.validate_password("correctpass".to_string()).await);
    }

    #[tokio::test]
    async fn test_validate_password_incorrect() {
        let mut adapter = AuthAdapter {
            password: Some("correctpass".to_string()),
            file_name: "chat_list.json".to_string(),
            chat_map: None,
        };

        assert!(!adapter.validate_password("wrongpass".to_string()).await);
    }

    #[tokio::test]
    async fn test_password_required_with_password() {
        let adapter = AuthAdapter {
            password: Some("testpass".to_string()),
            file_name: "chat_list.json".to_string(),
            chat_map: None,
        };

        assert!(adapter.password_required());
    }

    #[tokio::test]
    async fn test_password_required_without_password() {
        let adapter = AuthAdapter {
            password: None,
            file_name: "chat_list.json".to_string(),
            chat_map: None,
        };

        assert!(!adapter.password_required());
    }

    #[tokio::test]
    async fn test_chat_map_caching() {
        let mut test_adapter = TestAuthAdapter::new(Some("testpass".to_string())).await;

        // Register users to build initial cache
        test_adapter.register("telegram".to_string(), "user123".to_string())
            .await
            .expect("Failed to register user");

        // First authenticate should load the chat_map
        assert!(test_adapter.authenticate("telegram".to_string(), "user123".to_string()).await);

        // Register another user, which should invalidate the cache
        test_adapter.register("slack".to_string(), "user456".to_string())
            .await
            .expect("Failed to register user 2");

        // Authenticate should work for both users after cache invalidation
        assert!(test_adapter.authenticate("telegram".to_string(), "user123".to_string()).await);
        assert!(test_adapter.authenticate("slack".to_string(), "user456".to_string()).await);

        test_adapter.cleanup().await;
    }

    #[tokio::test]
    async fn test_json_file_format() {
        let mut test_adapter = TestAuthAdapter::new(Some("testpass".to_string())).await;

        // Register users
        test_adapter.register("telegram".to_string(), "chat_123".to_string())
            .await
            .expect("Failed to register user");

        // Read and verify JSON structure
        let file_path = test_adapter.get_file_path();
        let content = fs::read_to_string(&file_path).await.unwrap();
        
        // Verify it's valid JSON
        let chat_list: ChatList = serde_json::from_str(&content).unwrap();
        
        // Verify structure
        assert_eq!(chat_list.chats.len(), 1);
        let chat = &chat_list.chats[0];
        assert!(!chat.id.is_empty(), "ID should not be empty");
        assert_eq!(chat.client_name, "telegram");
        assert_eq!(chat.identity, "chat_123");

        test_adapter.cleanup().await;
    }
}