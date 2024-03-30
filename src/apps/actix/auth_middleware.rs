use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::Future;

use crate::users::{UserIdentity, UserSecurityService, UserService};

pub struct Auth {
    pub user_security_service: Arc<dyn UserSecurityService>,
    pub user_service: Arc<dyn UserService>,
}

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleWare<S>;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(AuthMiddleWare {
            service: Rc::new(RefCell::new(service)),
            user_security_service: self.user_security_service.clone(),
            user_service: self.user_service.clone(),
        })
    }
}

pub struct AuthMiddleWare<S> {
    service: Rc<RefCell<S>>,
    user_security_service: Arc<dyn UserSecurityService>,
    user_service: Arc<dyn UserService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    /* fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    } */

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let user_security_service = self.user_security_service.clone();
        let user_svc = self.user_service.clone();
        let svc = self.service.clone();
        Box::pin(async move {
            let authorization = req
                .headers()
                .get("authorization")
                .map(|v| v.to_str())
                .unwrap_or(Ok(""))
                .unwrap();
            let parts: Vec<&str> = authorization.split(' ').collect();
            if parts.len() != 2 || parts.is_empty() || parts[0].to_lowercase() != "bearer" {
                return Ok(
                    req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body())
                );
            }
            let token = parts[1];
            let payload = user_security_service.decode_token(token).await;
            if payload.is_err() {
                return Ok(
                    req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body())
                );
            }
            let user_boxed = user_svc.find_by_email(&payload.unwrap().email).await;
            if user_boxed.is_err() {
                return Ok(
                    req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body())
                );
            }
            let user = user_boxed.unwrap();
            let identity = UserIdentity {
                email: user.email,
                user_id: user.id,
            };
            req.extensions_mut().insert(identity);
            svc.call(req).await.map(ServiceResponse::map_into_left_body)
        })
    }
}
