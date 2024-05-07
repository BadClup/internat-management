use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatTextMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize, sqlx::Type, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatExitRequest {
    pub initial_location: LatLong,
    pub desired_location_name: String,
    pub request_content: String,

    pub approved_by: Option<u32>,
    pub approved_at: Option<i64>,

    pub came_back_at: Option<i64>,
    pub came_back_approved_by: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatMessageType {
    Text(ChatTextMessage),
    ExitRequest(ChatExitRequest),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    #[serde(flatten)]
    pub message_type: ChatMessageType,
    pub resident_id: i32,
    pub created_at: Option<String>,
}
