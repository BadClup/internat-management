use axum::{routing::get, Router};

mod catering;
mod types;

pub fn get_router() -> Router {
    Router::new().route(
        "/meals",
        get(catering::get::get_catering_rating).post(catering::post::post_catering_rating), //.delete(catering::delete_catering_rating),
    )
}
