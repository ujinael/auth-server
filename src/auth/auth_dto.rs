use axum::{response::IntoResponse, Json};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::DetailUserDto;
use validator::Validate;
static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SignInRequestDto {
    #[validate(length(min = 3, max = 16), regex = "USERNAME_REGEX")]
    pub login: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    //Todo make obligate
    pub user: Option<DetailUserDto>,
}

impl IntoResponse for SignInResponseDto {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCheckDto {
    pub user_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDto {
    pub access_token: String,
    pub refresh_token: Option<String>,
}
