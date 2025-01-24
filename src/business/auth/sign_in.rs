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

pub async fn sign_in(
    State(app_state): State<AppState>,
    Json(sign_in_data): Json<SignInData>,
) -> Result<Json<Value>, StatusCode> {
    let company = match retrieve_company_by_email(&app_state.pool, &sign_in_data.email).await {
        Some(company) => company,
        None => return Err(StatusCode::UNAUTHORIZED),
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
