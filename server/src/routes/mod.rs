use axum::Router;

mod user;
mod rating;
pub mod chat;

pub fn get_router() -> Router {
    Router::new()
        .nest("/user", user::get_router())
        .nest("/rating", rating::get_router())
        .nest("/chat", chat::get_router())
}
