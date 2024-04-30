use axum::{Extension, Json};
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde_json::json;

use super::rating::{CateringRating, RatingOptionsDto, Ratings};
use crate::{error::ApiResult, AppState};

pub async fn get_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    rating_options: RatingOptionsDto,
) -> ApiResult<'a, Json<Ratings>> {
    let ratings: Result<Vec<CateringRating>, sqlx::Error>= match rating_options.date {
        Some(option)=> sqlx::query_as!(CateringRating, r#"
            SELECT stars, created_at as "created_at: DateTime<Utc>", served_at as "served_at: DateTime<Utc>", dish_name FROM "catering_rating" cr
            JOIN "rating" r ON r.id = cr.rating_id
            JOIN "catering" c ON c.id = cr.catering_id
            JOIN "dish" d ON d.id = c.dish_id
            WHERE served_at = $1
        "#,
        option as _
    )
        .fetch_all(&app_state.db_pool)
        .await,
        None => sqlx::query_as!(CateringRating, r#"
            SELECT stars, created_at as "created_at: DateTime<Utc>", served_at as "served_at: DateTime<Utc>", dish_name FROM "catering_rating" cr
            JOIN "rating" r ON r.id = cr.rating_id
            JOIN "catering" c ON c.id = cr.catering_id
            JOIN "dish" d ON d.id = c.dish_id
        "#,
    )
        .fetch_all(&app_state.db_pool)
        .await,
    };

    if let Err(e) = ratings {
        return ApiResult::Sqlx(e);
    };

    let processed_ratings = ratings.unwrap();

    return ApiResult::Ok(Json(Ratings::Catering(processed_ratings)));
}

#[tokio::test]
async fn test_get_ratings() {
    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    let expected_output = json!({ "catering": [
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
    ]});

    let res = server
        .get("/rating/catering")
        .content_type("application/json")
        .json(&json!({}))
        .await;

    res.assert_json(&expected_output);
    res.assert_status_ok();
}
