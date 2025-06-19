use std::sync::Arc;

use super::{
    ChangePasswordDto, DetailUserDto, ListUserDto, RegisterUserDto, UpdateUserDto, Updated,
};
use crate::core_utils::{AppError, AppState};
use crate::db::DbPool;
use crate::role::{ListRoleDto, RoleService};
use sqlx::types::Json;
use uuid::Uuid;
pub struct UserService;
use crate::permissions::Permission;
impl UserService {
    pub async fn create_user(
        conn: Arc<DbPool>,
        payload: RegisterUserDto,
    ) -> Result<Updated, AppError> {
        let password = payload.password;
        let hashed_password = crate::auth::auth_utils::hash(password)
            .await
            .map_err(|_| AppError::AnyResponsableError("hashing password error".to_string()))?;
        let role = RoleService::get_role_by_id(&conn, payload.role_id).await?;
        let created_user = sqlx::query_as!(
            // language=PostgreSQL
            Updated,
            r#"
                INSERT INTO "users"(
                login,
                password_hash,
                role_id,
                permissions,
                is_active,
                data
                )
                values ($1, $2,$3,$4,$5,$6)
                RETURNING
                id
            "#,
            payload.login,
            hashed_password,
            payload.role_id,
            role.permissions as Json<Vec<Permission>>,
            payload.is_active,
            payload.data
        )
        .fetch_one(&*conn)
        .await?;
        Ok(created_user)
    }
    pub async fn update_user(
        state: &AppState,
        id: Uuid,
        payload: UpdateUserDto,
    ) -> Result<Updated, AppError> {
        let presented_user = Self::get_one_user(&state.pool, payload.id).await?;
        let mut json_permissions: Option<Json<Vec<Permission>>> = Some(presented_user.permissions);
        let old_role = RoleService::get_role_by_id(&state.pool, presented_user.role.id).await?;
        let role_pretender = match payload.role_id {
            Some(role_id) => Some(RoleService::get_role_by_id(&state.pool, role_id).await?),
            None => None,
        };
        let reference_permissions = match role_pretender {
            Some(role) => role.permissions,
            None => old_role.permissions,
        };

        if let Some(permissions) = payload.permissions {
            if is_subset(&permissions, &reference_permissions) {
                json_permissions = Some(Json(permissions));
            }
        } else if payload.role_id.is_some() {
            json_permissions = Some(reference_permissions);
        }
        let is_active = payload.is_active.unwrap_or(presented_user.is_active);
        let data = match payload.data {
            Some(data) => Some(data),
            None => presented_user.data,
        };
        let updated_user = sqlx::query_as!(
            // language=PostgreSQL
            Updated,
            r#"
                UPDATE "users" SET(
                login,
                role_id,
                permissions,
                is_active,
                data
                ) = ($1, $2,$3,$4,$5)
                WHERE id = $6
                RETURNING id
            "#,
            payload.login,
            payload.role_id,
            json_permissions as Option<Json<Vec<Permission>>>,
            is_active,
            data,
            id
        )
        .fetch_one(&*state.pool)
        .await?;
        Ok(updated_user)
    }
    pub async fn change_user_password(
        state: &AppState,
        id: Uuid,
        payload: ChangePasswordDto,
    ) -> Result<Updated, AppError> {
        let user = sqlx::query!(
            "
            SELECT password_hash FROM users WHERE id = $1
            ",
            id
        )
        .fetch_one(&*state.pool)
        .await?;
        let old_hash = user.password_hash;
        if old_hash.is_empty() {
            return Err(AppError::UnauthorizedError);
        }
        let check_password = crate::auth::auth_utils::verify(payload.old_password, old_hash)
            .await
            .map_err(|_| AppError::UnauthorizedError)?;
        if check_password == false {
            return Err(AppError::UnauthorizedError);
        }
        let hashed_password = crate::auth::auth_utils::hash(payload.new_password)
            .await
            .map_err(|_| AppError::AnyResponsableError("hashing password error".to_string()))?;

        let updated_user = sqlx::query_as!(
            // language=PostgreSQL
            Updated,
            r#"
                UPDATE "users" SET password_hash = $1

                WHERE id = $2
                RETURNING
                id
            "#,
            hashed_password,
            id
        )
        .fetch_one(&*state.pool)
        .await?;
        Ok(updated_user)
    }
    pub async fn delete_user(state: &AppState, id: Uuid) -> Result<Updated, AppError> {
        let res = sqlx::query_as!(
            Updated,
            r#"
                DELETE FROM users
                WHERE id = $1
                RETURNING
                id
            "#,
            id
        )
        .fetch_one(&*state.pool)
        .await?;
        Ok(res)
    }

    pub async fn get_all_users(conn: Arc<DbPool>) -> Result<Vec<ListUserDto>, AppError> {
        let res = sqlx::query!(
            r#"
            SELECT
            users.id as user_id,
            users.login,
            users.role_id,
            users.is_active,
            users.permissions as "permissions!:Json<Vec<Permission>>",
            data,
            roles.id,
            roles.title ,
            roles.description
            FROM "users"
            LEFT JOIN "roles" ON users.role_id = roles.id
            "#
        )
        .fetch_all(&*conn)
        .await?;
        let users_list: Vec<ListUserDto> = res
            .into_iter()
            .map(|row| -> ListUserDto {
                return ListUserDto {
                    id: row.user_id.unwrap(),
                    login: row.login.unwrap(),
                    is_active: row.is_active.unwrap(),
                    data: row.data,
                    role: Some(ListRoleDto {
                        id: row.role_id.unwrap(),
                        title: row.title,
                        description: row.description,
                    }),
                };
            })
            .collect();
        Ok(users_list)
    }
    pub async fn get_user_by_login(
        conn: &DbPool,
        login: String,
    ) -> Result<DetailUserDto, AppError> {
        let res = sqlx::query_as_unchecked!(
            DetailUserDto,
            r#"
            SELECT
            u.id,
            u.login,
            u.is_active,
            u.permissions as "permissions!:Json<Vec<Permission>>",
            JSONB_BUILD_OBJECT(
            'id',r.id,
            'title', r.title,
            'description', r.description) as "role!:Json<ListRoleDto>",
            data,
            password_hash
            FROM "users" u
            JOIN roles r ON u.role_id = r.id
            WHERE login = $1
            "#,
            login
        )
        .fetch_one(&*conn)
        .await?;
        Ok(res)
    }

    pub async fn get_one_user(conn: &DbPool, user_id: Uuid) -> Result<DetailUserDto, AppError> {
        let mut res = sqlx::query_as_unchecked!(
            DetailUserDto,
            r#"
            SELECT
            u.id,
            u.login,
            u.is_active,
            u.permissions as "permissions!:Json<Vec<Permission>>",
            JSONB_BUILD_OBJECT(
            'id',r.id,
            'title', r.title,
            'description', r.description) as "role!:Json<ListRoleDto>",
            data,
            password_hash
            FROM "users" u
            JOIN roles r ON u.role_id = r.id
            WHERE u.id = $1
            "#,
            user_id
        )
        .fetch_one(&*conn)
        .await?;

        res.password_hash = None;
        Ok(res)
    }
}
fn is_subset(first: &[Permission], second: &[Permission]) -> bool {
    let second_set: std::collections::HashSet<_> = second.iter().cloned().collect();
    first.iter().all(|item| second_set.contains(item))
}
