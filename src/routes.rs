use crate::business::{self, auth, middlewares};
use axum::{
    middleware,
    routing::{get, post},
    Json, Router,
};

pub async fn app() -> Router {
    Router::new()
        .route("/api/ping", get(ping))
        .route("/api/business/auth/sign-up", post(auth::sign_up::sign_up))
        .route("/api/business/auth/sign-in", post(auth::sign_in::sign_in))
        .route(
            "/api/business/promo",
            post(business::promo::create::create_promo).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
}

async fn ping() -> Json<String> {
    Json(String::from("hui"))
}
