use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ChatTextMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Debug)]
#[sqlx(type_name = "lat_long")]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ChatExitRequest {
    pub initial_location: LatLong,
    pub desired_location_name: String,
    pub request_content: Option<String>,

    pub approved_by: Option<u32>,
    pub approved_at: Option<i64>,

    pub came_back_at: Option<i64>,
    pub came_back_approved_by: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatMessageKind {
    Text(ChatTextMessage),
    ExitRequest(ChatExitRequest),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateChatMessageDto {
    #[serde(flatten)]
    pub message_kind: ChatMessageKind,
    pub resident_id: i32,
    pub created_at: Option<String>,
    pub reply_to: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct GetChatMessageDto {
    #[serde(flatten)]
    pub message_kind: ChatMessageKind,
    pub id: i32,
    pub reply_to: Option<i32>,
    pub recipient_id: i32,
    pub sender_id: i32,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ConversationListElement {
    pub recipient_id: i32,
    pub sender_id: Option<i32>,
    pub recent_message_date: Option<String>,
    pub recent_message: Option<String>,
}
