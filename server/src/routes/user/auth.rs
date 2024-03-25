use axum::Json;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::error::ApiResult;

#[derive(Serialize, Deserialize, Debug)]
enum Role {
    Manager,
    Student(StudentSpecificData),
    SupervisingTeacher,
}

#[derive(Serialize, Deserialize, Debug)]
struct StudentSpecificData {
    room_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserPublicData {
    id: u32,
    username: String,
    first_name: String,
    last_name: String,
    room_nr: Option<u32>,
}


#[derive(Serialize, Deserialize, Debug)]
struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserRegisterDto {
    username: String,
    first_name: String,
    last_name: String,
    room_nr: u32,
}

pub fn register_users(
    Json(users_data): Json<Vec<UserRegisterDto>>
) -> ApiResult<Json<Vec<UserCredentials>>> {
    todo!()
}

fn generate_random_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
