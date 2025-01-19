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
    sqlx::query(
        r#"
        INSERT INTO promos
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(create_promo.description)
    .bind(create_promo.image_url)
    .bind(sqlx::types::Json(create_promo.target))
    .bind(create_promo.max_count)
    .bind(create_promo.active_from)
    .bind(create_promo.active_until)
    .bind(create_promo.mode)
    .bind(create_promo.promo_common)
    .bind(create_promo.promo_unique)
    .bind(&id)
    .bind(company.id)
    .bind(company.name)
    .bind(0)
    .bind(0)
    .bind(false)
    .execute(&pool)
    .await
    .unwrap();

    Ok((StatusCode::CREATED, Json(id)))
}
