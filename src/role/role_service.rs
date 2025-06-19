use uuid::Uuid;

use crate::{core_utils::AppError, db::DbPool};

use super::{role_dto::DetailRoleDto, ListRoleDto};
use crate::permissions::Permission;
pub struct RoleService;

impl RoleService {
    pub async fn get_all_roles(conn: &DbPool) -> Result<Vec<ListRoleDto>, AppError> {
        let res = sqlx::query_as!(
            ListRoleDto,
            r#"
            SELECT id,title,description
            FROM "roles"
            "#
        )
        .fetch_all(&*conn)
        .await?;
        Ok(res)
    }
    pub async fn get_role_by_id(conn: &DbPool, id: Uuid) -> Result<DetailRoleDto, AppError> {
        let res = sqlx::query_as!(
            DetailRoleDto,
            r#"
        SELECT id,
        title,
        description,
        permissions as "permissions!:sqlx::types::Json<Vec<Permission>>"
        FROM "roles"
        WHERE id = $1
        "#,
            id
        )
        .fetch_one(conn)
        .await?;
        Ok(res)
    }
}
