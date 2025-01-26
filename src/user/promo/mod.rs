use super::User;
use crate::{
    business::promo::{Promo, PromoForUser},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::{json, Value};

pub mod comments;
pub mod like;

pub async fn get_promo(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<PromoForUser>, StatusCode> {
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

    Ok(Json(PromoForUser {
        promo_id: promo.promo_id,
        company_id: promo.company_id,
        company_name: promo.company_name,
        description: promo.description,
        image_url: promo.image_url,
        active: promo.active,
        is_activated_by_user: promo.activated_users.0.contains(&user.email),
        like_count: promo.likes.0.len() as i32,
        is_liked_by_user: promo.likes.0.contains(&user.email),
        comment_count: promo.comments.0.len() as i32,
    }))
}

pub async fn activate_promo(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    Json(promo_name): Json<Option<String>>,
) -> Result<Json<Value>, StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
            SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    let mut promo = match promo {
        Some(promo) => promo,
        None => return Err(StatusCode::NOT_FOUND),
    };

    if !promo.active
        || promo.max_count == promo.activated_users.0.len() as i32
        || promo.target.0.age_from.unwrap_or(user.other.age) > user.other.age
        || promo.target.0.age_until.unwrap_or(user.other.age) < user.other.age
    {
        return Err(StatusCode::FORBIDDEN);
    }

    if promo.mode == "UNIQUE" {
        if promo_name.is_none() {
            return Err(StatusCode::BAD_REQUEST);
        }
        if !promo
            .promo_unique
            .as_ref()
            .unwrap()
            .contains(&promo_name.unwrap())
        {
            return Err(StatusCode::NOT_FOUND);
        }

        if promo.promo_unique.as_ref().unwrap().len() < 1 {
            return Err(StatusCode::FORBIDDEN);
        }

        promo.activated_users.0.insert(user.email);
        let mut promos = Vec::from_iter(promo.promo_unique.as_ref().unwrap().iter());

        let activated_promo = promos.pop().unwrap();

        sqlx::query(
            r#"
                UPDATE promos
                SET activated_users = ?, promo_unique = ?
                WHERE promo_id = ?
            "#,
        )
        .bind(sqlx::types::Json(promo.activated_users))
        .bind(sqlx::types::Json(Some(promo.promo_unique.clone())))
        .execute(&app_state.pool)
        .await
        .unwrap();

        return Ok(Json(json!({
            "text": activated_promo
        })));
    } else {
        promo.activated_users.0.insert(user.email);

        sqlx::query(
            r#"
                UPDATE promos
                SET activated_users = ?
                WHERE promo_id = ?
            "#,
        )
        .bind(sqlx::types::Json(promo.activated_users))
        .execute(&app_state.pool)
        .await
        .unwrap();

        Ok(Json(json!({
            "text": promo.promo_common.unwrap()
        })))
    }
}
