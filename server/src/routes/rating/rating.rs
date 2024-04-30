use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{error::ApiResult, AppState};

use super::catering_rating::get_catering_rating;

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RatingType {
    Catering,
    Room,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Ratings {
    Catering(Vec<CateringRating>),
    Room(Vec<RoomRating>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RatingOptionsDto {
    pub room_id: Option<u32>,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CateringRating {
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
    pub dish_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomRating {
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub created_at: DateTime<Utc>,
    pub room_number: u32,
}

pub async fn get_rating<'a>(
    Path(rating_type): Path<RatingType>,
    app_state: Extension<AppState>,
    Json(rating_options): Json<RatingOptionsDto>,
) -> ApiResult<'a, Json<Ratings>> {
    match rating_type {
        RatingType::Catering => return get_catering_rating(app_state, rating_options).await,
        RatingType::Room => todo!()
    }
}

pub async fn post_rating(Path(rating_type): Path<RatingType>) {
    todo!()
}

pub async fn delete_rating(Path(rating_type): Path<RatingType>) {
    todo!()
}
