use crate::{
    user::{middlewares::authorize::hash_password, CreateUser, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn sign_up(
    State(app_state): State<AppState>,
    Json(create_user): Json<CreateUser>,
) -> Result<Json<String>, StatusCode> {
    if !create_user.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !is_unique_user(&app_state.pool, &create_user).await {
        return Err(StatusCode::CONFLICT);
    }

    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO users VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(create_user.name)
    .bind(create_user.surname)
    .bind(create_user.email)
    .bind(create_user.avatar_url)
    .bind(create_user.other)
    .bind(hash_password(&create_user.password).unwrap())
    .execute(&app_state.pool)
    .await
    .unwrap();

    Ok(Json(id))
}

async fn is_unique_user(pool: &PgPool, sign_up_data: &CreateUser) -> bool {
    let row: Option<User> = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE email = ?
        "#,
    )
    .bind(&sign_up_data.email)
    .fetch_optional(pool)
    .await
    .unwrap();

    row.is_none()
}
