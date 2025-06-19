use std::sync::Arc;

use uuid::Uuid;

use crate::{core_utils::AppError, db::DbPool, user::UserService};

use super::{Permission, PermissionCheckDto};
pub struct PermissionService {}
impl PermissionService {
    pub async fn check_permissions(
        conn: Arc<DbPool>,
        user_id: Uuid,
        checked_permissions: Vec<Permission>,
    ) -> Result<PermissionCheckDto, AppError> {
        let user_permissions = UserService::get_one_user(&conn, user_id).await?.permissions;
        let is_admin = user_permissions.clone().contains(&Permission::AdminCanRead);
        if is_admin == true {
            return Ok(PermissionCheckDto { check_status: true });
        }
        let all_elements_present = checked_permissions
            .iter()
            .all(|item| user_permissions.contains(item));

        Ok(PermissionCheckDto {
            check_status: all_elements_present,
        })
    }
}
