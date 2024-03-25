use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ApiResult<T = Response> {
    Ok(T),
    
    Unauthorized,
    Forbidden,
    NotFound,
    Sqlx(sqlx::Error),
    Anyhow(anyhow::Error),
}

impl <T> ApiResult<T> {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiResult::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiResult::Forbidden => StatusCode::FORBIDDEN,
            ApiResult::NotFound => StatusCode::NOT_FOUND,
            ApiResult::Sqlx(_) | ApiResult::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiResult::Ok(_) => StatusCode::OK,
        }
    }
}

impl <T> IntoResponse for ApiResult<T> where T: IntoResponse {
    fn into_response(self) -> Response {
        if let ApiResult::Ok(v) = self {
            return v.into_response();
        }
        self.status_code().into_response()
    }
}