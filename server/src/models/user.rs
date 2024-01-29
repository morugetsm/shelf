use super::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{Error, QueryBuilder, Row};

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct UserRes {
    id: i32,
    username: String,
    name: String,
    admin: bool,
    reg_date: NaiveDateTime,
    mod_date: Option<NaiveDateTime>,
}

impl From<MySqlRow> for UserRes {
    fn from(row: MySqlRow) -> Self {
        UserRes {
            id: row.get("id"),
            username: row.get("username"),
            name: row.get("name"),
            admin: row.get("admin_yn"),
            reg_date: row.get("reg_date"),
            mod_date: row.get("mod_date"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserReq {
    username: String,
    password: String,
    name: String,
    admin: Option<bool>,
}

pub async fn select_list(params: &HashMap<String, String>) -> Result<List<UserRes>, Error> {
    let mut conn = connect().await?;

    let mut builder = QueryBuilder::new("SELECT *, count(*) AS total FROM auth_user WHERE true");

    if let Some(username) = params.get("username") {
        builder.push(" AND username LIKE ").push_bind(username);
    }
    if let Some(name) = params.get("name") {
        builder.push(" AND name LIKE ").push_bind(name);
    }
    builder.push(" AND delete_yn = ").push_bind(0);

    let Solo {
        sort,
        order,
        limit,
        offset,
    } = Solo::from(params);

    builder.push(" ORDER BY ").push_bind(sort);
    builder.push(" ").push(order);
    builder.push(" LIMIT ").push_bind(limit);
    builder.push(" OFFSET ").push_bind(offset);

    let rows = builder.build().fetch_all(&mut conn).await?;

    Ok(List::from(rows))
}

pub async fn select_item(id: u32) -> Result<UserRes, Error> {
    let mut conn = connect().await?;

    let query = "SELECT * FROM auth_user WHERE id = ?";

    let row = sqlx::query(query).bind(id).fetch_one(&mut conn).await?;
    Ok(UserRes::from(row))
}

pub async fn insert(data: UserReq) -> Result<Option<UserRes>, Error> {
    let mut conn = connect().await?;

    let query = r#"INSERT
        INTO auth_user (username, password, name, admin_yn)
    VALUES
        (?, ?, ?, ?)
    RETURNING *"#;
    let result = sqlx::query(query)
        .bind(data.username)
        .bind(data.password)
        .bind(data.name)
        .bind(data.admin.unwrap_or(false))
        .fetch_optional(&mut conn)
        .await?;

    if let Some(row) = result {
        Ok(Some(UserRes::from(row)))
    } else {
        Ok(None)
    }
}

pub async fn update(id: u32, data: UserReq) -> Result<Option<UserRes>, Error> {
    let mut conn = connect().await?;

    let query = r"UPDATE
            auth_user 
        SET
            (username, password, name, admin_yn) = (?, ?, ?, ?) 
        WHERE
            id = ? AND delete_yn = 0
        RETURNING *";
    let result = sqlx::query(&query)
        .bind(data.username)
        .bind(data.password)
        .bind(data.name)
        .bind(data.admin)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    if let Some(row) = result {
        Ok(Some(UserRes::from(row)))
    } else {
        Ok(None)
    }
}

pub async fn delete(id: u32) -> Result<Option<UserRes>, Error> {
    let mut conn = connect().await?;

    let query = "SELECT id from auth_user where id = ?";
    sqlx::query(query).bind(id).fetch_one(&mut conn).await?;

    let query = r"UPDATE
            auth_user 
        SET
            delete_yn = 1, mod_date = NOW() 
        WHERE
            id = ? AND delete_yn = 0
        RETURNING *";
    let result = sqlx::query(query)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    if let Some(row) = result {
        Ok(Some(UserRes::from(row)))
    } else {
        Ok(None)
    }
}
