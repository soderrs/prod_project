use crate::business::{auth::Company, promo::CreatePromo};
use axum::{http::StatusCode, Extension, Json};
use sqlx::SqlitePool;
use std::env;
use uuid::Uuid;

pub async fn create_promo(
    Extension(company): Extension<Company>,
    Json(create_promo): Json<CreatePromo>,
) -> Result<(StatusCode, Json<String>), StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if !create_promo.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query (r#"
        INSERT INTO promos
        (id, description, target, max_count, active_from, active_until, mode, promo_common, promo_unique)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
    .bind(&id)
    .bind(create_promo.description)
    .bind(sqlx::types::Json(create_promo.target))
    .bind(create_promo.max_count)
    .bind(create_promo.active_from)
    .bind(create_promo.active_until)
    .bind(create_promo.mode)
    .bind(create_promo.promo_common)
    .bind(create_promo.promo_unique)
    .execute(&pool)
    .await
    .unwrap();

    Ok((StatusCode::CREATED, Json(id)))
}
