use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

use crate::permissions::Permission;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListRoleDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailRoleDto {
    pub id: Uuid,
    pub permissions: Json<Vec<Permission>>,
    pub title: String,
    pub description: String,
}
