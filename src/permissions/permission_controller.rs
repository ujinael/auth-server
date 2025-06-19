use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use uuid::Uuid;

use crate::core_utils::{AppError, AppJson, AppState};

use super::{Permission, PermissionCheckDto, PermissionService};

pub struct PermissionController {}
impl PermissionController {
    pub fn new(state: AppState) -> Router {
        Router::new()
            .route("/check-permissions/:id", post(Self::check_permissions))
            .with_state(state)
    }
    async fn check_permissions(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        AppJson(checked_permissions): AppJson<Vec<Permission>>,
    ) -> Result<AppJson<PermissionCheckDto>, AppError> {
        let result =
            PermissionService::check_permissions(state.pool.clone(), id, checked_permissions)
                .await?;
        Ok(AppJson(result))
    }
}
