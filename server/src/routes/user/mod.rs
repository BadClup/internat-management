use axum::routing::{get, post};
use axum::Router;

pub mod auth;
mod info;

pub fn get_router() -> Router {
    Router::new()
        .route("/register-many", post(auth::register_residents))
        .route("/login", post(auth::login))
        .route("/info", get(info::get_user_info))
}
