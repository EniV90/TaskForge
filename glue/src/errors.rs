use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[cfg(feature = "actix")]
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};

#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum SchedulerServiceErrorStatus {
    #[error("Requested resource was not found")]
    NotFound,
    #[error("You are forbidden to access requested resource")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown,
    #[error("Bad Request")]
    BadRequest,
    #[error("Conflict")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Debug, Deserialize, Serialize, Error)]
pub struct SchedulerServiceError {
    pub message: String,
    pub status: SchedulerServiceErrorStatus,
}

impl SchedulerServiceError {
    pub fn new(message: String, status: SchedulerServiceErrorStatus) -> SchedulerServiceError {
        SchedulerServiceError { message, status }
    }
}

impl fmt::Display for SchedulerServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "actix")]
impl ResponseError for SchedulerServiceError {
    fn status_code(&self) -> StatusCode {
        match self.status {
            SchedulerServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
            SchedulerServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
            SchedulerServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            SchedulerServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
            SchedulerServiceErrorStatus::Conflict => StatusCode::CONFLICT,
            SchedulerServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.message.clone())
    }
}

#[macro_export]
macro_rules! safe_eject {
    ($e:expr, $err_status:expr) => {
        $e.map_err(|x| SchedulerServiceError::new(x.to_string(), $err_status))
    };
    ($e:expr, $err_status:expr, $message_context:expr) => {
        $e.map_err(|x| {
            SchedulerServiceError::new(
                format!("{}: {}", $message_context, x.to_string()),
                $err_status,
            )
        })
    };
}
