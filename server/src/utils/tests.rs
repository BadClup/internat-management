use axum_test::TestServer;
use serde_json::json;
use crate::AppState;

pub async fn login_returning_bearer_token() -> String {
    let app_state = AppState::new().await;
    let app = crate::get_app(app_state.clone());
    let server = TestServer::new(app).expect("Failed to create test server");

    let res = server.post("/user/login")
        .content_type("application/json")
        .json(&json!({
                "username": "bstrama",
                "password": "12345678",
            }))
        .await;

    let body: serde_json::Value = res.json();
    body["bearer_token"].as_str().unwrap().to_string()
}
