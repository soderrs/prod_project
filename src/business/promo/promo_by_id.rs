use std::str::FromStr;

use super::{PatchPromo, Promo, PromoReadOnly, PromoStat};
use crate::{business::auth::Company, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::NaiveDate;

pub async fn get_promo(
    State(app_state): State<AppState>,
    Extension(company): Extension<Company>,
    Path(id): Path<String>,
) -> Result<Json<PromoReadOnly>, StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if promo.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let promo = promo.unwrap();

    if promo.company_id != company.id {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(Json(PromoReadOnly {
        description: promo.description,
        image_url: promo.image_url,
        target: promo.target,
        max_count: promo.max_count,
        active_from: promo.active_from,
        active_until: promo.active_until,
        mode: promo.mode,
        promo_common: promo.promo_common,
        promo_unique: Some(promo.promo_unique.unwrap_or_default()),
        promo_id: promo.promo_id,
        company_id: promo.company_id,
        company_name: promo.company_name,
        like_count: promo.likes.0.len() as i32,
        used_count: promo.used_count,
        active: promo.active,
    }))
}

pub async fn edit_promo(
    State(app_state): State<AppState>,
    Extension(company): Extension<Company>,
    Path(id): Path<String>,
    Json(patch_promo): Json<PatchPromo>,
) -> Result<Json<PromoReadOnly>, StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE promo_id = ?
    "#,
    )
    .bind(&id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if promo.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    if promo.as_ref().unwrap().company_id != company.id {
        return Err(StatusCode::FORBIDDEN);
    }
    if !patch_promo.is_valid(promo.as_ref().unwrap()) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut promo = promo.unwrap();
    promo.description = patch_promo.description.unwrap_or(promo.description);
    if let Some(image_url) = patch_promo.image_url {
        promo.image_url = Some(image_url);
    }
    promo.target = patch_promo.target.unwrap_or(promo.target);
    promo.max_count = patch_promo.max_count.unwrap_or(promo.max_count);
    if let Some(_) = patch_promo.active_from {
        promo.active_from = Some(sqlx::types::Json(
            NaiveDate::from_str(&patch_promo.active_from.unwrap()).unwrap(),
        ));
    }
    if let Some(_) = patch_promo.active_until {
        promo.active_until = Some(sqlx::types::Json(
            NaiveDate::from_str(&patch_promo.active_until.unwrap()).unwrap(),
        ));
    }

    sqlx::query(r#"
        UPDATE promos
        SET description = ?, image_url = ?, target = ?, max_count = ?, active_from = ?, active_until = ?
        WHERE promo_id = ?
        "#)
    .bind(&promo.description)
    .bind(&promo.image_url)
    .bind(&promo.target)
    .bind(promo.max_count)
    .bind(&promo.active_from)
    .bind(&promo.active_until)
    .bind(id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    Ok(Json(PromoReadOnly {
        description: promo.description,
        image_url: promo.image_url,
        target: promo.target,
        max_count: promo.max_count,
        active_from: promo.active_from,
        active_until: promo.active_until,
        mode: promo.mode,
        promo_common: promo.promo_common,
        promo_unique: Some(promo.promo_unique.unwrap_or_default()),
        promo_id: promo.promo_id,
        company_id: promo.company_id,
        company_name: promo.company_name,
        like_count: promo.likes.0.len() as i32,
        used_count: promo.used_count,
        active: promo.active,
    }))
}

pub async fn get_promo_stat(
    State(app_state): State<AppState>,
    Extension(company): Extension<Company>,
    Path(id): Path<String>,
) -> Result<Json<PromoStat>, StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if promo.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let promo = promo.unwrap();
    if promo.company_id != company.id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(PromoStat {
        activate_count: promo.used_count,
        countries: promo.countries,
    }))
}
