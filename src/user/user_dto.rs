use crate::{permissions::Permission, role::ListRoleDto};

use axum::{body::Body, http::Response, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{Json, JsonValue},
};
use uuid::Uuid;

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserDto {
    pub login: String,
    pub password: String,
    pub role_id: Uuid,
    pub is_active: bool,
    pub data: Option<JsonValue>,
}

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDto {
    pub id: Uuid,
    pub login: Option<String>,
    pub role_id: Option<Uuid>,
    pub data: Option<JsonValue>,
    pub permissions: Option<Vec<Permission>>,
    pub is_active: Option<bool>,
}
#[serde_with::serde_as]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordDto {
    pub user_id: Uuid,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseUserDto {
    pub id: Uuid,
    pub login: String,
}

#[derive(serde::Deserialize, Debug, Serialize, FromRow)]
pub struct Updated {
    pub id: Uuid,
}
#[derive(serde::Deserialize, Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]

pub struct ListUserDto {
    pub id: Uuid,
    pub login: String,
    pub is_active: bool,
    pub role: Option<ListRoleDto>,
    pub data: Option<JsonValue>,
}
impl IntoResponse for ListUserDto {
    fn into_response(self) -> Response<Body> {
        axum::Json(self).into_response()
    }
}
#[derive(serde::Deserialize, Debug, Serialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]

pub struct DetailUserDto {
    pub id: Uuid,
    pub login: String,
    pub is_active: bool,
    pub role: Json<ListRoleDto>,
    pub permissions: Json<Vec<Permission>>,
    pub data: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
}
impl IntoResponse for DetailUserDto {
    fn into_response(self) -> Response<Body> {
        axum::Json(self).into_response()
    }
}
