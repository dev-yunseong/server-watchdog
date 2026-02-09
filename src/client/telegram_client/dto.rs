use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SendMessageDto {
    chat_id: String,
    text: String,
    reply_markup: Option<ReplyMarkup>
}

#[derive(Serialize, Deserialize)]
pub struct ReplyMarkup {
    pub inline_keyboard: Vec<InlineKeyboardButton>
}

#[derive(Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>
}

#[derive(Deserialize)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub result: T,
    pub error_code: Option<i16>,
    pub description: Option<String>,
}

impl SendMessageDto {
    pub fn new(chat_id: &str, text: &str, reply_markup: Option<ReplyMarkup>) -> Self {
        Self {
            chat_id: chat_id.to_string(),
            text: text.to_string(),
            reply_markup
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub from: User,
    pub date: i64,
    pub chat: Chat,
    pub text: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub callback_query: Option<CallbackQuery>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub is_direct_messages: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String
}