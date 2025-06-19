use super::{
    ChangePasswordDto, DetailUserDto, ListUserDto, RegisterUserDto, UpdateUserDto, Updated,
    UserService,
};
use crate::core_utils::{AppError, AppJson, AppState};
use axum::extract::{Path, State};
use axum::routing::delete;
use axum::{
    routing::{get, patch, post, put},
    Router,
};
use uuid::Uuid;
pub struct UserController {}
impl UserController {
    pub fn new(state: AppState) -> Router {
        Router::new()
            .route("/", get(Self::get_all_users))
            .route("/", post(Self::create_user))
            .route("/:id", put(Self::update_user))
            .route("/:id/change-password", patch(Self::change_user_password))
            .route("/:id", delete(Self::delete_user))
            .route("/:id", get(Self::get_one_user))
            .with_state(state)
    }
    async fn get_all_users(
        State(state): State<AppState>,
    ) -> Result<AppJson<Vec<ListUserDto>>, AppError> {
        let conn = state.pool;
        let users_dto = UserService::get_all_users(conn).await?;
        Ok(AppJson(users_dto))
    }
    async fn get_one_user(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<AppJson<DetailUserDto>, AppError> {
        let user = UserService::get_one_user(&state.pool, id).await?;
        Ok(AppJson(user))
    }
    async fn create_user(
        State(state): State<AppState>,
        AppJson(create_user_dto): AppJson<RegisterUserDto>,
    ) -> Result<AppJson<Updated>, AppError> {
        let conn = state.pool;
        let result = UserService::create_user(conn, create_user_dto).await?;
        Ok(AppJson(result))
    }
    async fn update_user(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        AppJson(user_dto): AppJson<UpdateUserDto>,
    ) -> Result<AppJson<Updated>, AppError> {
        let user = UserService::update_user(&state, id, user_dto).await?;
        Ok(AppJson(user))
    }
    async fn change_user_password(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        AppJson(change_password_dto): AppJson<ChangePasswordDto>,
    ) -> Result<AppJson<Updated>, AppError> {
        let user = UserService::change_user_password(&state, id, change_password_dto).await?;
        Ok(AppJson(user))
    }
    async fn delete_user(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<AppJson<Updated>, AppError> {
        let user = UserService::delete_user(&state, id).await?;
        Ok(AppJson(user))
    }
}
