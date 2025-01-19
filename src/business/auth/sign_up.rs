use crate::business::auth::{hash_password, Company};
use axum::{http::StatusCode, Json};
use regex::Regex;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::env;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateCompany {
    name: String,
    email: String,
    password: String,
}

pub async fn sign_up(Json(sign_up_data): Json<CreateCompany>) -> Result<StatusCode, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    if !is_valid_sign_up_data(&sign_up_data) {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !is_unique_company(&sign_up_data).await {
        return Err(StatusCode::CONFLICT);
    }
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO companies VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(sign_up_data.name)
    .bind(sign_up_data.email)
    .bind(hash_password(&sign_up_data.password).unwrap())
    .execute(&pool)
    .await
    .unwrap();

    Ok(StatusCode::OK)
}

async fn is_unique_company(sign_up_data: &CreateCompany) -> bool {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let row: Option<Company> = sqlx::query_as(
        r#"
        SELECT * FROM companies WHERE email = ?
        "#,
    )
    .bind(&sign_up_data.email)
    .fetch_optional(&pool)
    .await
    .unwrap();

    row.is_none()
}

fn is_valid_sign_up_data(sign_up_data: &CreateCompany) -> bool {
    let email_regex = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();

    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

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
        && sign_up_data.password.len() >= 8
        && sign_up_data.password.len() <= 60
}
