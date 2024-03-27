use axum::handler::Handler;
use axum::{Extension, Router};

mod routes;
mod error;

const DEFAULT_DATABASE_URL: &str = "postgresql://postgres:test@localhost:5432/internat_management";

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        dotenv::dotenv().ok().expect("Failed to load .env file");

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or(DEFAULT_DATABASE_URL.to_string());

        Self {
            db_pool: sqlx::PgPool::connect(&database_url)
                .await
                .unwrap(),
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
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
