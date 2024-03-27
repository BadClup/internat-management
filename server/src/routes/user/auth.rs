use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum_test::TestServer;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use crate::AppState;
use crate::error::ApiResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentSpecificData {
    pub room_id: u32,
}


#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
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

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct UserPublicData {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub room_nr: Option<i32>,
    pub role: UserRole,
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
        return ApiResult::Sqlx(e);
    }
    let user = user.unwrap();

    if let None = user {
        return ApiResult::Custom("Invalid username", StatusCode::UNAUTHORIZED);
    }
    let user = user.unwrap();

    let db_password_hash;
    match PasswordHash::new(&user.password) {
        Ok(v) => { db_password_hash = v }
        Err(e) => { return ApiResult::Unknown(e.to_string()); }
    };

    let verify_result = Argon2::default()
        .verify_password(user_credentials.password.as_bytes(), &db_password_hash);

    if let Err(_) = verify_result {
        return ApiResult::Custom("Password is incorrect", StatusCode::UNAUTHORIZED);
    }

    let jwt = serialize_jwt(UserPublicData {
        id: user.id,
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        room_nr: user.room_nr,
        role: user.role,
    });
    if let Err(e) = jwt {
        return ApiResult::Anyhow(e);
    }
    let jwt = jwt.unwrap();

    ApiResult::Ok(Json(json!({
        "bearer_token": jwt,
    })))
}

pub fn serialize_jwt(val: UserPublicData) -> anyhow::Result<String> {
    let secret = std::env::var("SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let header = get_jwt_header();
    let token_str = Token::new(header, val).sign_with_key(&key)?;

    Ok(token_str.as_str().to_string())
}

fn get_jwt_header() -> Header {
    Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    }
}

pub async fn register_users<'a>(
    Extension(app_state): Extension<AppState>,
    Json(users_data): Json<Vec<UserRegisterDto>>,
) -> ApiResult<'a, Json<Vec<UserCredentials>>> {
    if users_data.len() == 0 {
        return ApiResult::Custom("No users to register", StatusCode::BAD_REQUEST);
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
        let hashed_password = hash_password(&password);

        if let Err(e) = hashed_password {
            return ApiResult::Anyhow(e);
        }

        hashed_passwords.push(hashed_password.unwrap());
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
        return ApiResult::Sqlx(e);
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

    let res = server.post("/user/register-many")
        .content_type("application/json")
        .json(&json!(users_data))
        .await;

    res.assert_status_ok();

    sqlx::query!("DELETE FROM \"user\" WHERE username = 'test1' OR username = 'test2'")
        .execute(&app_state.db_pool)
        .await
        .expect("Failed to delete test users");
}

fn hash_password(passwd: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_passwd = argon2
        .hash_password(passwd.as_bytes(), &salt)
        .map_err(|_| anyhow::Error::msg("Failed to hash password"))?;

    Ok(hashed_passwd.to_string())
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
