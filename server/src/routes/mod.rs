use axum::Router;

pub mod chat;
mod ratings;
mod user;

pub fn get_router() -> Router {
    Router::new()
        .nest("/user", user::get_router())
        .nest("/ratings", ratings::get_router())
        .nest("/chat", chat::get_router())
}
