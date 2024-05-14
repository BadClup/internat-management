use std::str::FromStr;

use axum::{
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde_json::json;

use super::super::rating::{CateringRatingDto, RatingsDto};
use crate::routes::{rating::rating::PostRatingReq, user::auth::{get_user_from_bearer, UserRole}};
use crate::{
    error::ApiResult,
    routes::rating::rating::{PostCateringRatingReq},
    AppState,
};

pub async fn post_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    header: HeaderMap,
    new_rating: PostCateringRatingReq,
) -> ApiResult<'a, Json<RatingsDto>> {
    let user_public_data;

    match get_user_from_bearer(header) {
        Ok(v) => user_public_data = v,
        Err(e) => return e,
    };

    if !matches!(user_public_data.role, UserRole::Resident) {
        return ApiResult::Custom(
            "You need to be a resident in order to rate catering",
            StatusCode::FORBIDDEN,
        );
    }

    let query = sqlx::query_as!(
        CateringRatingDto,
        r#"
        WITH new_rating AS (
            INSERT INTO "rating" (user_id, stars)
            VALUES ($1, $2)
            RETURNING id, user_id, created_at, stars
        ), proper_catering AS (
            SELECT c.id, served_at, dish_name
            FROM "catering" c 
            JOIN "dish" d 
            ON c.dish_id = c.id 
            WHERE served_at = $3 
            LIMIT 1
        ), new_catering_rating AS (
            INSERT INTO "catering_rating" (rating_id, catering_id)
            SELECT nr.id, pc.id FROM new_rating nr, proper_catering pc
            RETURNING catering_rating.id
        )
        SELECT 
            ncr.id, 
            nr.stars, 
            nr.created_at as "created_at: DateTime<Utc>", 
            pc.served_at as "served_at: DateTime<Utc>", 
            pc.dish_name 
        FROM "new_rating" nr,
        "proper_catering" pc,
        "new_catering_rating" ncr;
        "#,
        user_public_data.id,
        new_rating.stars,
        new_rating.served_at as _
    )
    .fetch_one(&app_state.db_pool)
    .await;


    if let Err(e) = query {
        return ApiResult::Sqlx(e);
    }

    let inserted_rating = query.unwrap();
    let mut result = vec![];
    result.push(inserted_rating);

    return ApiResult::Ok(Json(RatingsDto::Catering(result)));
}

#[tokio::test]
async fn test_post_catering_rating() {
    let ratings_data = PostRatingReq::Catering(PostCateringRatingReq {
        stars: 4,
        served_at: Result::expect(DateTime::from_str("2020-01-01T00:00:00Z"), "wrong time"),
    });

    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    // TODO: add authorization
    let res = server
        .post("/rating/catering")
        .content_type("application/json")
        .json(&json!(ratings_data))
        .await;

    res.assert_status_unauthorized();
}
