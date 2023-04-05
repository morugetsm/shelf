use axum::http::StatusCode;
use std::io::ErrorKind;

pub mod user;

trait IntoStatusCode {
    fn into_status_code(self) -> StatusCode;
}

impl IntoStatusCode for sqlx::Error {
    fn into_status_code(self) -> StatusCode {
        println!("response error: {:#?}", self);

        match self {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            sqlx::Error::Database(ref e) => {
                let e: &sqlx::postgres::PgDatabaseError = e.downcast_ref();
                println!("database error: {:#?}", e);
                match e.code() {
                    "23505" => StatusCode::CONFLICT,
                    "42601" => StatusCode::INTERNAL_SERVER_ERROR, // SQL syntax error
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            sqlx::Error::Io(e) => match e.kind() {
                ErrorKind::ConnectionRefused => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
