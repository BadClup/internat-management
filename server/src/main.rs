use axum::handler::Handler;
use axum::{Extension, Router};

mod routes;
mod error;

const DEFAULT_DATABASE_URL: &str = "postgresql://postgres:test@localhost:5432/internat_management";

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok().expect("Failed to load .env file");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or(DEFAULT_DATABASE_URL.to_string());

    let app_state = AppState {
        db_pool: sqlx::PgPool::connect(&database_url)
            .await
            .unwrap(),
    };

    let router = Router::new()
        .nest("/", routes::get_router())
        .layer(Extension(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
