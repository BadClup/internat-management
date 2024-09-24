use std::{str::FromStr, vec};

use axum::{Extension, Json};
#[allow(unused)]
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    routes::ratings::types::{MealDto, MealRatingDto, MealSubratingDto},
    AppState,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRatingReq {
    pub date: Option<DateTime<Utc>>,
}

async fn query_ratings(app_state: &AppState) -> Result<Vec<MealDto>, sqlx::Error> {
    sqlx::query_as!(
        MealDto,
        r#"
            SELECT 
                m.id, 
                m.served_at as "served_at: DateTime<Utc>", 
                d.dish_name, 
                get_meal_ratings(m.id) as "ratings: sqlx::types::Json<Vec<MealRatingDto>>"
            FROM 
                "meal" m 
            JOIN "dish" d 
                ON m.dish_id = d.id
        "#,
    )
    .fetch_all(&app_state.db_pool)
    .await
}

async fn query_ratings_with_date(
    app_state: &AppState,
    date: DateTime<Utc>,
) -> Result<Vec<MealDto>, sqlx::Error> {
    sqlx::query_as!(
        MealDto,
        r#"
            SELECT 
                m.id, 
                m.served_at as "served_at: DateTime<Utc>", 
                d.dish_name, 
                get_meal_ratings(m.id) as "ratings: sqlx::types::Json<Vec<MealRatingDto>>"
            FROM 
                "meal" m 
            JOIN "dish" d 
                ON m.dish_id = d.id
            WHERE served_at = $1
        "#,
        date as _
    )
    .fetch_all(&app_state.db_pool)
    .await
}

pub async fn get_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    Json(rating_options): Json<GetRatingReq>,
) -> ApiResult<'a, Json<Vec<MealDto>>> {
    let ratings: Result<Vec<MealDto>, sqlx::Error> = match rating_options.date {
        Some(option) => query_ratings_with_date(&app_state, option).await,
        None => query_ratings(&app_state).await,
    };

    let ratings = match ratings {
        Ok(ratings) => ratings,
        Err(_) => {
            return ApiResult::Internal("Internal database error".to_string());
        }
    };

    return ApiResult::Ok(Json(ratings));
}

#[allow(unused)]
fn check_first_res(response: Vec<MealDto>) {
    let expected_in_date = vec![MealDto {
        id: 2,
        served_at: Result::expect(DateTime::from_str("2020-01-02T00:00:00Z"), "wrong datetime"),
        dish_name: "kurczak z kurczakiem".to_string(),
        ratings: Option::Some(sqlx::types::Json(vec![MealRatingDto {
            id: 2,
            points: 0,
            subratings: Option::Some(sqlx::types::Json(vec![
                MealSubratingDto {
                    description: Option::Some("TRAGEDIA".to_string()),
                    id: 4,
                    points: 0,
                    question: "Smakowało?".to_string(),
                },
                MealSubratingDto {
                    description: Option::None,
                    id: 5,
                    points: 0,
                    question: "Długo trzeba było czekać?".to_string(),
                },
            ])),
        }])),
    }];

    assert_eq!(json!(response[0..1]), json!(expected_in_date));
}

#[sqlx::test(fixtures(path = "../../../../db_docker", scripts("schema.sql", "seed.sql")))]
async fn test_get_catering_ratings(pool: PgPool) {
    let app_state = AppState { db_pool: pool };
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    let res = server
        .get("/ratings/meals")
        .content_type("application/json")
        .json(&json!({
            "date": "2020-01-02T00:00:00Z"
        }))
        .await;

    res.assert_status_ok();

    let json_res = res.json::<Vec<MealDto>>();

    check_first_res(json_res);

    let res = server
        .get("/ratings/meals")
        .content_type("application/json")
        .json(&json!({}))
        .await;

    res.assert_status_ok();

    let json_response: serde_json::Value = res.json();

    assert!(json_response.is_array());
    assert!(json_response.as_array().unwrap().len() > 0);
}
