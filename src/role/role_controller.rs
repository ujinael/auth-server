use super::role_dto::DetailRoleDto;
use super::ListRoleDto;
use super::RoleService;
use crate::core_utils::{AppError, AppJson, AppState};
use axum::extract::Path;
use axum::extract::State;
use axum::{routing::get, Router};
use uuid::Uuid;

pub struct RoleController {}

impl RoleController {
    pub fn new(state: AppState) -> Router {
        Router::new()
            .route("/", get(Self::get_all_roles))
            .route("/:id", get(Self::get_one_role))
            .with_state(state)
    }

    async fn get_all_roles(
        State(state): State<AppState>,
    ) -> Result<AppJson<Vec<ListRoleDto>>, AppError> {
        let roles_dto = RoleService::get_all_roles(&state.pool).await?;
        Ok(AppJson(roles_dto))
    }
    async fn get_one_role(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<AppJson<DetailRoleDto>, AppError> {
        let roles_dto = RoleService::get_role_by_id(&state.pool, id).await?;
        Ok(AppJson(roles_dto))
    }
}
