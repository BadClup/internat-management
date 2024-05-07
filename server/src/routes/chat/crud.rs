use axum::{Extension, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use sqlx::PgPool;
use crate::AppState;
use crate::error::ApiResult;
use crate::routes::chat::{ChatMessage, ChatMessageType, LatLong};
use crate::routes::user::auth::{get_user_from_header, UserPublicData};
use crate::utils::kafka;

pub async fn send_message_controller<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(msg): Json<ChatMessage>,
) -> ApiResult<'a, &'a str> {
    let user_public_data = get_user_from_header(bearer_token);

    let user = match user_public_data {
        Ok(user_) => user_,
        Err(err) => return err,
    };
      
    let result =  match msg.message_type {
        ChatMessageType::Text(_) => send_chat_message(msg, &app_state.db_pool, &user).await,
        _ => return ApiResult::Code(StatusCode::BAD_REQUEST),
    };
    
    result.map(|_| "Message sent")
}

async fn send_chat_message<'a>(mut msg: ChatMessage, db_pool: &PgPool, user: &UserPublicData) -> ApiResult<'a, ()> {
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
        Err(_) => {
            return ApiResult::Internal("Internal database error".to_string());
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

    if let Err(_) = pg_query {
        return ApiResult::Internal("Internal database error".to_string());
    }

    match kafka::send_chat_message(msg, db_message.id as u32).await {
        Ok(_) => ApiResult::Ok(()),
        Err(_) => ApiResult::Internal("Internal kafka error".to_string())
    }
}
