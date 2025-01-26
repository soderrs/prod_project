use super::{Comment, Country};
use crate::{
    business::{auth::Company, promo::CreatePromo},
    AppState,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use chrono::Utc;
use serde_json::json;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, PartialEq)]
struct Id {
    id: String,
}

pub async fn create_promo(
    State(app_state): State<AppState>,
    Extension(company): Extension<Company>,
    Json(create_promo): Json<CreatePromo>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    if !create_promo.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO promos
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(create_promo.description)
    .bind(create_promo.image_url)
    .bind(sqlx::types::Json(create_promo.target))
    .bind(if let Some(max_count) = create_promo.max_count {
        Some(max_count as i32)
    } else {
        None
    })
    .bind(sqlx::types::Json(Utc::now()))
    .bind(create_promo.active_from)
    .bind(create_promo.active_until)
    .bind(create_promo.mode)
    .bind(create_promo.promo_common)
    .bind(create_promo.promo_unique)
    .bind(&id)
    .bind(company.id)
    .bind(company.name)
    .bind(sqlx::types::Json(Vec::<String>::new()))
    .bind(0)
    .bind(false)
    .bind(sqlx::types::Json(Vec::<Country>::new()))
    .bind(sqlx::types::Json(Vec::<Comment>::new()))
    .bind(sqlx::types::Json(Vec::<String>::new()))
    .execute(&app_state.pool)
    .await
    .unwrap();

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": id,
        })),
    ))
}
