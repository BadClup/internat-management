#[allow(unused)]
use std::str::FromStr;

#[allow(unused)]
use axum::http::{HeaderName, HeaderValue};
use axum::{http::StatusCode, Extension, Json};
#[allow(unused)]
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::json;
use sqlx::postgres::PgTypeInfo;
use sqlx::prelude::Type;

#[allow(unused)]
use crate::routes::ratings::types::{
    MealRatingDto, MealSubratingDto, TestMealRating, TestMealSubrating,
};
use crate::routes::user::auth::{get_user_from_header, UserRole};
use crate::{error::ApiResult, AppState};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;

#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name = "meal_rating_part_type")]
pub struct PostSubrating {
    pub question_id: i32,
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PostRatingReq {
    pub points: i32,
    #[serde(with = "DateTime")]
    pub served_at: DateTime<Utc>,
    pub subratings: Vec<PostSubrating>,
}

#[derive(sqlx::Encode)]
struct Subratings<'a>(&'a [PostSubrating]);

impl sqlx::Type<sqlx::Postgres> for Subratings<'_> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("meal_rating_part_type[]")
    }
}

async fn query_ratings<'a>(
    app_state: &AppState,
    user_id: i32,
    points: i32,
    served_at: DateTime<Utc>,
    subratings: Subratings<'a>,
) -> Result<MealRatingDto, sqlx::Error> {
    sqlx::query_as!(MealRatingDto,
        r#"
        WITH 
        proper_meal AS (
            SELECT id FROM "meal" WHERE served_at = $3 LIMIT 1
        ), 
        new_rating AS (
            INSERT INTO "meal_rating" (meal_id, user_id, points)
            SELECT id, $1, $2
            FROM proper_meal
            RETURNING id, created_at, points
        ),
        new_subratings AS (
            INSERT INTO "meal_rating_part" (meal_rating_id, rating_question_id, points, description)
            SELECT proper_meal.id, question_id, points, description FROM unnest(cast($4 as meal_rating_part_type[])), proper_meal
            RETURNING meal_rating_part.id, rating_question_id, points, description
        )
        SELECT 
            nr.id,
            nr.points, 
            COALESCE((
                SELECT json_agg(json_build_object(
                        'id', s.id, 
                        'question', question, 
                        'points', points, 
                        'description', description
                    ))
                FROM new_subratings s 
                JOIN meal_rating_question q 
                    ON s.rating_question_id = q.id
            ), '[]'::json) as "subratings: sqlx::types::Json<Vec<MealSubratingDto>>"
            FROM new_rating nr;
        "#,
        user_id,
        points,
        served_at as _,
        subratings as _
    )
    .fetch_one(&app_state.db_pool)
    .await
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

    let rating = query_ratings(
        &app_state,
        user_public_data.id,
        new_rating.points,
        new_rating.served_at,
        Subratings(&new_rating.subratings),
    )
    .await;

    let rating = match rating {
        Ok(rating) => rating,
        Err(e) => {
            return ApiResult::Internal(e.to_string());
        }
    };

    return ApiResult::Ok(Json(rating));
}

#[tokio::test]
async fn test_post_catering_rating() {
    let ratings_data = PostRatingReq {
        points: 4,
        served_at: Result::expect(DateTime::from_str("2020-01-01T00:00:00Z"), "wrong time"),
        subratings: vec![PostSubrating {
            question_id: 2,
            points: 3,
            description: Option::None,
        }],
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

    let expected_output = TestMealRating {
        points: 4,
        subratings: Some(sqlx::types::Json(vec![TestMealSubrating {
            description: Option::None,
            points: 3,
            question: "Ciepłe?".to_string(),
        }]))
        .into(),
    };

    let json_res = res.json::<TestMealRating>();

    assert_eq!(json!(json_res), json!(expected_output));
}
