use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    core_utils::{AppError, AppState},
    user::UserService,
};

use super::{
    auth_dto::{TokenCheckDto, TokenDto},
    auth_utils::TTL,
    SignInRequestDto, SignInResponseDto,
};

pub struct AuthService {}
impl AuthService {
    pub async fn authentificate(
        state: &AppState,
        payload: SignInRequestDto,
    ) -> Result<SignInResponseDto, AppError> {
        let user = UserService::get_user_by_login(&state.pool, payload.login).await?;
        let hash = user.password_hash.clone().unwrap();
        let check_password = super::auth_utils::verify(payload.password, hash)
            .await
            .map_err(|_| AppError::UnauthorizedError)?;
        if check_password == false {
            return Err(AppError::UnauthorizedError);
        }
        let mut detail = user.clone();
        detail.password_hash = None;
        let access_token = super::auth_utils::generate_token(&detail, None)?;
        let refresh_token =
            super::auth_utils::generate_token(&detail, Some(TTL.refresh_token_ttl))?;

        let mut conn = state.redis_pool.get().await.map_err(|_| {
            AppError::AnyResponsableError("error when set refresh_token".to_string())
        })?;
        let ttl: u64 = TTL.refresh_token_ttl.num_seconds() as u64; //represent of 1 day of storing refresh_token
        let token_key = format!("r_token:{}", user.id.to_string().as_str());
        conn.set_ex(token_key, refresh_token.clone(), ttl)
            .await
            .map_err(|_| {
                AppError::AnyResponsableError("error when set refresh_token".to_string())
            })?;
        let result = SignInResponseDto {
            access_token,
            refresh_token,
            user: Some(detail),
        };
        Ok(result)
    }
    pub async fn check_auth(access_token: String) -> Result<TokenCheckDto, AppError> {
        let claims = super::auth_utils::validate_token(access_token)?;
        let user_id = Uuid::parse_str(claims.sub.as_str())
            .map_err(|_| AppError::AnyResponsableError("wrong format of user_id".to_string()))?;
        Ok(TokenCheckDto { user_id })
    }
    pub async fn refresh_token(
        state: &AppState,
        token_dto: TokenDto,
    ) -> Result<SignInResponseDto, AppError> {
        let old_r_token = match token_dto.refresh_token {
            Some(token) => token,
            None => {
                return Err(AppError::UnauthorizedError);
            }
        };
        let claims = super::auth_utils::validate_token(old_r_token.clone())?;

        let user_id = Uuid::parse_str(claims.sub.as_str())
            .map_err(|_| AppError::AnyResponsableError("wrong format of user_id".to_string()))?;
        let token_key = format!("r_token:{}", user_id.clone().to_string().as_str());

        let mut conn = state.redis_pool.get().await.unwrap();
        let stored_token: Option<String> = conn.get(token_key.clone()).await.unwrap();

        if stored_token != Some(old_r_token) {
            return Err(AppError::UnauthorizedError);
        }
        let user = UserService::get_one_user(&state.pool, user_id).await?;

        let access_token = super::auth_utils::generate_token(&user, None)?;
        let refresh_token = super::auth_utils::generate_token(&user, Some(TTL.refresh_token_ttl))?;
        let ttl: u64 = TTL.refresh_token_ttl.num_seconds() as u64; //represent of 1 day of storing refresh_token

        let _ = conn
            .set_ex(token_key, refresh_token.clone(), ttl)
            .await
            .map_err(|_| {
                AppError::AnyResponsableError("error when set refresh_token".to_string())
            })?;

        let result = SignInResponseDto {
            access_token,
            refresh_token,
            user: None,
        };
        Ok(result)
    }
}
