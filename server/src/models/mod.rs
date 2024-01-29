pub mod user;

use serde::Serialize;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlRow},
    Connection, Error, MySqlConnection, Row,
};
use std::{collections::HashMap, fmt::Display};

pub async fn connect() -> Result<MySqlConnection, Error> {
    let options = MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .username("houu")
        .database("shelf");

    let conn = MySqlConnection::connect_with(&options).await?;

    Ok(conn)
}

#[derive(Debug, Serialize)]
pub struct List<T> {
    pub total: i64,
    pub records: Vec<T>,
}

impl<T> From<Vec<MySqlRow>> for List<T>
where
    T: From<MySqlRow>,
{
    fn from(rows: Vec<MySqlRow>) -> Self {
        if !rows.is_empty()
            && rows
                .first()
                .is_some_and(|row| row.get::<i32, &str>("total") > 0)
        {
            Self {
                total: rows[0].get("total"),
                records: rows.into_iter().map(|row| T::from(row)).collect(),
            }
        } else {
            Self::default()
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self {
            total: 0,
            records: vec![],
        }
    }
}

pub enum Order {
    Asc,
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        };
        write!(f, "{}", order)
    }
}

impl From<&String> for Order {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "asc" | "ASC" => Self::Asc,
            "desc" | "DESC" => Self::Desc,
            _ => Self::default(),
        }
    }
}

struct Solo {
    sort: String,
    order: Order,
    limit: i32,
    offset: i32,
}

impl From<&HashMap<String, String>> for Solo {
    fn from(params: &HashMap<String, String>) -> Self {
        Self {
            sort: params
                .get("sort")
                .map(|val| {
                    val.chars()
                        .filter(|ch| ch.is_alphanumeric() || ch == &'_')
                        .collect()
                })
                .unwrap_or(String::from("idx")),
            order: params.get("order").map(Order::from).unwrap_or_default(),
            limit: params
                .get("limit")
                .and_then(|val| val.parse::<i32>().ok())
                .unwrap_or(15),
            offset: params
                .get("offset")
                .and_then(|val| val.parse::<i32>().ok())
                .unwrap_or(0),
        }
    }
}
