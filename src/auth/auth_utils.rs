use anyhow::{anyhow, Context};
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use tokio::task;

use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::core_utils::AppError;
use crate::user::DetailUserDto;

use super::Claims;
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
pub struct TtlToken {
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}
pub static TTL: Lazy<TtlToken> = Lazy::new(|| {
    let access_ttl = std::env::var("ACCESS_TOKEN_EXP").expect("ACCESS_TOKEN_EXP must be set");
    let refresh_ttl = std::env::var("REFRESH_TOKEN_EXP").expect("REFRESH_TOKEN_EXP must be set");
    TtlToken {
        access_token_ttl: Duration::minutes(access_ttl.parse().unwrap()),
        refresh_token_ttl: Duration::minutes(refresh_ttl.parse().unwrap()),
    }
});

pub fn generate_token(user: &DetailUserDto, exp: Option<Duration>) -> Result<String, AppError> {
    let exp_offset = exp.unwrap_or(TTL.access_token_ttl);
    let expiration = (chrono::Utc::now() + exp_offset).timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::AnyResponsableError("Token creation error".to_string()))?;
    return Ok(token);
}
pub fn validate_token(token: String) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token.as_str(),
        &KEYS.decoding,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|e| {
        println!("{}", e.to_string());
        AppError::UnauthorizedError
    })?;
    println!("{}", &token_data.claims.sub);
    Ok(token_data.claims)
}
pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("failed to hash password"))?
            .to_string())
    })
    .await
    .context("panic in hash()")?
}

pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow!(e).context("BUG: password hash invalid"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("failed to verify password")),
        }
    })
    .await
    .context("panic in verify()")?
}
