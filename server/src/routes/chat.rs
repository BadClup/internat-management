use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::HeaderMap;
use rdkafka::Message as KraftMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::error::ApiResult;
use crate::routes::user::auth::{get_user_from_header, UserRole};
use crate::utils::{kafka, web_sockets};
use crate::utils::web_sockets::WsMessage;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    header_map: HeaderMap,
) -> impl IntoResponse {
    let user_public_data = get_user_from_header(bearer_token);
    let user;

    match user_public_data {
        Ok(user_) => user = user_,
        Err(err) => return err,
    };

    let resident_id = match header_map.get("resident-id") {
        Some(header) => {
            let parsing_error = "Failed parsing resident-id header";

            header.to_str()
                .map_err(|_| ApiResult::Custom(parsing_error, StatusCode::BAD_REQUEST))
                .and_then(|v| v.parse::<u32>().map_err(|_| ApiResult::Custom(parsing_error, StatusCode::BAD_REQUEST)))
        }
        None => {
            if matches!(user.role, UserRole::Resident) {
                Ok(u32::try_from(user.id).unwrap()) // it is 100% sure that user.id (got from jwt) is unsigned
            } else {
                Err(ApiResult::Custom("Missing resident-id header", StatusCode::BAD_REQUEST))
            }
        }
    };

    let resident_id = match resident_id {
        Ok(resident_id) => resident_id,
        Err(err) => return err,
    };

    ApiResult::Ok(ws.on_upgrade(move |socket|
        handle_socket(socket, resident_id)
    ))
}

async fn handle_socket(ws: WebSocket, resident_id: u32) -> () {
    let ws = Arc::new(Mutex::new(ws));
    let ws_clone = ws.clone();

    let mut is_listening_kafka_events = true;

    tokio::spawn(async move {
        listen_for_messages(ws_clone, resident_id).await;
        is_listening_kafka_events = false;
    });

    while let Some(msg) = ws.lock().await.recv().await {
        if !is_listening_kafka_events {
            break;
        }

        if let Err(err) = msg {
            let err_msg = format!("Error encountered while trying to decode websocket message, {}", err.to_string());
            _ = ws_send_internal_error(&ws, err_msg).await;
            continue;
        }
        let msg = if let Ok(message) = msg.unwrap().into_text() {
            message
        } else {
            continue;
        };

        let msg_json: Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(_) => {
                _ = ws_send_error(&ws, "Your message is not a valid json", StatusCode::BAD_REQUEST).await;
                continue;
            }
        };

        let msg_type = if let Some(message_type) = msg_json.get("type") {
            message_type.to_string()
        } else {
            let message_type_field_not_found = "ERROR: Your request should contain \"type\" property".to_string();
            _ = ws_send_error(&ws, message_type_field_not_found, StatusCode::BAD_REQUEST).await;
            continue;
        };

        let unknown_message_type = format!("Unknown message type: {}", msg_type);

        match msg_type {
            _ => {
                _ = ws_send_error(&ws, unknown_message_type, StatusCode::BAD_REQUEST).await;
                continue;
            }
        };
    }
}

#[derive(Serialize, Deserialize)]
struct ChatTextMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatExitRequest {
    pub initial_location: (f64, f64),
    pub desired_location_name: String,
    pub request_content: String,

    pub approved_by: Option<u32>,
    pub approved_at: Option<i64>,

    pub came_back_at: Option<i64>,
    pub came_back_approved_by: Option<u32>,
}

#[derive(Serialize, Deserialize)]
enum ChatMessageType {
    Text(ChatTextMessage),
    ExitRequest(ChatExitRequest),
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_type: ChatMessageType,
    pub sender_id: i32,
    pub resident_id: i32,
    pub created_at: i64,
}

async fn listen_for_messages(ws: Arc<Mutex<WebSocket>>, resident_id: u32) {
    let kafka_consumer = match kafka::get_chat_consumer(resident_id) {
        Ok(consumer) => consumer,
        Err(_) => return,
    };

    while let Ok(msg) = kafka_consumer.recv().await {
        let msg = match msg.payload() {
            Some(message) => message,
            None => continue,
        };

        let msg = match std::str::from_utf8(msg) {
            Ok(message) => message,
            Err(_) => continue,
        };

        let msg: ChatMessage = match serde_json::from_str(msg) {
            Ok(message) => message,
            Err(_) => continue,
        };

        if msg.resident_id != resident_id as i32 {
            _ = ws_send_internal_error(&ws, "Unexpected error occurred").await;
        }
        
        match ws_send_chat_message(&ws, msg).await {
            Ok(_) => {},
            Err(_) => return, // We end the function because it is too important to skip potential errors
        };
    }
}

async fn ws_send_chat_message(ws: &Arc<Mutex<WebSocket>>, msg: ChatMessage) -> Result<(), axum_core::Error> {
    let msg = WsMessage::ChatMessage(msg);

    ws_send(ws, msg).await
}

async fn ws_send_error<T>(ws: &Arc<Mutex<WebSocket>>, msg: T, status_code: StatusCode) -> Result<(), axum_core::Error> where T: Into<String> {
    let msg = (msg.into(), status_code);
    let msg = WsMessage::Error(web_sockets::Error::from(msg));

    ws_send(ws, msg).await
}

async fn ws_send_internal_error<T>(ws: &Arc<Mutex<WebSocket>>, msg: T) -> Result<(), axum_core::Error> where T: Into<String> {
    let msg = msg.into();
    let msg = WsMessage::Error(web_sockets::Error::from(msg));

    ws_send(ws, msg).await
}

async fn ws_send(ws: &Arc<Mutex<WebSocket>>, msg: WsMessage) -> Result<(), axum_core::Error> {
    let msg_json = match serde_json::to_string(&msg) {
        Ok(v) => v,
        Err(e) => return Err(axum_core::Error::new(e)),
    };

    ws.lock().await.send(
        Message::Text(msg_json)
    ).await
}
