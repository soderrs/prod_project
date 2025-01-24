use crate::{business::promo::Promo, user::User, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};

pub async fn add_like(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Path(promo_id): Path<String>,
) -> Result<(), StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
            SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(&promo_id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if let Some(mut promo) = promo {
        promo.likes.0.insert(user.email);
        sqlx::query(
            r#"
                UPDATE promos
                SET likes = ?
                WHERE promo_id = ?
            "#,
        )
        .bind(promo.likes)
        .bind(promo_id)
        .execute(&app_state.pool)
        .await
        .unwrap();
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(())
}

pub async fn remove_like(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Path(promo_id): Path<String>,
) -> Result<(), StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
            SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(&promo_id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if let Some(mut promo) = promo {
        promo.likes.0.remove(&user.email);
        sqlx::query(
            r#"
                UPDATE promos
                SET likes = ?
                WHERE promo_id = ?
            "#,
        )
        .bind(promo.likes)
        .bind(promo_id)
        .execute(&app_state.pool)
        .await
        .unwrap();
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(())
}
