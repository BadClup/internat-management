use axum::{routing::get, Router};

mod catering_rating;
mod rating;

pub fn get_router() -> Router {
    Router::new().route(
        "/:type",
        get(rating::get_rating)
            .post(rating::post_rating)
            .delete(rating::delete_rating),
    )
}
