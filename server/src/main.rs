use axum::{Extension, Router};
use lazy_static::lazy_static;

mod env;
mod error;
mod routes;
mod utils;

lazy_static! {
    pub static ref ENV: env::Env = env::Env::new();
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            db_pool: sqlx::PgPool::connect(&ENV.DATABASE_URL).await.unwrap(),
        }
    }
}

pub fn get_app(app_state: AppState) -> Router {
    Router::new()
        .nest("/", routes::get_router())
        .layer(Extension(app_state))
}

#[tokio::main]
async fn main() {
    let app_state = AppState::new().await;
    let router = get_app(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
