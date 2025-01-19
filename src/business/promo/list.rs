use super::{Promo, PromoReadOnly};
use crate::business::auth::Company;
use axum::{extract::Query, http::StatusCode, Extension, Json};
use sqlx::SqlitePool;
use std::env;

pub async fn list_promos(
    Extension(company): Extension<Company>,
    Query(params): Query<Vec<(String, String)>>,
) -> Result<Json<Vec<PromoReadOnly>>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let mut countries = vec![];

    for param in params {
        if param.0 == "country" {
            countries.push(param.1);
        }
    }

    let promos: Vec<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE company_id = ?
        "#,
    )
    .bind(company.id)
    .fetch_all(&pool)
    .await
    .unwrap();

    let promos: Vec<Promo> = promos
        .into_iter()
        .filter(|promo| {
            promo.target.0.country.is_none()
                || countries.contains(&promo.target.0.country.as_ref().unwrap())
        })
        .collect();

    let read_only_promos = promos
        .into_iter()
        .map(|promo| PromoReadOnly {
            description: promo.description,
            image_url: promo.image_url,
            target: promo.target,
            max_count: promo.max_count,
            active_from: promo.active_from,
            active_until: promo.active_until,
            mode: promo.mode,
            promo_common: promo.promo_common,
            promo_unique: promo.promo_unique,
            promo_id: promo.promo_id,
            company_id: promo.company_id,
            company_name: promo.company_name,
            like_count: promo.like_count,
            used_count: promo.used_count,
            active: promo.active,
        })
        .collect();

    Ok(Json(read_only_promos))
}
