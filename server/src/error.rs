use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ApiResult<'a, T = Response> {
    Ok(T),
    
    Unauthorized,
    Forbidden,
    NotFound,
    Sqlx(sqlx::Error),
    Anyhow(anyhow::Error),
    Unknown(String),
    Custom(&'a str, StatusCode),
}

impl <T> ApiResult<'_, T> {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiResult::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiResult::Forbidden => StatusCode::FORBIDDEN,
            ApiResult::NotFound => StatusCode::NOT_FOUND,
            ApiResult::Sqlx(_) | ApiResult::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiResult::Ok(_) => StatusCode::OK,
            ApiResult::Custom(_, status_code) => *status_code,
            ApiResult::Unknown(content) => {
                eprintln!("Unknown error: {:?}", content);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl <T> IntoResponse for ApiResult<'_, T> where T: IntoResponse {
    fn into_response(self) -> Response {
        if let ApiResult::Ok(v) = self {
            return v.into_response();
        }
        
        match &self {
            ApiResult::Sqlx(err) => {
                eprintln!("Sqlx error: {:?}", err);
            },
            ApiResult::Anyhow(err) => {
                eprintln!("Anyhow error: {:?}", err);
            },
            ApiResult::Custom(msg, status_code) => {
                eprintln!("Custom error: {:?} - {:?}", status_code, msg);
            },
            _ => {},
        }
        
        if let ApiResult::Custom(message, status_code) = self {
            return (status_code, message.to_string()).into_response();
        }
        
        self.status_code().into_response()
    }
}