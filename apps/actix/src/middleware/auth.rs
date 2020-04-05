use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::Future;

use rwebapi_users::{UserSecurityService, UserService};

pub struct Auth {
    pub user_security_service: Arc<Box<dyn UserSecurityService>>,
    pub user_service: Arc<Box<dyn UserService>>,
}

impl<S: 'static, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
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
    user_security_service: Arc<Box<dyn UserSecurityService>>,
    user_service: Arc<Box<dyn UserService>>,
}

impl<S, B> Service for AuthMiddleWare<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let user_security_service = self.user_security_service.clone();
        let user_svc = self.user_service.clone();
        let mut svc = self.service.clone();
        Box::pin(async move {
            let authorization = req
                .headers()
                .get("authorization")
                .map(|v| v.to_str())
                .unwrap_or(Ok(""))
                .unwrap();
            let parts: Vec<&str> = authorization.split(' ').collect();
            if parts.len() != 2 || parts.is_empty() || parts[0].to_lowercase() != "bearer" {
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()));
            }
            let token = parts[1];
            let payload = user_security_service.decode_token(&token).await;
            if payload.is_err() {
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()));
            }
            let user_boxed = user_svc.find_by_email(&payload.unwrap().email).await;
            if user_boxed.is_err() {
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()));
            }
            let user = user_boxed.unwrap();
            let identity = crate::identity::UserIdentity {
                email: user.email,
                user_id: user.id,
            };
            req.head().extensions_mut().insert(identity);
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
