use crate::models::user as model;

use axum::{
    extract::{Form, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::collections::HashMap;

pub async fn get_all(
    id: Option<Path<u32>>,
    params: Option<Query<HashMap<String, String>>>,
) -> (StatusCode, impl IntoResponse) {
    let params = params.map(|query| query.0).unwrap_or_default();

    if let Ok(res) = model::select_list(params).await {
        (StatusCode::OK, Json::from(res))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
    }
}

pub async fn get_one(id: Path<u32>) -> (StatusCode, impl IntoResponse) {
    let id = id.0;

    if let Ok(res) = model::select_item(id).await {
        (StatusCode::OK, Json::from(res))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
    }
}

pub async fn post(body: Form<model::UserReq>) -> (StatusCode, impl IntoResponse) {
    let body = body.0;

    match model::insert(body).await {
        Ok(row) => (StatusCode::CREATED, Json::from(row)),
        Err(e) => match e {
            sqlx::Error::Database(ref error) if error.code().unwrap_or_default() == "23505" => {
                (StatusCode::CONFLICT, Json::default())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json::default()),
        },
    }
}

pub async fn patch(
    id: Option<Path<u32>>,
    body: Form<model::UserReq>,
) -> (StatusCode, impl IntoResponse) {
    let body = body.0;

    let Some(id) = id.map(|path| path.0) else {
        return (StatusCode::BAD_REQUEST, Json::default());
    };

    match model::update(id, body).await {
        Ok(Some(row)) => (StatusCode::OK, Json::from(row)),
        Ok(None) => (StatusCode::CONFLICT, Json::default()),
        Err(e) => match e {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, Json::default()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json::default()),
        },
    }
}

pub async fn delete(id: Option<Path<u32>>) -> (StatusCode, impl IntoResponse) {
    let Some(id) = id.map(|path| path.0) else {
        return (StatusCode::BAD_REQUEST, Json::default());
    };

    match model::delete(id).await {
        Ok(Some(row)) => (StatusCode::OK, Json::from(row)),
        Ok(None) => (StatusCode::GONE, Json::default()),
        Err(e) => match e {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, Json::default()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json::default()),
        },
    }
}
