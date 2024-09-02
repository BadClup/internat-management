use std::{str::FromStr, vec};

use axum::{Extension, Json};
use axum_test::TestServer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{error::ApiResult, routes::ratings::types::{MealDto, MealRatingDto, MealSubratingDto}, AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRatingReq {
    pub date: Option<DateTime<Utc>>,
}

pub async fn get_catering_rating<'a>(
    Extension(app_state): Extension<AppState>,
    Json(rating_options): Json<GetRatingReq>,
) -> ApiResult<'a, Json<Vec<MealDto>>> {
    let ratings: Result<Vec<MealDto>, sqlx::Error> = match rating_options.date {
        Some(option) => sqlx::query_as!(MealDto, 
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
        option as _
        )
        .fetch_all(&app_state.db_pool)
        .await,

        None => sqlx::query_as!(MealDto, r#"
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
         .await,
    };

    let ratings = match ratings {
        Ok(ratings) => ratings,
        Err(_) => {
            return ApiResult::Internal("Internal database error".to_string());
        }
    };

    return ApiResult::Ok(Json(ratings));
}

#[tokio::test]
async fn test_get_catering_ratings() {
    let app_state = AppState::new().await;
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

    let expected_in_date= vec![
        MealDto {
            id: 2,
            served_at: Result::expect(DateTime::from_str("2020-01-02T00:00:00Z"), "wrong datetime"),
            dish_name: "kurczak z kurczakiem".to_string(),
            ratings: Option::Some(sqlx::types::Json(vec![
                MealRatingDto {
                id: 2,
                points: 0,
                //created_at: Result::expect(
                //    DateTime::from_str("2020-01-05T00:00:00Z"),
                //    "wrong datetime",
                //),
                subratings: Option::Some(sqlx::types::Json(vec![
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
                ]))
                }
            ]))
        }
        ];


    let json_res = res.json::<Vec<MealDto>>();

    assert_eq!(json!(json_res[0..1]), json!(expected_in_date));
    
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
