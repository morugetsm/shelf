use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::default::Default;

pub mod user;

pub fn make_response(
    result: Result<impl IntoResponse + Default + Serialize, impl std::error::Error>,
) -> (StatusCode, impl IntoResponse) {
    match result {
        Ok(res) => (StatusCode::OK, Json::from(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json::default()),
    }
}
