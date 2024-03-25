use axum::Router;

mod change_location;
mod auth;

pub fn get_router() -> Router {
    Router::new()
}