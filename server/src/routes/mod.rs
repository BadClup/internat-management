use axum::Router;

mod user;
pub mod chat;

pub fn get_router() -> Router {
    Router::new()
        .nest("/user", user::get_router())
        .nest("/chat", chat::get_router())
}
