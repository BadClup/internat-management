use axum::Json;
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use crate::error::ApiResult;
use crate::routes::user::auth::{get_user_from_header, UserPublicData};

pub async fn get_user_info<'a>(
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
) -> ApiResult<'a, Json<UserPublicData>> {
    let user_public_data = get_user_from_header(bearer_token);
    
    match user_public_data {
        Ok(val) => ApiResult::Ok(Json(val)),
        Err(err) => err,
    }
}