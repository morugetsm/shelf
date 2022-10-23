use super::*;
use chrono::prelude::*;
use serde::Serialize;
use sqlx::{postgres::PgRow, query, Error, Row};

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct User {
    idx: i32,
    username: String,
    name: String,
    admin: bool,
    rdate: NaiveDateTime,
    udate: Option<NaiveDateTime>,
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            idx: row.get("idx"),
            username: row.get("username"),
            name: row.get("name"),
            admin: row.get("admin"),
            rdate: row.get("rdate"),
            udate: row.get("udate"),
        }
    }
}

pub async fn select(
    idx: Option<u32>,
    params: Option<HashMap<String, String>>,
) -> Result<Response<User>, Error> {
    let mut conn = connect().await?;

    let mut builder = String::from("SELECT count(*) OVER() AS total, * FROM \"user\" ");

    if let Some(idx) = idx {
        builder.push_str(&format!("WHERE idx={} ", idx));
    }

    if let Some(params) = params {
        if let Some(username) = params.get("username") {
            if builder.contains("WHERE") {
                builder.push_str(&format!("AND username=\'{}\' ", username));
            } else {
                builder.push_str(&format!("WHERE username=\'{}\' ", username));
            }
        }
        builder.push_str(&loos(params));
    }

    println!("{}", builder);

    let query = query(&builder);

    let rows = query.fetch_all(&mut conn).await?;

    Ok(Response::from(rows))
}
