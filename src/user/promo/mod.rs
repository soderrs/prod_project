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
        like_count: promo.likes.0.len() as u32,
        is_liked_by_user: promo.likes.0.contains(&user.email),
        comment_count: promo.comments.0.len() as u32,
    }))
}
