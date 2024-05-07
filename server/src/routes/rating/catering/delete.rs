
use axum::{http::HeaderMap, Extension, Json};

use super::super::rating::{RatingsDto};
use crate::{error::ApiResult, AppState};

pub async fn delete_catering_rating<'a>(
    Extension(_app_state): Extension<AppState>,
    _header: HeaderMap,
    _id_to_delete: i32,
) -> ApiResult<'a, Json<RatingsDto>> {

    todo!()
}
