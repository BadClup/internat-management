use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::HeaderMap;
use rdkafka::Message as KraftMessage;

use crate::error::ApiResult;
use crate::routes::chat::types::CreateChatMessageDto;
use crate::routes::user::auth::{get_user_from_header, UserRole};
use crate::utils::{kafka, web_sockets};
use crate::utils::web_sockets::WsMessage;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    header_map: HeaderMap,
) -> impl IntoResponse {
    let user_public_data = get_user_from_header(bearer_token);

    let user = match user_public_data {
        Ok(user_) => user_,
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
        handle_kafka_chat_events(socket, resident_id)
    ))
}

async fn handle_kafka_chat_events(mut ws: WebSocket, resident_id: u32) {
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

        let msg: CreateChatMessageDto = match serde_json::from_str(msg) {
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
    
    _ = ws.close().await;
}


async fn ws_send_chat_message(ws: &mut WebSocket, msg: CreateChatMessageDto) -> Result<(), axum_core::Error> {
    let msg = WsMessage::ChatMessage(msg);

    ws_send(ws, msg).await
}

pub(crate) async fn ws_send_status_msg<T>(ws: &mut WebSocket, msg: T, status_code: StatusCode) -> Result<(), axum_core::Error> where T: Into<String> {
    let msg = (msg.into(), status_code);
    let msg = WsMessage::Message(web_sockets::StatusMessage::from(msg));

    ws_send(ws, msg).await
}

pub async fn ws_send_internal_error<T>(ws: &mut WebSocket, msg: T) -> Result<(), axum_core::Error> where T: Into<String> {
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
