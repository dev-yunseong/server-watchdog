use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ChatList {
    pub chats: Vec<Chat>
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: String,
    pub client_name: String,
    pub identity: String
}

impl Chat {
    pub fn new(client_name: String, identity: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            client_name,
            identity
        }
    }
}

pub struct ChatMap {
    chats: HashMap<(String, String), Chat>
}

impl ChatMap {

    pub fn get_id(&self, client_name: &str, identity: &str) -> Option<&str> {
        let chat = self.chats.get(&(client_name.to_string(), identity.to_string()))?;
        Some(chat.id.as_str())
    }

    pub fn contains(&self, client_name: &str, identity: &str) -> bool {
        self.chats.contains_key(&(client_name.to_string(), identity.to_string()))
    }

    pub fn from(chat_list: ChatList) -> Self {
        let mut chats = HashMap::new();
        for chat in chat_list.chats.into_iter() {
            chats.insert((chat.client_name.clone(), chat.identity.clone()), chat);
        }

        Self {
            chats
        }
    }
}