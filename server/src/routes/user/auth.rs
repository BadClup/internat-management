use std::str::FromStr;
use axum::{Extension, Json};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_extra::TypedHeader;
use axum_test::http::HeaderName;
use axum_test::TestServer;
use headers::{Authorization, HeaderValue};
use headers::authorization::Bearer;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256, Sha512};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

use crate::AppState;
use crate::error::ApiResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentSpecificData {
    pub room_id: u32,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Supervisor,
    Resident,
}

impl UserRole {
    pub fn to_string(&self) -> String {
        match self {
            UserRole::Supervisor => "supervisor".to_string(),
            UserRole::Resident => "resident".to_string(),
        }
    }
}

impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("user_role")
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserPublicData {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub room_nr: Option<i32>,
    pub role: UserRole,
}

impl IntoResponse for UserPublicData {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(json!(self))).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterDto {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub room_nr: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

pub fn get_user_from_header<'a, T>(token: TypedHeader<Authorization<Bearer>>) -> Result<UserPublicData, ApiResult<'a, T>> {
    match deserialize_jwt(token.token()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ApiResult::Custom(
            "Failed to deserialize jwt from Bearer token", StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn login<'a>(
    Extension(app_state): Extension<AppState>,
    Json(user_credentials): Json<UserCredentials>,
) -> ApiResult<'a, Json<serde_json::Value>> {
    let user = sqlx::query!(r#"
            SELECT id, username, first_name, last_name, room_number as room_nr, role as "role: UserRole", password
            FROM "user"
            WHERE username = $1
        "#,
        user_credentials.username,
    )
        .fetch_optional(&app_state.db_pool)
        .await;

    if let Err(e) = user {
        return ApiResult::from(e);
    }
    let user = user.unwrap();

    if let None = user {
        return ApiResult::Custom("Invalid username", StatusCode::UNAUTHORIZED);
    }
    let user = user.unwrap();

    if sha512(&user_credentials.password) != user.password {
        return ApiResult::Custom("Invalid password", StatusCode::UNAUTHORIZED);
    }

    let user_public_data = UserPublicData {
        id: user.id,
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        room_nr: user.room_nr,
        role: user.role,
    };
    
    let jwt = match serialize_jwt(user_public_data.clone()) {
        Ok(val) => val,
        Err(e) => return ApiResult::from(e),
    };

    ApiResult::Ok(Json(json!({
        "bearer_token": jwt,
        "user": user_public_data,
    })))
}

pub fn serialize_jwt(val: UserPublicData) -> anyhow::Result<String> {
    let secret = std::env::var("SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let header = get_jwt_header();
    let token_str = Token::new(header, val).sign_with_key(&key)?;

    Ok(token_str.as_str().to_string())
}

pub fn deserialize_jwt(token: &str) -> anyhow::Result<UserPublicData> {
    let secret = std::env::var("SECRET")?;

    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let token: Token<Header, UserPublicData, _> =
        VerifyWithKey::verify_with_key(token, &key)?;

    Ok(token.claims().clone())
}

fn get_jwt_header() -> Header {
    Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    }
}

pub async fn register_residents<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(users_data): Json<Vec<UserRegisterDto>>,
) -> ApiResult<'a, Json<Vec<UserCredentials>>> {
    let user_public_data = get_user_from_header(bearer_token);
    let user;

    match user_public_data {
        Ok(user_) => user = user_,
        Err(err) => return err,
    };

    // If not supervisor
    if !matches!(user.role, UserRole::Supervisor) {
        return ApiResult::Custom(
            "You need to be a supervisor in order to register new residents",
            StatusCode::FORBIDDEN,
        );
    }

    if users_data.len() == 0 {
        return ApiResult::Custom("No residents to register", StatusCode::BAD_REQUEST);
    }

    let mut usernames = vec![];
    let mut passwords = vec![];
    let mut hashed_passwords = vec![];
    let mut first_names = vec![];
    let mut last_names = vec![];
    let mut room_numbers = vec![];
    let mut user_roles = vec![];

