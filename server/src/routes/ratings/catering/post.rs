/*use std::str::FromStr;

use axum::{http::StatusCode, Extension, Json};
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routes::ratings::types::CateringRatingDto;
use crate::routes::user::auth::{get_user_from_header, UserRole};
use crate::{error::ApiResult, AppState};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCateringRatingReq {
    pub stars: i32,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
}

pub async fn post_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(new_rating): Json<PostCateringRatingReq>,
) -> ApiResult<'a, Json<Vec<CateringRatingDto>>> {
    let user_public_data;

    match get_user_from_header(bearer_token) {
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
        return ApiResult::Internal(e.to_string());
    }

    let inserted_rating = query.unwrap();
    let mut result = vec![];
    result.push(inserted_rating);

    return ApiResult::Ok(Json(result));
}

#[tokio::test]
async fn test_post_catering_rating() {
    let ratings_data = PostCateringRatingReq {
        stars: 4,
        served_at: Result::expect(DateTime::from_str("2020-01-01T00:00:00Z"), "wrong time"),
    };

    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    // TODO: add authorization
    let res = server
        .post("/ratings/meals")
        .content_type("application/json")
        .json(&json!(ratings_data))
        .await;

    // TODO: figure out why this is 400 not 401
    res.assert_status_bad_request();
}
*/