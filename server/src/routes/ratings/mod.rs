use axum::{routing::get, Router};

mod meals;
mod types;

pub fn get_router() -> Router {
    Router::new().route(
        "/meals",
        get(meals::get::get_meal_ratings)
            .post(meals::post::post_meal_rating)
            .delete(meals::delete::delete_meals_ratings),
    )
}
