use crate::business::{self, auth, middlewares};
use axum::{
    middleware,
    routing::{get, patch, post},
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
        .route(
            "/api/business/promo",
            get(business::promo::list::list_promos).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
        .route(
            "/api/business/promo/{id}",
            get(business::promo::promo_by_id::get_promo).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
        .route(
            "/api/business/promo/{id}",
            patch(business::promo::promo_by_id::edit_promo).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
        .route(
            "/api/business/promo/{id}/stat",
            get(business::promo::promo_by_id::get_promo_stat).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
}

async fn ping() -> Json<String> {
    Json(String::from("hui"))
}
