use std::str::FromStr;

use axum::http::{HeaderName, HeaderValue};
use axum::{http::StatusCode, Extension, Json};
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgTypeInfo;
use sqlx::prelude::Type;

use crate::routes::ratings::types::{MealRatingDto, MealSubratingDto};
use crate::routes::user::auth::{get_user_from_header, UserRole};
use crate::{error::ApiResult, AppState};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;

#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name="meal_rating_part_type")]
pub struct PostSubrating {
    pub question_id: i32,
    pub points: i32,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct PostRatingReq {
    pub points: i32,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
    pub subratings: Vec<PostSubrating>
}

#[derive(sqlx::Encode)]
struct Subratings<'a>(&'a [PostSubrating]);

impl sqlx::Type<sqlx::Postgres> for Subratings<'_> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("meal_rating_part_type[]")
    }
}

pub async fn post_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(new_rating): Json<PostRatingReq>,
) -> ApiResult<'a, Json<MealRatingDto>> {

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

    let rating = sqlx::query!(
        r#"
        WITH 
        proper_meal AS (
            SELECT id FROM "meal" WHERE served_at = $3 LIMIT 1
        ), 
        new_rating AS (
            INSERT INTO "meal_rating" (meal_id, user_id, points)
            SELECT id, $1, $2
            FROM proper_meal
            RETURNING id, user_id, created_at, points
        ),
        new_subratings AS (
            INSERT INTO "meal_rating_part" (meal_rating_id, rating_question_id, points, description)
            SELECT proper_meal.id, question_id, points, description FROM unnest(cast($4 as meal_rating_part_type[])), proper_meal
        )
        -- TODO: return whole object MealRating with subratings
        SELECT created_at FROM new_rating;
        "#,
        user_public_data.id,
        new_rating.points,
        new_rating.served_at as _,
        Subratings(&new_rating.subratings) as _
    )
    .fetch_one(&app_state.db_pool)
    .await;

    let rating= match rating {
        Ok(rating) => rating,
        Err(_) => {
            return ApiResult::Internal("Internal database error".to_string());
        }
    };

    let mut result = vec![];
    result.push(rating);

    return ApiResult::Ok(Json(
                MealRatingDto {
                id: 2,
                points: 0,
                //created_at: Result::expect(
                //    DateTime::from_str("2020-01-05T00:00:00Z"),
                //    "wrong datetime",
                //),
                subratings: vec![
                    MealSubratingDto {
                        description: Option::Some("TRAGEDIA".to_string()),
                        id: 4,
                        points: 0,
                        question: "Smakowało?".to_string()
                    },
                    MealSubratingDto {
                        description: Option::None,
                        id: 5,
                        points: 0,
                        question: "Długo trzeba było czekać?".to_string()
                    },
                ]
                }
    ));
}

#[tokio::test]
async fn test_post_catering_rating() {
    let ratings_data = PostRatingReq {
        points: 4,
        served_at: Result::expect(DateTime::from_str("2020-01-01T00:00:00Z"), "wrong time"),
        subratings: vec![
            PostSubrating {
                question_id: 1,
                points: 3,
                description: Option::None
            }
        ]
    };

    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    let bearer_token = crate::utils::tests::login_returning_bearer_token().await;

    let res = server
        .post("/ratings/meals")
        .content_type("application/json")
        .json(&json!(ratings_data))
        .add_header(
            HeaderName::from_str("Authorization").unwrap(),
            HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap(),
        )
        .await;

    res.assert_status_ok();
}
