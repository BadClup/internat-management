use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CateringRatingDto {
    pub id: i32,
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
    pub dish_name: String,
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
