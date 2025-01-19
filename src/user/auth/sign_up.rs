use super::CreateUser;
use crate::business::auth::hash_password;
use axum::{http::StatusCode, Json};
use sqlx::SqlitePool;
use std::env;

pub async fn sign_up(Json(create_user): Json<CreateUser>) -> Result<(), StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if !create_user.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query(
        r#"
        INSERT INTO users VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(create_user.name)
    .bind(create_user.surname)
    .bind(create_user.email)
    .bind(create_user.avatar_url)
    .bind(create_user.other)
    .bind(hash_password(&create_user.password).unwrap())
    .execute(&pool)
    .await
    .unwrap();

    Ok(())
}
