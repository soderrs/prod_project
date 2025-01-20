use std::env;

use axum::{http::StatusCode, Extension, Json};
use sqlx::SqlitePool;

use super::{User, UserProfile};

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
