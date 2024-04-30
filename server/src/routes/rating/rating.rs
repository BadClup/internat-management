use axum::{extract::Path, Extension, Json};
use axum_test::TestServer;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::{error::ApiResult, AppState};

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RatingType {
    Catering,
    Room,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RatingOptionsDto {
    pub room_id: Option<u32>,
    //pub date
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub created_at: DateTime<Utc>,
    pub served_at: DateTime<Utc>,
    pub dish_name: String
}

pub async fn get_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    rating_options: RatingOptionsDto,
) -> ApiResult<'a, Json<Vec<Rating>>> {
    let ratings: Result<Vec<Rating>, sqlx::Error> = sqlx::query_as!(Rating, r#"
            SELECT stars, created_at as "created_at: DateTime<Utc>", served_at as "served_at: DateTime<Utc>", dish_name FROM "catering_rating" cr
            JOIN "rating" r ON r.id = cr.rating_id
            JOIN "catering" c ON c.id = cr.catering_id
            JOIN "dish" d ON d.id = c.dish_id
        "#,
    )
        .fetch_all(&app_state.db_pool)
        .await;

    if let Err(e) = ratings {
        return ApiResult::Sqlx(e);
    };

    let processed_ratings = ratings.unwrap();

    return ApiResult::Ok(Json(processed_ratings));
}

pub async fn get_room_rating<'a>(
    Extension(app_state): Extension<AppState>,
    rating_options: RatingOptionsDto,
) -> ApiResult<'a, Json<Vec<Rating>>> {
    todo!()
}
pub async fn get_rating<'a>(Path(rating_type): Path<RatingType>,
    app_state: Extension<AppState>,
    Json(rating_options): Json<RatingOptionsDto>,
) -> ApiResult<'a, Json<Vec<Rating>>> {
    match rating_type {
        RatingType::Catering => return get_catering_rating(app_state, rating_options).await,
        RatingType::Room => return get_room_rating(app_state, rating_options).await,
    }
}

#[tokio::test]
async fn test_get_ratings() {
    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    let expected_output = json!([
        {
                "stars": 4,
                "created_at": "2020-01-04T00:00:00Z",
                "served_at": "2020-01-01T00:00:00Z",
                "dish_name": "kurczak z ryżem"
        },
        {
                "stars": 4,
                "created_at": "2020-01-05T00:00:00Z",
                "served_at": "2020-01-02T00:00:00Z",
                "dish_name": "kurczak z kurczakiem"
        },
        {
                "stars": 4,
                "created_at": "2020-01-06T00:00:00Z",
                "served_at": "2020-01-03T00:00:00Z",
                "dish_name": "kurczak z kurczakiem"
        }
    ]);

    let res = server.get("/rating/catering")
        .content_type("application/json")
        .json(&json!({}))
        .await;

    res.assert_json(&expected_output);
    res.assert_status_ok();
}

pub async fn post_rating(Path(rating_type): Path<RatingType>) {
    todo!()
}

pub async fn delete_rating(Path(rating_type): Path<RatingType>) {
    todo!()
}
