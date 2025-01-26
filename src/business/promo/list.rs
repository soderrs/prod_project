use super::{Promo, PromoReadOnly};
use crate::{business::auth::Company, AppState};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::NaiveDate;

pub async fn list_promos(
    State(app_state): State<AppState>,
    Extension(company): Extension<Company>,
    Query(params): Query<Vec<(String, String)>>,
) -> Response {
    let promos: Vec<Promo> = sqlx::query_as(
        r#"
        SELECT * FROM promos WHERE company_id = ?
        "#,
    )
    .bind(company.id)
    .fetch_all(&app_state.pool)
    .await
    .unwrap();

    let mut countries = vec![];

    for param in params.iter() {
        if param.0 == "country" {
            countries.push(param.1.clone().to_lowercase());
        }
    }

    let mut promos: Vec<Promo> = promos
        .into_iter()
        .filter(|promo| {
            promo.target.0.country.is_none()
                || countries.contains(&promo.target.0.country.as_ref().unwrap())
                || countries.is_empty()
        })
        .collect();
    println!("{:?}", params);

    if let Some(offset) = params.iter().rposition(|param| param.0 == "offset") {
        let mut offset = params[offset].1.parse::<usize>().unwrap();
        if offset + 1 > promos.len() {
            offset = promos.len();
        }
        promos = promos.split_at(offset).1.to_vec();
    }

    if let Some(limit) = params.iter().rposition(|param| param.0 == "limit") {
        let mut limit = params[limit].1.parse::<usize>().unwrap();
        if limit + 1 > promos.len() {
            limit = promos.len();
        }
        promos = promos.split_at(limit).0.to_vec();
    }

    if let Some(sort_idx) = params.iter().rposition(|param| param.0 == "sort_by") {
        if params[sort_idx].1 != "active_from" && params[sort_idx].1 != "active_until" {
            return StatusCode::BAD_REQUEST.into_response();
        } else if params[sort_idx].1 == "active_until" {
            promos.sort_by(|a, b| {
                a.active_until
                    .unwrap_or(sqlx::types::Json(
                        NaiveDate::from_ymd_opt(1, 12, 31).unwrap(),
                    ))
                    .0
                    .cmp(
                        &b.active_until
                            .unwrap_or(sqlx::types::Json(
                                NaiveDate::from_ymd_opt(1, 12, 31).unwrap(),
                            ))
                            .0,
                    )
            });
        } else if params[sort_idx].1 == "active_from" {
            promos.sort_by(|a, b| {
                a.active_from
                    .unwrap_or(sqlx::types::Json(
                        NaiveDate::from_ymd_opt(1, 12, 31).unwrap(),
                    ))
                    .0
                    .cmp(
                        &b.active_from
                            .unwrap_or(sqlx::types::Json(
                                NaiveDate::from_ymd_opt(1, 12, 31).unwrap(),
                            ))
                            .0,
                    )
            });
        }
    } else {
        promos.sort_by(|b, a| a.create_date.0.cmp(&b.create_date.0));
    }
    let read_only_promos: Vec<PromoReadOnly> = promos
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
            like_count: promo.likes.0.len() as i32,
            used_count: promo.used_count,
            active: promo.active,
        })
        .collect();
    let mut headers = HeaderMap::new();
    headers.insert("X-Total-Count", HeaderValue::from(read_only_promos.len()));
    (StatusCode::OK, headers, Json(read_only_promos)).into_response()
}
