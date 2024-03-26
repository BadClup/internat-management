use axum::Router;

mod user;

pub fn get_router() -> Router {
    Router::new()
        .nest("/user", user::get_router())
}
