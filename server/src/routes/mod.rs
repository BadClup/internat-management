use axum::Router;

mod user;
mod rating;

pub fn get_router() -> Router {
    Router::new()
        .nest("/user", user::get_router())
        .nest("/rating", rating::get_router())
}
