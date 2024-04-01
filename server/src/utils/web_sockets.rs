use axum_test::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::routes::chat;

#[derive(Serialize, Deserialize)]
pub enum WsMessage {
    #[serde(rename = "error")]
    Error(Error),
    #[serde(rename = "chat_message")]
    ChatMessage(chat::ChatMessage),
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub status_code: u16,
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self { message, status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16() }
    }
}

impl From<(String, StatusCode)> for Error {
    fn from((message, status_code): (String, StatusCode)) -> Self {
        Self { message, status_code: status_code.as_u16() }
    }
}
