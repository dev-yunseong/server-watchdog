use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SendMessageDto {
    chat_id: String,
    text: String
}

#[derive(Deserialize)]
pub struct TelegramResponse<T> {
    pub  ok: bool,
    pub  result: T,
    pub  error_code: Option<i16>,
    pub  description: Option<String>,
}

impl SendMessageDto {
    pub fn new(chat_id: &str, text: &str) -> Self {
        Self {
            chat_id: chat_id.to_string(),
            text: text.to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub  message_id: i64,
    pub  message_thread_id: Option<i64>,
    pub  from: User,
    pub  date: i64,
    pub  chat: Chat,
    pub  text: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub  update_id: i64,
    pub  message: Option<Message>,
    pub  edited_message: Option<Message>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    id: i64,
    r#type: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_forum: Option<bool>,
    is_direct_messages: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String
}