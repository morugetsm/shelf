use crate::{controllers::IntoStatusCode, models::user as model};

use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use std::collections::HashMap;

pub async fn get(
    id: Option<Path<u32>>,
    params: Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match id {
        Some(Path(id)) => {
            let result = model::select_item(id).await;

            match result {
                Ok(res) => Json::from(res).into_response(),
                Err(err) => err.into_status_code().into_response(),
            }
        }
        None => {
            let Query(params) = params;

            let result = model::select_list(params).await;

            match result {
                Ok(res) => Json::from(res).into_response(),
                Err(err) => err.into_status_code().into_response(),
            }
        }
    }
}

pub async fn post(Json(body): Json<model::UserReq>) -> impl IntoResponse {
    let result = model::insert(body).await;

    match result {
        Ok(res) => (StatusCode::CREATED, Json::from(res)).into_response(),
        Err(err) => err.into_status_code().into_response(),
    }
}

pub async fn patch(Path(id): Path<u32>, Json(body): Json<model::UserReq>) -> impl IntoResponse {
    let result = model::update(id, body).await;

    match result {
        Ok(Some(res)) => (StatusCode::OK, Json::from(res)).into_response(),
        Ok(None) => StatusCode::CONFLICT.into_response(),
        Err(err) => err.into_status_code().into_response(),
    }
}

pub async fn delete(Path(id): Path<u32>) -> impl IntoResponse {
    let result = model::delete(id).await;

    match result {
        Ok(Some(res)) => (StatusCode::OK, Json::from(res)).into_response(),
        Ok(None) => StatusCode::GONE.into_response(),
        Err(err) => err.into_status_code().into_response(),
    }
}
