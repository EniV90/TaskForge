// Defining Header token modules

use crate::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HeaderToken {
    pub unique_id: String,
}

impl HeaderToken {
    pub fn get_key() -> Result<String, SchedulerServiceError> {
        std::env::var("JWT_SECRET").map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unauthorized)
        })
    }

    pub fn encode(self) -> Result<String, SchedulerServiceError> {
        let key_str = Self::get_key()?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        return match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(error) => Err(SchedulerServiceError::new(
                error.to_string(),
                SchedulerServiceErrorStatus::Unauthorized,
            )),
        };
    }

    pub fn decode(token: &str) -> Result<Self, SchedulerServiceError> {
        let key_str = Self::get_key()?;
        let key = DecodingKey::from_secret(key_str.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.required_spec_claims.remove("exp");

        match decode::<Self>(token, &key, &validation) {
            Ok(token_data) => return Ok(token_data.claims),
            Err(error) => {
                return Err(SchedulerServiceError::new(
                    error.to_string(),
                    SchedulerServiceErrorStatus::Unauthorized,
                ));
            }
        }
    }
}
#[cfg(feature = "actix")]
mod actix_impl {
    use super::HeaderToken;
    pub use actix_web::{FromRequest as ActixFromRequest, HttpRequest, dev::Payload};

    use crate::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
    use futures::future::{Ready, err, ok};

    // Implementing Actix Header extraction trait
    impl ActixFromRequest for HeaderToken {
        type Error = SchedulerServiceError;
        type Future = Ready<Result<HeaderToken, SchedulerServiceError>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("token") {
                Some(data) => data.to_str().expect("convert token to sting"),
                None => {
                    return err(SchedulerServiceError {
                        status: SchedulerServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token' ".to_string(),
                    });
                }
            };
            // Convert the data to a string
            let token = match HeaderToken::decode(raw_data) {
                Ok(token) => token,
                Err(_) => {
                    return err(SchedulerServiceError {
                        status: SchedulerServiceErrorStatus::Unauthorized,
                        message: "token not a valid string".to_string(),
                    });
                }
            };
            ok(token)
        }
    }
}

#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;
