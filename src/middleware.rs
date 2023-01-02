use std::future::{Ready, ready};

use actix_web::dev::{ServiceRequest, Service, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, error};
use futures::future::LocalBoxFuture;

pub struct Authentication;
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = AuthenticationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware{service}))
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // let header = req
        //     .headers()
        //     .get("Authorization")
        //     .unwrap() // TODO
        //     .to_str()
        //     .map_err(|e| error::ErrorUnauthorized(e))?;
        Ok(())
    }
}