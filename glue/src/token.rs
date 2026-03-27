// Defining Header token modules
pub struct HeaderToken {
    pub message: String,
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
                Some(data) => data,
                None => {
                    return err(SchedulerServiceError {
                        status: SchedulerServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token' ".to_string(),
                    });
                }
            };
            // Convert the data to a string
            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return err(SchedulerServiceError {
                        status: SchedulerServiceErrorStatus::Unauthorized,
                        message: "token not a valid string".to_string(),
                    });
                }
            };
            return ok(HeaderToken { message });
        }
    }
}

#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;