    for user_data in users_data {
        println!("user_data: {:?}\n\n", user_data);
        usernames.push(user_data.username);
        first_names.push(user_data.first_name);
        last_names.push(user_data.last_name);
        room_numbers.push(user_data.room_nr as i32);
        user_roles.push(UserRole::Resident.to_string());

        let password = generate_random_password(8);

        hashed_passwords.push(sha512(&password));
        passwords.push(password);
    }

    let query = sqlx::query!(r#"
        INSERT INTO "user" (username, password, first_name, last_name, room_number, role)
        SELECT username, password, first_name,last_name, room_number, role
        FROM UNNEST(
           $1::text[],
           $2::text[],
           $3::text[],
           $4::text[],
           $5::int[],
           $6::user_role[]
        ) as t(username, password, first_name, last_name, room_number, role); "#,
        usernames.clone() as Vec<String>,
        hashed_passwords as Vec<String>,
        first_names as Vec<String>,
        last_names as Vec<String>,
        room_numbers as Vec<i32>,
        user_roles as Vec<String>,
    )
        .execute(&app_state.db_pool).await;

    if let Err(e) = query {
        return ApiResult::from(e);
    }

    let mut users_credentials = vec![];

    for i in 0..usernames.len() {
        users_credentials.push(UserCredentials {
            username: usernames[i].clone(),
            password: passwords[i].clone(),
        });
    }

    return ApiResult::Ok(Json(users_credentials));
}

fn sha512(data: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();

    let mut hash_string = "".to_string();
    for byte in hash.iter() {
        hash_string = format!("{}{:02x}", hash_string, byte);
    }

    hash_string
}

fn generate_random_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[test]
fn test_generate_random_password() {
    let password = generate_random_password(2137);
    assert_eq!(password.len(), 2137);
}

#[tokio::test]
async fn test_users_register() {
    let users_data = vec![
        UserRegisterDto {
            username: "test1".to_string(),
            first_name: "Test".to_string(),
            last_name: "One".to_string(),
            room_nr: 1,
        },
        UserRegisterDto {
            username: "test2".to_string(),
            first_name: "Test".to_string(),
            last_name: "Two".to_string(),
            room_nr: 2,
        },
    ];

    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    struct TestUser {
        user_role: UserRole,
        bearer_token: String,
    }

    let test_users = vec![
        // "bstrama" account from seed.sql
        TestUser {
            user_role: UserRole::Resident,
            bearer_token: "Bearer eyJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwidXNlcm5hbWUiOiJic3RyYW1hIiwiZmlyc3RfbmFtZSI6IkJhcnTFgm9taWVqIiwibGFzdF9uYW1lIjoiU3RyYW1hIiwicm9vbV9uciI6MSwicm9sZSI6IlJlc2lkZW50In0.NDRhw0XKORAiAeCP2uS0Z2-2wema8A_9nHNiScZkJyA".to_owned(),
        },
        // "kierowniczka" account from seed.sql
        TestUser {
            user_role: UserRole::Supervisor,
            bearer_token: "Bearer eyJhbGciOiJIUzI1NiJ9.eyJpZCI6MiwidXNlcm5hbWUiOiJraWVyb3duaWN6a2EiLCJmaXJzdF9uYW1lIjoiQm9ndW1pxYJhIiwibGFzdF9uYW1lIjoiS2FwdHVya2lld2ljeiIsInJvb21fbnIiOm51bGwsInJvbGUiOiJTdXBlcnZpc29yIn0.SlkzM77qydnK3raBIfwfVtisdwXy5QzgvRoUVEspsMI".to_owned(),
        },
    ];

    for user in test_users {
        let res = server.post("/user/register-many")
            .content_type("application/json")
            .add_header(
                HeaderName::from_str("Authorization").unwrap(),
                HeaderValue::from_str(&user.bearer_token).unwrap(),
            )
            .json(&json!(users_data))
            .await;

        match user.user_role {
            UserRole::Resident => res.assert_status_forbidden(),
            UserRole::Supervisor => {
                res.assert_status_ok();

                // cleanup
                sqlx::query!("DELETE FROM \"user\" WHERE username = 'test1' OR username = 'test2'")
                    .execute(&app_state.db_pool)
                    .await
                    .expect("Failed to delete test users");
            }
        }
    }
}

