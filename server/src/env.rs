#[allow(non_snake_case)]
pub struct Env {
    pub DATABASE_URL: String,
    pub JWT_SECRET: String,
    pub KAFKA_SERVER_URL: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv::dotenv().ok().expect("Failed to load .env file");

        Self {
            DATABASE_URL: std::env::var("DATABASE_URL")
                .unwrap_or("postgresql://postgres:test@localhost:5432/internat_management".to_string()),
            JWT_SECRET: std::env::var("SECRET")
                .expect("SECRET must be set in .env file"),
            KAFKA_SERVER_URL: std::env::var("KAFKA_SERVER_URL")
                .unwrap_or("localhost:9092".to_string()),
        }
    }
}