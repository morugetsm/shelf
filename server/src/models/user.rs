use super::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, query, Error, Row};

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct UserRow {
    username: String,
    name: String,
    admin_yn: bool,
    rdate: NaiveDateTime,
    udate: Option<NaiveDateTime>,
}

impl From<PgRow> for UserRow {
    fn from(row: PgRow) -> Self {
        UserRow {
            username: row.get("username"),
            name: row.get("name"),
            admin_yn: row.get("admin_yn"),
            rdate: row.get("rdate"),
            udate: row.get("udate"),
        }
    }
}

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct UserRes {
    id: i32,
    username: String,
    name: String,
    admin_yn: bool,
    rdate: NaiveDateTime,
    udate: Option<NaiveDateTime>,
}

impl From<PgRow> for UserRes {
    fn from(row: PgRow) -> Self {
        UserRes {
            id: row.get("id"),
            username: row.get("username"),
            name: row.get("name"),
            admin_yn: row.get("admin_yn"),
            rdate: row.get("rdate"),
            udate: row.get("udate"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserReq {
    username: String,
    password: String,
    name: String,
    admin_yn: Option<bool>,
}

pub async fn select_list(params: HashMap<String, String>) -> Result<List<UserRow>, Error> {
    let mut conn = connect().await?;

    let mut builder = String::from("SELECT count(*) OVER() AS total, * FROM public.user ");
    builder.push_str(&make_where(&params, vec!["id", "username", "remove_yn"]));
    builder.push_str(&make_solo(&params));

    println!("{}", builder);

    let query = query(&builder);
    let results = query.fetch_all(&mut conn).await?;
    Ok(List::from(results))
}

pub async fn select_item(id: u32) -> Result<UserRes, Error> {
    let mut conn = connect().await?;

    let builder = format!("SELECT * FROM public.user WHERE id = {}", id);

    println!("{}", builder);

    let query = query(&builder);
    let row = query.fetch_one(&mut conn).await?;
    Ok(UserRes::from(row))
}

pub async fn insert(data: UserReq) -> Result<UserRes, Error> {
    let mut conn = connect().await?;

    let builder = format!(
        r#"
        INSERT INTO public.user 
        (username, password, name, admin_yn) 
        VALUES ('{}', '{}', '{}', {}) RETURNING *"#,
        data.username,
        data.password,
        data.name,
        data.admin_yn.unwrap_or(false)
    );

    let query = query(&builder);
    let result = query.fetch_one(&mut conn).await?;
    Ok(UserRes::from(result))
}

pub async fn update(id: u32, data: UserReq) -> Result<Option<UserRes>, Error> {
    let mut conn = connect().await?;

    let builder = format!(
        "UPDATE public.user SET (username, password, name, admin_yn) = ({}, {}, {}, {})",
        data.username, data.password, data.name, true
    );

    let query = query(&builder);

    let result = query.fetch_optional(&mut conn).await?;
    if let Some(row) = result {
        Ok(Some(UserRes::from(row)))
    } else {
        Ok(None)
    }
}

pub async fn delete(id: u32) -> Result<Option<UserRes>, Error> {
    let mut conn = connect().await?;

    let check = format!("SELECT id from public.user where id={}", id);
    query(&check).fetch_one(&mut conn).await?;

    let builder = format!("UPDATE public.user SET remove_yn='true', udate=NOW() WHERE id={} AND remove_yn='false' RETURNING *", id);

    println!("{}", builder);

    let query = query(&builder);
    let result = query.fetch_optional(&mut conn).await?;
    if let Some(row) = result {
        Ok(Some(UserRes::from(row)))
    } else {
        Ok(None)
    }
}
