use axum::{http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::user::middlewares::authorize::{encode_jwt, retrieve_user_by_email, verify_password};

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(Json(sign_in_data): Json<SignInData>) -> Result<Json<Value>, StatusCode> {
    let user = match retrieve_user_by_email(&sign_in_data.email).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&sign_in_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "token": token
    })))
}
