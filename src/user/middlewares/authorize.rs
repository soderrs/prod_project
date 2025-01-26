use std::env;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{user::User, AppState};

pub async fn authorize_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = match req.headers_mut().get(http::header::AUTHORIZATION) {
        Some(header) => header.to_str().map_err(|_| StatusCode::FORBIDDEN)?,
        None => return Err(StatusCode::FORBIDDEN)?,
    };

    let [_, token, ..] = *auth_header.split_whitespace().collect::<Vec<&str>>() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let token_data = match decode_jwt(token) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let user = match retrieve_user_by_email(&app_state.pool, &token_data.claims.email).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub async fn retrieve_user_by_email(pool: &PgPool, email: &str) -> Option<User> {
    let user = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .unwrap();

    user
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let secret = env::var("RANDOM_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let now = Utc::now();
    let expire: TimeDelta = Duration::hours(24);
    let exp = (now + expire).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claims = Claims { exp, iat, email };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("RANDOM_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    result
}
