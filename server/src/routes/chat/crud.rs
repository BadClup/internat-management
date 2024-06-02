use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use sqlx::PgPool;
use crate::AppState;
use crate::error::ApiResult;
use crate::routes::chat::{CreateChatMessageDto, ChatMessageKind, LatLong, GetChatMessageDto, ChatTextMessage, ChatExitRequest};
use crate::routes::user::auth::{get_user_from_header, UserPublicData};
use crate::utils::kafka;
use crate::utils::request_types::{DEFAULT_SKIP, DEFAULT_TAKE, TakeSkip};

pub async fn send_message_controller<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(msg): Json<CreateChatMessageDto>,
) -> ApiResult<'a, &'a str> {
    let user_public_data = get_user_from_header(bearer_token);

    let user = match user_public_data {
        Ok(user_) => user_,
        Err(err) => return err,
    };

    let result = send_chat_message(msg, app_state.db_pool, user).await;

    result.map(|_| "Message sent")
}

async fn send_chat_message<'a>(mut msg: CreateChatMessageDto, db_pool: PgPool, user: UserPublicData) -> ApiResult<'a, ()> {
    let db_message = sqlx::query!(
        r#"
            INSERT INTO "message" (sender_id, recipient_id, reply_to, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id, created_at
        "#,
        user.id,
        msg.resident_id,
        msg.reply_to,
    )
        .fetch_one(&db_pool)
        .await;

    let db_message = match db_message {
        Ok(msg) => msg,
        Err(_) => {
            return ApiResult::Internal("Internal database error".to_string());
        }
    };

    msg.created_at = Some(db_message.created_at.to_string());

    let pg_query = match msg.clone().message_kind {
        ChatMessageKind::Text(text_message) => {
            sqlx::query!(r#"
                            INSERT INTO "text_message" (message_id, content)
                            VALUES ($1, $2)
                        "#,
                        db_message.id,
                        text_message.content,
                    )
                .execute(&db_pool)
                .await
        }
        ChatMessageKind::ExitRequest(exit_request) => {
            sqlx::query!(r#"
                            INSERT INTO "exit_request_message" (message_id, initial_location, desired_location_name, request_content)
                            VALUES ($1, $2, $3, $4)
                        "#,
                        db_message.id,
                        exit_request.initial_location as LatLong,
                        exit_request.desired_location_name,
                        exit_request.request_content,
                    )
                .execute(&db_pool)
                .await
        }
    };

    if let Err(err) = pg_query {
        return ApiResult::from(err);
    }

    match kafka::send_chat_message(msg, db_message.id as u32).await {
        Ok(_) => ApiResult::Ok(()),
        Err(_) => ApiResult::Internal("Internal kafka error".to_string())
    }
}

pub async fn get_messages_controller<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Query(TakeSkip { take, skip }): Query<TakeSkip>,
    Path(recipient_id): Path<u32>,
) -> ApiResult<'a, Json<Vec<GetChatMessageDto>>> {
    let take = take.unwrap_or(DEFAULT_TAKE);
    let skip = skip.unwrap_or(DEFAULT_SKIP);

    let user_public_data = get_user_from_header(bearer_token);
    let user = match user_public_data {
        Ok(user_) => user_,
        Err(err) => return err,
    };

    get_messages(app_state.db_pool, user, take, skip, recipient_id)
        .await
        .map(|elem| Json(elem))
}

async fn get_messages<'a>(
    db_pool: PgPool,
    user: UserPublicData,
    take: u32,
    skip: u32,
    recipient_user_id: u32,
) -> ApiResult<'a, Vec<GetChatMessageDto>> {
    let messages = sqlx::query!(r#"
            SELECT m.*,
                t.content as "content: Option<String>",
                er.initial_location as "initial_location: Option<LatLong>",
                er.desired_location_name as "desired_location_name: Option<String>",
                er.request_content,
                er.approved_by,
                er.approved_at,
                er.came_back_at,
                er.came_back_approved_by
            FROM message as m
            LEFT JOIN text_message t ON t.message_id = m.id
            LEFT JOIN exit_request_message er ON er.message_id = m.id
            WHERE sender_id = $1 AND recipient_id = $2
            ORDER BY created_at DESC
            LIMIT $3
            OFFSET $4;
        "#,
        user.id as i32,
        recipient_user_id as i32,
        take as i32,
        skip as i32,
    )
        .fetch_all(&db_pool)
        .await;

    if let Err(err) = messages {
        return ApiResult::from(err);
    }
    let messages = messages.unwrap();

    let mut result = vec![];

    for msg in messages {
        // it checks if message kind is text_message:
        if let Some(content) = msg.content {
            result.push(GetChatMessageDto {
                recipient_id: msg.recipient_id,
                id: msg.id,
                reply_to: msg.reply_to,
                sender_id: msg.sender_id,
                created_at: msg.created_at.to_string(),
                message_kind: ChatMessageKind::Text(ChatTextMessage {
                    content,
                }),
            });
        } else {
            if msg.desired_location_name.is_none() || msg.initial_location.is_none() {
                // It means that it is neither text_message nor exit_request_message
                eprintln!("Invalid internal sql message kind");
                continue;
            }

            result.push(GetChatMessageDto {
                recipient_id: msg.recipient_id,
                id: msg.id,
                reply_to: msg.reply_to,
                sender_id: msg.sender_id,
                created_at: msg.created_at.to_string(),
                message_kind: ChatMessageKind::ExitRequest(ChatExitRequest {
                    approved_by: msg.approved_by.map(|x| x as u32),
                    came_back_approved_by: msg.came_back_approved_by.map(|x| x as u32),
                    request_content: msg.request_content,
                    approved_at: msg.approved_at.map(|e| e.assume_utc().unix_timestamp().into()),
                    came_back_at: msg.came_back_at.map(|e| e.assume_utc().unix_timestamp().into()),
                    // it is safe to unwrap because we checked it before
                    desired_location_name: msg.desired_location_name.unwrap(),
                    initial_location: msg.initial_location.unwrap(),
                }),
            });
        }
    }

    ApiResult::Ok(result)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use axum::http::{HeaderName, HeaderValue};
    use axum_test::TestServer;
    use serde_json::json;
    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        test_send_message_().await;
    }

    async fn test_send_message_() {
        let app_state = AppState::new().await;
        let app = crate::get_app(app_state.clone());
        let server = TestServer::new(app).expect("Failed to create test server");

        let bearer_token = crate::utils::tests::login_returning_bearer_token().await;
        let request_body = json!({
            "content": "Example message",
            "resident_id": 1,
        });

        let res = server
            .post("/chat")
            .content_type("application/json")
            .add_header(
                HeaderName::from_str("Authorization").unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap(),
            )
            .json(&request_body)
            .await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_messages() {
        let app_state = AppState::new().await;
        let app = crate::get_app(app_state.clone());
        let server = TestServer::new(app).expect("Failed to create test server");

        // start by populating the database with some messages:
        test_send_message_().await;

        let bearer_token = crate::utils::tests::login_returning_bearer_token().await;

        let res = server
            .get("/chat/1")
            .add_query_params(json!({
                "take": 10,
                "skip": 0,
            }))
            .add_header(
                HeaderName::from_str("Authorization").unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap(),
            )
            .await;

        println!("{:?}", res);

        res.assert_status_ok();
        let json_response: serde_json::Value = res.json();

        assert!(json_response.is_array());
        assert!(json_response.as_array().unwrap().len() > 0);
    }
}