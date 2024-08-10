use axum::Router;
use axum::routing::{get, post};

mod ws;
mod crud;
mod types;

pub use ws::*;
pub use types::*;

pub fn get_router() -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/:recipient_id", get(crud::get_messages_controller))
        .route("/conversations", get(crud::get_conversations_controller))
        .route("/", post(crud::send_message_controller))
}
