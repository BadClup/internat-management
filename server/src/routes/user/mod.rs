use axum::routing::post;
use axum::Router;

mod auth;
mod change_location;

pub fn get_router() -> Router {
    Router::new()
        .route("/register-many", post(auth::register_residents))
        .route("/login", post(auth::login))
}
