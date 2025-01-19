use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use std::env;

pub mod sign_in;
pub mod sign_up;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

#[derive(FromRow, Clone, Debug)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn retrieve_user_by_email(email: &str) -> Option<Company> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let company = sqlx::query_as(
        r#"
        SELECT * FROM companies WHERE email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(&pool)
    .await
    .unwrap();

    company
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
