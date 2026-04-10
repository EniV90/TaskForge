use actix_web::HttpRequest;
use base64::{Engine, engine::general_purpose};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub async fn extrect_credentials(req: HttpRequest) -> Result<Credentials, SchedulerServiceError> {
    let header_value = match req.headers().get("Authorization") {
        Some(auth_header) => auth_header,
        None => {
            return Err(SchedulerServiceError::new(
                "No credentials provided".to_string(),
                SchedulerServiceErrorStatus::Unauthorized,
            ));
        }
    };
    let encoded = match header_value.to_str() {
        Ok(encoded) => encoded,
        Err(_) => {
            return Err(SchedulerServiceError::new(
                "Invalid credentials".to_string(),
                SchedulerServiceErrorStatus::Unauthorized,
            ));
        }
    };

    if !encoded.starts_with("Basic") {
        return Err(SchedulerServiceError::new(
            "Invalid string".to_string(),
            SchedulerServiceErrorStatus::Unauthorized,
        ));
    }

    let base64_credentials = &encoded[6..];
    let decoded = general_purpose::STANDARD
        .decode(base64_credentials)
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unauthorized)
        })?;
    let credentials = String::from_utf8(decoded).map_err(|e| {
        SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unauthorized)
    })?;

    let parts: Vec<&str> = credentials.splitn(2, ":").collect();
    if parts.len() == 2 {
        let email = parts[0];
        let password = parts[1];

        return Ok(Credentials {
            email: email.to_string(),
            password: password.to_string(),
        });
    } else {
        return Err(SchedulerServiceError::new(
            "Invalid credentials".to_string(),
            SchedulerServiceErrorStatus::Unauthorized,
        ));
    }
}
