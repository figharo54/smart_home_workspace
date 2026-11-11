use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RoomNotFound,
    DeviceNotFound,
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RoomNotFound => write!(f, "комната не найдена"),
            Self::DeviceNotFound => write!(f, "устройство не найдено"),
            Self::BadRequest(m) => write!(f, "неверный запрос: {m}"),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::RoomNotFound | Self::DeviceNotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        #[derive(Serialize)]
        struct ErrBody<'a> {
            error: &'a str,
        }
        HttpResponse::build(self.status_code()).json(ErrBody {
            error: &self.to_string(),
        })
    }
}
