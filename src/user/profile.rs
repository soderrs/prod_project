use std::env;

use axum::{http::StatusCode, Extension, Json};
use sqlx::SqlitePool;

use crate::business::auth::hash_password;

use super::{PatchUser, User, UserProfile};

pub async fn get_profile(
    Extension(user): Extension<User>,
) -> Result<Json<UserProfile>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_profile: UserProfile = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE email = ?
        "#,
    )
    .bind(user.email)
    .fetch_one(&pool)
    .await
    .unwrap();

    Ok(Json(user_profile))
}

pub async fn edit_profile(
    Extension(user): Extension<User>,
    Json(patch_user): Json<PatchUser>,
) -> Result<Json<UserProfile>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if !patch_user.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query(
        r#"
        UPDATE users
        SET name = ?, surname = ?, avatar_url = ?, password_hash = ?
        WHERE email = ?
        "#,
    )
    .bind(patch_user.name.unwrap_or(user.name))
    .bind(patch_user.surname.unwrap_or(user.surname))
    .bind(patch_user.avatar_url)
    .bind(if let Some(ref password) = patch_user.password {
        hash_password(password).unwrap()
    } else {
        user.password_hash
    })
    .bind(&user.email)
    .execute(&pool)
    .await
    .unwrap();

    let user_profile: UserProfile = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE email = ?
        "#,
    )
    .bind(user.email)
    .fetch_one(&pool)
    .await
    .unwrap();

    Ok(Json(user_profile))
}
