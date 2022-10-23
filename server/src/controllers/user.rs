use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::collections::HashMap;

use crate::models::user as model;

pub async fn get(
    idx: Option<Path<u32>>,
    params: Option<Query<HashMap<String, String>>>,
) -> (StatusCode, impl IntoResponse) {
    let idx = idx.map(|path| path.0);
    let params = params.map(|query| query.0);

    if let Ok(response) = model::select(idx, params).await {
        (StatusCode::OK, Json::from(response))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
    }
}
