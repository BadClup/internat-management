use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub enum ApiResult<'a, T> {
    Ok(T),

    Unauthorized,
    Forbidden,
    NotFound,
    Internal(String),
    Custom(&'a str, StatusCode),
}

impl<'a, T> From<sqlx::Error> for ApiResult<'a, T> {
    fn from(err: sqlx::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl<'a, T> From<anyhow::Error> for ApiResult<'a, T> {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl<T> ApiResult<'_, T> {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiResult::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiResult::Forbidden => StatusCode::FORBIDDEN,
            ApiResult::NotFound => StatusCode::NOT_FOUND,
            ApiResult::Ok(_) => StatusCode::OK,
            ApiResult::Custom(_, status_code) => *status_code,
            ApiResult::Internal(content) => {
                eprintln!("Unknown error: {:?}", content);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl<T> IntoResponse for ApiResult<'_, T> where T: IntoResponse {
    fn into_response(self) -> Response {
        if let ApiResult::Ok(v) = self {
            return v.into_response();
        }

        match &self {
            ApiResult::Custom(msg, status_code) => {
                eprintln!("Custom error: {:?} - {:?}", status_code, msg);
            }
            _ => {}
        }

        if let ApiResult::Custom(message, status_code) = self {
            return (status_code, message.to_string()).into_response();
        }

        self.status_code().into_response()
    }
}