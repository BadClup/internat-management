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
        .route("/", post(crud::send_message_controller))
}