use crate::business::auth::{decode_jwt, retrieve_company_by_email};
use axum::{
    body::Body,
    extract::Request,
    http::{self, Response, StatusCode},
    middleware::Next,
};

pub async fn authorize_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = match req.headers_mut().get(http::header::AUTHORIZATION) {
        Some(header) => header.to_str().map_err(|_| StatusCode::FORBIDDEN)?,
        None => return Err(StatusCode::FORBIDDEN)?,
    };

    let [_, token, ..] = *auth_header.split_whitespace().collect::<Vec<&str>>() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let token_data = match decode_jwt(token) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let company = match retrieve_company_by_email(&token_data.claims.email).await {
        Some(company) => company,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(company);
    Ok(next.run(req).await)
}
