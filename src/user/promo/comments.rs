use std::collections::HashSet;

use axum::{
    extract::{
        connect_info::Connected,
        rejection::{JsonDataError, LengthLimitError},
        Path, State,
    },
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    business::promo::{Comment, CommentAuthor, Promo},
    user::User,
    AppState,
};

pub async fn add_comment(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Path(promo_id): Path<String>,
    Json(text): Json<String>,
) -> Result<(StatusCode, Json<Comment>), StatusCode> {
    let id = Uuid::new_v4().to_string();
    let comment = Comment {
        id,
        text,
        date: chrono::Utc::now().to_string(),
        author: CommentAuthor {
            name: user.name,
            surname: user.surname,
            avatar_url: user.avatar_url,
        },
    };

    let promo: Option<Promo> = sqlx::query_as(
        r#"
        SELECT comments FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(&promo_id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if let Some(promo) = promo {
        let mut comments = promo.comments;
        comments.insert(comment.clone());
        sqlx::query(
            r#"
                UPDATE promos
                SET comments = ?
                WHERE promo_id = ?
            "#,
        )
        .bind(sqlx::types::Json(&comment))
        .bind(promo_id)
        .execute(&app_state.pool)
        .await
        .unwrap();
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn get_comments(
    State(app_state): State<AppState>,
    Extension(_user): Extension<User>,
    Path(promo_id): Path<String>,
) -> Result<Json<HashSet<Comment>>, StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
            SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(promo_id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if let Some(promo) = promo {
        return Ok(Json(promo.comments.0));
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}

pub async fn get_comment_by_id(
    State(app_state): State<AppState>,
    Path(promo_id): Path<String>,
    Path(comment_id): Path<String>,
) -> Result<Json<Comment>, StatusCode> {
    let promo: Option<Promo> = sqlx::query_as(
        r#"
            SELECT * FROM promos WHERE promo_id = ?
        "#,
    )
    .bind(promo_id)
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    if let Some(promo) = promo {
        let comments = Vec::from_iter(promo.comments.0.into_iter());
        if let Some(comment) = comments
            .into_iter()
            .find(|comment| comment.id == comment_id)
        {
            return Ok(Json(comment));
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}
