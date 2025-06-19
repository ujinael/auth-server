use std::io::Error;

// #[cfg(test)]
use sqlx::{PgPool, Row};

use crate::role::RoleService;
#[sqlx::test]
async fn role_service_test(pool: PgPool) -> sqlx::Result<()> {
    let roles = RoleService::get_all_roles(&pool)
        .await
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
    let role = roles.into_iter().find(|r| r.title == "admin").unwrap();

    let role_by_id = RoleService::get_role_by_id(&pool, role.id)
        .await
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;

    assert_eq!(role.title, "admin");
    assert_eq!(role_by_id.description, "админ");

    Ok(())
}
