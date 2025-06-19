use axum::{extract::State, routing::post, Router};

use crate::core_utils::{AppError, AppJson, AppState};

use super::{
    auth_dto::{TokenCheckDto, TokenDto},
    AuthService, SignInRequestDto, SignInResponseDto,
};

pub struct AuthController {}
impl AuthController {
    pub fn new(state: AppState) -> Router {
        Router::new()
            .route("/sign-in", post(Self::authentificate))
            .route("/check-auth", post(Self::check_auth))
            .route("/refresh-token", post(Self::refresh_token))
            .with_state(state)
    }
    async fn authentificate(
        State(state): State<AppState>,
        AppJson(sign_in_dto): AppJson<SignInRequestDto>,
    ) -> Result<SignInResponseDto, AppError> {
        let result = AuthService::authentificate(&state, sign_in_dto).await?;
        Ok(result)
    }
    async fn check_auth(
        State(_): State<AppState>,
        AppJson(token_dto): AppJson<TokenDto>,
    ) -> Result<AppJson<TokenCheckDto>, AppError> {
        let result = AuthService::check_auth(token_dto.access_token).await?;
        Ok(AppJson(result))
    }
    async fn refresh_token(
        State(state): State<AppState>,
        AppJson(token_dto): AppJson<TokenDto>,
    ) -> Result<AppJson<SignInResponseDto>, AppError> {
        let result = AuthService::refresh_token(&state, token_dto).await?;
        Ok(AppJson(result))
    }
}
