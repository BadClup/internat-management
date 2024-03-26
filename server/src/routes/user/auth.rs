use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::{Extension, Json};
use axum::http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use crate::AppState;
use crate::error::ApiResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentSpecificData {
    pub room_id: u32,
}


#[allow(non_camel_case_types)]
#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    supervisor,
    resident,
}

impl UserRole {
    pub fn to_string(&self) -> String {
        match self {
            UserRole::supervisor => "supervisor".to_string(),
            UserRole::resident => "resident".to_string(),
        }
    }
}

impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("user_role")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublicData {
    pub id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub room_nr: Option<u32>,
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
        usernames.push(user_data.username);
        first_names.push(user_data.first_name);
        last_names.push(user_data.last_name);
        room_numbers.push(user_data.room_nr as i32);
        user_roles.push(UserRole::resident.to_string());

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
        FROM UNNEST($1::text[]) username,
           UNNEST($2::text[]) password,
           UNNEST($3::text[]) first_name,
           UNNEST($4::text[]) last_name,
           UNNEST($5::int[]) room_number,
           UNNEST($6::user_role[]) role;
    "#,
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
