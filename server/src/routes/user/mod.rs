use axum::Router;
use axum::routing::post;

pub mod auth;

pub fn get_router() -> Router {
    Router::new()
        .route("/register-many", post(auth::register_residents))
        .route("/login", post(auth::login))
}
