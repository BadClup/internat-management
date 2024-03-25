use axum::Router;

mod user;

pub fn get_router() -> Router {
    Router::new()
        .route_service("/user", user::get_router())
}