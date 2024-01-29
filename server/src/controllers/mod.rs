use axum::http::StatusCode;
use std::io::ErrorKind;

pub mod user;

trait IntoStatusCode {
    fn into_status_code(self) -> StatusCode;
}

impl IntoStatusCode for sqlx::Error {
    fn into_status_code(self) -> StatusCode {
        tracing::error!("response error: {:#?}", self);

        let error_code = match self {
            sqlx::Error::RowNotFound => Some(StatusCode::NOT_FOUND),
            sqlx::Error::Database(ref e) => {
                let e: &sqlx::mysql::MySqlDatabaseError = e.downcast_ref();
                tracing::error!("database error: {:#?}", e);
                match e.code() {
                    Some("23505") => Some(StatusCode::CONFLICT),
                    Some("42601") => Some(StatusCode::INTERNAL_SERVER_ERROR), // SQL syntax error
                    _ => None,
                }
            }
            sqlx::Error::Io(e) => match e.kind() {
                ErrorKind::ConnectionRefused => Some(StatusCode::SERVICE_UNAVAILABLE),
                _ => None,
            },
            _ => None,
        };

        error_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
