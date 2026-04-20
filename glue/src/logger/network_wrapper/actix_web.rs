// Logging middleware

use actix_web::dev::{Service, Transform};
use actix_web::{Error, dev::ServiceRequest, dev::ServiceResponse};
use futures_util::future::{Ready, ok};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ActixLogger;

impl<S, B> Transform<S, ServiceRequest> for ActixLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware { service })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}
impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let req_info = format!(
                "{} {} {}",
                res.request().method(),
                res.request().uri(),
                res.status().as_str()
            );
            tracing::info!("Request: {}", req_info);
            Ok(res)
        })
    }
}
