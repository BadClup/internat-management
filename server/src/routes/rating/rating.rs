use super::catering;
use crate::{error::ApiResult, AppState};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RatingType {
    Catering,
    Room,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RatingsDto {
    Catering(Vec<CateringRatingDto>),
    Room(Vec<RoomRatingDto>),
}

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

// GET specific
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRatingReq {
    pub room_id: Option<u32>,
    pub date: Option<DateTime<Utc>>,
}

pub async fn get_rating<'a>(
    Path(rating_type): Path<RatingType>,
    app_state: Extension<AppState>,
    Json(rating_options): Json<GetRatingReq>,
) -> ApiResult<'a, Json<RatingsDto>> {
    match rating_type {
        RatingType::Catering => {
            return catering::get::get_catering_rating(app_state, rating_options).await
        }
        RatingType::Room => todo!(),
    }
}

// POST specific
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PostRatingReq {
    Catering(PostCateringRatingReq),
    Room(Vec<PostRoomRatingReq>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCateringRatingReq {
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRoomRatingReq {
    pub stars: i32,
    pub room_number: u32,
}

pub async fn post_rating<'a>(
    Path(rating_type): Path<RatingType>,
    app_state: Extension<AppState>,
    header: HeaderMap,
    Json(new_ratings): Json<PostRatingReq>,
) -> ApiResult<'a, Json<RatingsDto>> {
    match new_ratings {
        PostRatingReq::Catering(catering_rating) => {
            if let RatingType::Catering = rating_type {
                return catering::post::post_catering_rating(app_state, header, catering_rating)
                    .await;
            } else {
                return ApiResult::Custom(
                    "Trying to rate room on catering route",
                    StatusCode::BAD_REQUEST,
                );
            }
        }
        PostRatingReq::Room(room_rating) => todo!(),
    }
}

// DELETE specific

pub async fn delete_rating(Path(rating_type): Path<RatingType>) {
    todo!()
}
