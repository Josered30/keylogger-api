use actix_rt::blocking::BlockingError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Deserialize;
use serde_json::json;
use std::{fmt, io};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> Self {
        return ApiError {
            message,
            status_code,
        };
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.write_str(self.message.as_str());
    }
}

impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> Self {
        match error {
            BlockingError::Error(error) => error,
            BlockingError::Canceled => ApiError::new(500, format!("Blocking error: canceled")),
        }
    }
}

impl From<io::Error> for ApiError {
    fn from(error: io::Error) -> Self {
       return ApiError::new(500, format!("IO error {}", error));
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };
        return HttpResponse::build(status_code).json(json!({ "message": error_message }));
    }
}
