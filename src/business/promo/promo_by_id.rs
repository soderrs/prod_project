use std::env;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::SqlitePool;

use crate::business::auth::Company;

use super::{PatchPromo, Promo, PromoReadOnly};

pub async fn get_promo(
    Extension(company): Extension<Company>,
    Path(id): Path<String>,
) -> Result<Json<PromoReadOnly>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let promo: Option<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE promo_id = ? AND company_id = ?
        "#,
    )
    .bind(id)
    .bind(&company.id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    if promo.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let promo = promo.unwrap();

    Ok(Json(PromoReadOnly {
        description: promo.description,
        image_url: promo.image_url,
        target: promo.target,
        max_count: promo.max_count,
        active_from: promo.active_from,
        active_until: promo.active_until,
        mode: promo.mode,
        promo_common: promo.promo_common,
        promo_unique: Some(promo.promo_unique.unwrap_or_default().0),
        promo_id: promo.promo_id,
        company_id: promo.company_id,
        company_name: promo.company_name,
        like_count: promo.like_count,
        used_count: promo.used_count,
        active: promo.active,
    }))
}

pub async fn edit_promo(
    Extension(company): Extension<Company>,
    Path(id): Path<String>,
    Json(patch_promo): Json<PatchPromo>,
) -> Result<Json<PromoReadOnly>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let promo: Option<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE promo_id = ?
    "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    if promo.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if promo.as_ref().unwrap().company_id != company.id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut promo = promo.unwrap();
    promo.description = patch_promo.description.unwrap_or(promo.description);
    promo.image_url = patch_promo
        .image_url
        .is_some()
        .then_some(patch_promo.image_url.unwrap());
    promo.target = patch_promo.target.unwrap_or(promo.target);
    promo.max_count = patch_promo.max_count.unwrap_or(promo.max_count);
    promo.active_from = patch_promo
        .active_from
        .is_some()
        .then_some(patch_promo.active_from.unwrap());
    promo.active_until = patch_promo
        .active_until
        .is_some()
        .then_some(patch_promo.active_until.unwrap());

    sqlx::query(r#"
        UPDATE promos
        SET description = ?, image_url = ?, target = ?, max_count = ?, active_from = ?, active_until = ?,
        WHERE promo_id = ?
        "#)
    .bind(&promo.description)
    .bind(&promo.image_url)
    .bind(&promo.target)
    .bind(promo.max_count)
    .bind(&promo.active_from)
    .bind(&promo.active_until).fetch_optional(&pool).await.unwrap();

    Ok(Json(PromoReadOnly {
        description: promo.description,
        image_url: promo.image_url,
        target: promo.target,
        max_count: promo.max_count,
        active_from: promo.active_from,
        active_until: promo.active_until,
        mode: promo.mode,
        promo_common: promo.promo_common,
        promo_unique: Some(promo.promo_unique.unwrap_or_default().0),
        promo_id: promo.promo_id,
        company_id: promo.company_id,
        company_name: promo.company_name,
        like_count: promo.like_count,
        used_count: promo.used_count,
        active: promo.active,
    }))
}
