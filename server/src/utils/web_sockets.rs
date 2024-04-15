use axum_test::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::routes::chat;

#[derive(Serialize, Deserialize)]
pub enum WsMessage {
    #[serde(rename = "status_message")]
    Message(StatusMessage),
    #[serde(rename = "chat_message")]
    ChatMessage(chat::ChatMessage),
}

#[derive(Serialize, Deserialize)]
pub struct StatusMessage {
    pub message: String,
    pub status_code: u16,
}

impl From<(String, StatusCode)> for StatusMessage {
    fn from((message, status_code): (String, StatusCode)) -> Self {
        Self { message, status_code: status_code.as_u16() }
    }
}
