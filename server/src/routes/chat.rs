use std::fmt::format;
use std::future::Future;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Router};
use axum::routing::get;
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::HeaderMap;
use rdkafka::error::KafkaResult;
use rdkafka::Message as KraftMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Error, PgPool};
use sqlx::postgres::PgQueryResult;
use tokio::sync::Mutex;
use crate::AppState;

use crate::error::ApiResult;
use crate::routes::user::auth::{get_user_from_header, UserPublicData, UserRole};
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
    Extension(app_state): Extension<AppState>,
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
        handle_socket(socket, app_state, resident_id, user)
    ))
}

async fn handle_socket(ws: WebSocket, app_state: AppState, resident_id: u32, user: UserPublicData) -> () {
    let ws = Arc::new(Mutex::new(ws));
    let ws_clone = ws.clone();

    let mut is_listening_kafka_events = true;

    tokio::spawn(async move {
        listen_for_messages(ws_clone, resident_id).await;
        is_listening_kafka_events = false;
    });
    loop {
        let mut ws = ws.lock().await;

        let msg = if let Some(message) = ws.recv().await {
            message
        } else {
            continue;
        };

        if !is_listening_kafka_events {
            break;
        }

        if let Err(err) = msg {
            let err_msg = format!("Error encountered while trying to decode websocket message, {}", err.to_string());
            _ = ws_send_internal_error(&mut ws, err_msg).await;
            continue;
        }
        let msg = if let Ok(message) = msg.unwrap().into_text() {
            message
        } else {
            continue;
        };

        let ws_msg: WsMessage = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(err) => {
                let err_msg = format!("Your request json in not a WsMessage type, error message: {}", err.to_string());
                _ = ws_send_status_msg(&mut ws, err_msg, StatusCode::BAD_REQUEST).await;

                continue;
            }
        };

        match ws_msg {
            WsMessage::ChatMessage(msg) => {
                send_chat_message(&mut ws, msg, &app_state.db_pool, &user).await;
            }
            _ => {
                _ = ws_send_status_msg(&mut ws, "unknown message type", StatusCode::BAD_REQUEST).await;
            }
        };
    }

    if let Ok(ws) = Arc::try_unwrap(ws) {
        _ = ws.into_inner().close().await;
    };
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatTextMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize, sqlx::Type, Clone)]
struct LatLong {
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatExitRequest {
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

async fn listen_for_messages(ws: Arc<Mutex<WebSocket>>, resident_id: u32) {
    let kafka_consumer = match kafka::get_chat_consumer(resident_id) {
        Ok(consumer) => consumer,
        Err(_) => return,
    };

    while let Ok(msg) = kafka_consumer.recv().await {
        let mut ws = ws.lock().await;

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
            _ = ws_send_status_msg(&mut ws, "Cannot send a message to someone else chat", StatusCode::BAD_REQUEST).await;
        }

        match ws_send_chat_message(&mut ws, msg).await {
            Ok(_) => {}
            Err(_) => return, // We end the function because it is too important to skip potential errors
        };
    }
}

async fn send_chat_message(ws: &mut WebSocket, mut msg: ChatMessage, db_pool: &PgPool, user: &UserPublicData) {
    let db_message = sqlx::query!(
        r#"
            INSERT INTO "message" (sender_id, recipient_id, created_at)
            VALUES ($1, $2, NOW())
            RETURNING id, created_at
        "#,
        user.id,
        msg.resident_id,
    )
        .fetch_one(db_pool)
        .await;

    let db_message = match db_message {
        Ok(msg) => msg,
        Err(err) => {
            _ = ws_send_internal_error(ws, err.to_string()).await;
            return;
        }
    };
    
    msg.created_at = Some(db_message.created_at.to_string());

    let pg_query = match msg.clone().message_type {
        ChatMessageType::Text(text_message) => {
            sqlx::query!(r#"
                            INSERT INTO "text_message" (message_id, content)
                            VALUES ($1, $2)
                        "#,
                        db_message.id,
                        text_message.content,
                    )
                .execute(db_pool)
                .await
        }
        ChatMessageType::ExitRequest(exit_request) => {
            sqlx::query!(r#"
                            INSERT INTO "exit_request_message" (message_id, initial_location, desired_location_name, request_content)
                            VALUES ($1, $2, $3, $4)
                        "#,
                        db_message.id,
                        exit_request.initial_location as LatLong,
                        exit_request.desired_location_name,
                        exit_request.request_content,
                    )
                .execute(db_pool)
                .await
        }
    };
    
    if let Err(err) = pg_query {
        _ = ws_send_internal_error(ws, err.to_string()).await;
        return;
    }
    
    match kafka::send_chat_message(msg, db_message.id as u32).await {
        Ok(_) => {
            _ = ws_send_status_msg(ws, "Message sent", StatusCode::OK).await;
        },
        Err(_) => {
            _ = ws_send_internal_error(ws, "Failed to send message to kafka").await;
        },
    };
}

async fn ws_send_chat_message(ws: &mut WebSocket, msg: ChatMessage) -> Result<(), axum_core::Error> {
    let msg = WsMessage::ChatMessage(msg);

    ws_send(ws, msg).await
}

async fn ws_send_status_msg<T>(ws: &mut WebSocket, msg: T, status_code: StatusCode) -> Result<(), axum_core::Error> where T: Into<String> {
    let msg = (msg.into(), status_code);
    let msg = WsMessage::Message(web_sockets::StatusMessage::from(msg));

    ws_send(ws, msg).await
}

async fn ws_send_internal_error<T>(ws: &mut WebSocket, msg: T) -> Result<(), axum_core::Error> where T: Into<String> {
    let msg = msg.into();
    let msg = WsMessage::Message(web_sockets::StatusMessage::from((msg, StatusCode::INTERNAL_SERVER_ERROR)));

    ws_send(ws, msg).await
}

async fn ws_send(ws: &mut WebSocket, msg: WsMessage) -> Result<(), axum_core::Error> {
    let msg_json = match serde_json::to_string(&msg) {
        Ok(v) => v,
        Err(e) => return Err(axum_core::Error::new(e)),
    };

    ws.send(
        Message::Text(msg_json)
    ).await
}
