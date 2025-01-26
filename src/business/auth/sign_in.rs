use crate::{
    business::auth::{encode_jwt, retrieve_company_by_email, verify_password},
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

impl SignInData {
    pub fn is_valid(&self) -> bool {
        if self.email.chars().count() < 8 || self.email.chars().count() > 120 {
            return false;
        }

        let mut has_whitespace = false;
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        for c in self.password.chars() {
            has_whitespace |= c.is_whitespace();
            has_lower |= c.is_lowercase();
            has_upper |= c.is_uppercase();
            has_digit |= c.is_digit(10);
        }

        return !has_whitespace
            && has_upper
            && has_lower
            && has_digit
            && self.password.chars().count() >= 8
            && self.password.chars().count() <= 60;
    }
}

pub async fn sign_in(
    State(app_state): State<AppState>,
    Json(sign_in_data): Json<SignInData>,
) -> Result<Json<Value>, StatusCode> {
    if !sign_in_data.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let company = match retrieve_company_by_email(&app_state.pool, &sign_in_data.email).await {
        Some(company) => company,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if !verify_password(&sign_in_data.password, &company.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(company.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "token": token
    })))
}
