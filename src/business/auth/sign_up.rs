use crate::{
    business::auth::{hash_password, Company},
    user::middlewares::authorize::encode_jwt,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateCompany {
    name: String,
    email: String,
    password: String,
}

pub async fn sign_up(
    State(app_state): State<AppState>,
    Json(sign_up_data): Json<CreateCompany>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !is_valid_sign_up_data(&sign_up_data) {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !is_unique_company(&app_state.pool, &sign_up_data).await {
        return Err(StatusCode::CONFLICT);
    }
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO companies VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(sign_up_data.name)
    .bind(&sign_up_data.email)
    .bind(hash_password(&sign_up_data.password).unwrap())
    .execute(&app_state.pool)
    .await
    .unwrap();

    let token = encode_jwt(sign_up_data.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "company_id": id,
        "token": token
    })))
}

async fn is_unique_company(pool: &PgPool, sign_up_data: &CreateCompany) -> bool {
    let row: Option<Company> = sqlx::query_as(
        r#"
        SELECT * FROM companies WHERE email = ?
        "#,
    )
    .bind(&sign_up_data.email)
    .fetch_optional(pool)
    .await
    .unwrap();

    row.is_none()
}

fn is_valid_sign_up_data(sign_up_data: &CreateCompany) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    if sign_up_data.name.chars().count() < 5 || sign_up_data.name.chars().count() > 50 {
        return false;
    }

    for c in sign_up_data.password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }

    if !email_regex.is_match(&sign_up_data.email) {
        return false;
    }
    !has_whitespace
        && has_upper
        && has_lower
        && has_digit
        && (sign_up_data.password.chars().count() >= 8
            || sign_up_data.password.chars().count() <= 60)
}
