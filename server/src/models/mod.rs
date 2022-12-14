pub mod user;

use serde::Serialize;
use sqlx::{
    postgres::{PgConnection, PgRow},
    Connection, Error, Row,
};
use std::collections::HashMap;

pub async fn connect() -> Result<PgConnection, Error> {
    let conn = PgConnection::connect("postgresql://postgres@localhost:5432/shelf").await?;
    Ok(conn)
}

#[derive(Debug, Default, Serialize)]
pub struct Response<T> {
    pub total: i64,
    pub rows: Vec<T>,
}

impl<T> From<Vec<PgRow>> for Response<T>
where
    T: From<PgRow>,
{
    fn from(rows: Vec<PgRow>) -> Self {
        if rows.len() > 0 {
            Response {
                total: rows[0].get("total"),
                rows: rows.into_iter().map(|row| T::from(row)).collect(),
            }
        } else {
            Response {
                total: 0,
                rows: vec![],
            }
        }
    }
}

pub fn make_where(params: &HashMap<String, String>, keys: Vec<&str>) -> String {
    let mut where_vec = Vec::new();

    for (key, value) in params.iter() {
        if keys.contains(&key.as_str()) {
            where_vec.push(format!("{}='{}' ", key, value));
        }
    }

    if where_vec.len() > 0 {
        let mut result = String::from("WHERE ");
        result.push_str(&where_vec.join("AND "));
        result
    } else {
        String::new()
    }
}

pub fn make_oslo(params: &HashMap<String, String>) -> String {
    let mut result = String::new();

    if let Some(order) = params.get("order") {
        if let Some(sort) = params.get("sort") {
            if !order.is_empty() && !sort.is_empty() {
                result.push_str(&format!("ORDER BY {} {} ", order, sort));
            }
        }
    }

    if let Some(limit) = params.get("limit") {
        if !limit.is_empty() {
            result.push_str(&format!("LIMIT {} ", limit));
            if let Some(offset) = params.get("offset") {
                if !offset.is_empty() {
                    result.push_str(&format!("OFFSET {} ", offset));
                }
            }
        }
    }

    result
}
