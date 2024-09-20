#[allow(unused)]
use std::str::FromStr;

#[allow(unused)]
use axum::http::{HeaderName, HeaderValue};
use axum::{http::StatusCode, Extension, Json};
#[allow(unused)]
use axum_test::TestServer;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::json;
use sqlx::postgres::PgQueryResult;

#[allow(unused)]
use crate::routes::ratings::types::{
    MealRatingDto, MealSubratingDto, TestMealRating, TestMealSubrating,
};
use crate::routes::user::auth::{get_user_from_header, UserRole};
use crate::{error::ApiResult, AppState};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;

#[derive(Serialize, Deserialize)]
pub struct DeleteRatingReq {
    ratings: Vec<i32>,
    subratings: Vec<i32>,
}

async fn query_ratings<'a>(
    app_state: &AppState,
    rating_ids: Vec<i32>,
    subrating_ids: Vec<i32>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            WITH deleted_rating as (
                DELETE FROM meal_rating
                WHERE id = ANY ($1::int[])
                RETURNING id
            ),
            subratings_to_delete as (
                SELECT id FROM deleted_rating
                UNION (
                    SELECT id FROM unnest(cast($2 as int[])) as id
                )
            )
            DELETE FROM meal_rating_part
            WHERE id = ANY (SELECT id FROM subratings_to_delete)
        "#,
        rating_ids as _,
        subrating_ids as _
    )
    .execute(&app_state.db_pool)
    .await
}

pub async fn delete_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(to_delete): Json<DeleteRatingReq>,
) -> ApiResult<'a, ()> {
    let user_public_data;

    // TODO: make a function for it
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

    let rating = query_ratings(&app_state, to_delete.ratings, to_delete.subratings).await;

    match rating {
        Ok(rating) => rating,
        Err(e) => {
            return ApiResult::Internal(e.to_string());
        }
    };

    return ApiResult::Ok(());
}

// TODO: setup sqlx database tester
#[tokio::test]
async fn test_delete_catering_rating() {
    let ratings_data = DeleteRatingReq {
        ratings: vec![1],
        subratings: vec![],
    };

    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    let bearer_token = crate::utils::tests::login_returning_bearer_token().await;

    let res = server
        .delete("/ratings/meals")
        .content_type("application/json")
        .json(&json!(ratings_data))
        .add_header(
            HeaderName::from_str("Authorization").unwrap(),
            HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap(),
        )
        .await;

    res.assert_status_ok();
}
