use super::{promo, User};
use crate::{
    business::promo::{Promo, PromoForUser},
    AppState,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};

pub async fn promo_feed(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Query(params): Query<Vec<(String, String)>>,
) -> Result<Json<Vec<PromoForUser>>, StatusCode> {
    let promos: Vec<Promo> = sqlx::query_as(
        r#"
            SELECT * FROM promos
        "#,
    )
    .fetch_all(&app_state.pool)
    .await
    .unwrap();
    let promos_for_user = promos
        .into_iter()
        .map(|promo| PromoForUser {
            promo_id: promo.promo_id,
            company_id: promo.company_id,
            company_name: promo.company_name,
            description: promo.description,
            image_url: promo.image_url,
            active: promo.active,
            is_activated_by_user: promo.activated_users.contains(&user.email),
            like_count: promo.likes.0.len() as u32,
            is_liked_by_user: promo.likes.0.contains(&user.email),
            comment_count: promo.comments.0.len() as u32,
        })
        .collect();

    Ok(Json(promos_for_user))
}
