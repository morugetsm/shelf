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
    query_string: Option<Query<HashMap<String, String>>>,
) -> (StatusCode, impl IntoResponse) {
    let idx = idx.map(|path| path.0);
    let query_string = query_string.map(|query| query.0);

    let mut params = HashMap::new();
    params.insert("removed".to_string(), "false".to_string());

    if let Some(idx) = idx {
        params.insert("idx".to_string(), idx.to_string());
    }

    if let Some(query_string) = query_string {
        params.extend(query_string.into_iter());
    }

    if let Ok(response) = model::select(params).await {
        (StatusCode::OK, Json::from(response))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
    }
}

pub async fn delete(idx: Option<Path<u32>>) -> impl IntoResponse {
    let Some(idx) = idx.map(|path| path.0) else {
        return (StatusCode::BAD_REQUEST, Json::default());
    };

    match model::delete(idx).await {
        Ok(Some(row)) => (StatusCode::OK, Json::from(row)),
        Ok(None) => (StatusCode::GONE, Json::default()),
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, Json::default()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
            }
        }
    }
}