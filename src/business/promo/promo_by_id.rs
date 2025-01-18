use axum::{extract::Path, Extension, Json};

use crate::business::auth::Company;

// pub async fn get_promo(Extension(company): Extension<Company>, Path(id): Path<String>) -> Json<Promo>
