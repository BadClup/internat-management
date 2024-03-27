use axum::Router;
use axum::routing::post;

mod change_location;
mod auth;

pub fn get_router() -> Router {
    Router::new()
        .route("/register-many", post(auth::register_users))
        .route("/login", post(auth::login))
}
