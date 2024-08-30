use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct MealSubratingDto {
    pub id: i32,
    pub question: String,
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MealRatingDto {
    pub id: i32,
    pub points: i32,
    //#[serde(with = "DateTime")]
    //pub created_at: DateTime<Utc>,
    pub subratings: Vec<MealSubratingDto>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MealDto {
    pub id: i32,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
    pub dish_name: String,
    pub ratings: Option<Json<Vec<MealRatingDto>>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomRatingDto {
    pub id: i32,
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub created_at: DateTime<Utc>,
    pub room_number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRoomRatingReq {
    pub stars: i32,
    pub room_number: u32,
}
