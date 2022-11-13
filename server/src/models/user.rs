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

pub async fn select(params: HashMap<String, String>) -> Result<Response<User>, Error> {
    let mut conn = connect().await?;

    let mut builder = String::from("SELECT count(*) OVER() AS total, * FROM public.user ");
    builder.push_str(&make_where(&params, vec!["removed", "idx", "username"]));
    builder.push_str(&make_oslo(&params));

    println!("{}", builder);

    let query = query(&builder);
    let results = query.fetch_all(&mut conn).await?;
    Ok(Response::from(results))
}

pub async fn delete(idx: u32) -> Result<Option<User>, Error> {
    let mut conn = connect().await?;

    let check = format!("SELECT idx from public.user where idx={}", idx);
    if let Err(exist) = query(&check).fetch_one(&mut conn).await {
        return Err(exist);
    }
    
    let builder = format!("UPDATE public.user SET removed='true', udate=NOW() WHERE idx={} AND removed='false' RETURNING *", idx);
    
    println!("{}", builder);
    
    let query = query(&builder);
    let result = query.fetch_optional(&mut conn).await?;
    if let Some(row) = result {
        Ok(Some(User::from(row)))
    } else {
        Ok(None)
    }
}
