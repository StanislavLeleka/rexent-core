use crate::{routes_collection::RoutesCollection, token_validator::is_request_allowed};
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};

use futures::Future;
use once_cell::sync::Lazy;
use std::{pin::Pin, sync::Mutex};

pub static UNAUTHORIZED_ROUTES: Lazy<Mutex<RoutesCollection>> =
    Lazy::new(|| Mutex::new(RoutesCollection::new()));

pub static S2S_ROUTES: Lazy<Mutex<RoutesCollection>> =
    Lazy::new(|| Mutex::new(RoutesCollection::new()));

pub struct AuthenticationMiddleware<S> {
    pub(crate) service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let is_allowed = is_request_allowed(&req);

        if is_allowed {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?.map_into_left_body();
                Ok(res)
            })
        } else {
            Box::pin(async move {
                let (req, _res) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                let srv = ServiceResponse::new(req, res);
                Ok(srv)
            })
        }
    }
}
