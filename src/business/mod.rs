use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub mod auth;
pub mod middlewares;
pub mod promo;

pub struct BusinessResponse<T> {
    data: Option<T>,
    message: String,
    status_code: StatusCode,
}

// impl<T> IntoResponse for BusinessResponse<T> {
//     fn into_response(self) -> axum::response::Response {
//         let response = Json(json!({
//             "data": self.data,
//             "message": self.message,
//             "status_code": self.status_code
// }        ));
//         // (self.data, self.message, self.status_code).into_response()
//     }
// }
