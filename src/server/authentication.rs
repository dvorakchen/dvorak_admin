//! Authentication of user
//! included all of authentication ability
//!
//! - login logout functions
//! - issues authentication token to cookie or get authentication token from cookie
//! actix middleware for check is logged in
//!
//! # example
//! // enable Authentication middleware
//! ```
//! HttpServer::new(move || {
//!     App::new()
//!         .wrap(Authentication)
//! })
//! .bind(&addr)?
//! .run()
//! .await
//! ```
//!
//!

mod authenticated;

pub use authenticated::*;

use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::server::AppDataCipher;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{self, ACCEPT},
    Error, HttpMessage, HttpResponse,
};
use base64::prelude::*;
use futures_util::future::{FutureExt, LocalBoxFuture};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        async move {
            if is_not_login_page(&req) {
                let cipher = req.extract::<AppDataCipher>().await.unwrap();
                if let Some(authenticate_token) = is_logged_in(&req, cipher) {
                    req.extensions_mut()
                        .insert::<RequestAuthenticationToken>(Rc::new(authenticate_token));
                } else {
                    let (request, _) = req.into_parts();
                    let resp = HttpResponse::Found()
                        .insert_header((header::LOCATION, "/login"))
                        .finish()
                        .map_into_right_body();

                    return Ok(ServiceResponse::new(request, resp));
                }
            }

            service
                .call(req)
                .await
                .map(ServiceResponse::map_into_left_body)
        }
        .boxed_local()
    }
}

fn is_not_login_page(req: &ServiceRequest) -> bool {
    if let Some(accept) = req.headers().get(ACCEPT) {
        if let Ok(value) = accept.to_str() {
            if value.find("text/html").is_some() && req.path() != "/login" {
                return true;
            }
        }
    }

    false
}

fn is_logged_in(req: &ServiceRequest, cipher: AppDataCipher) -> Option<AuthenticationToken> {
    if let Some(cookie) = req.cookie(LOGIN_COOKIE_NAME) {
        let mut cipher = cipher.lock().unwrap();
        let decrypted = BASE64_STANDARD.decode(cookie.value().as_bytes()).ok()?;
        if let Ok(decrypted) = cipher.decrypt(&decrypted.to_vec()) {
            let decrypted = String::from_utf8(decrypted).unwrap();
            return AuthenticationToken::from_json(&decrypted);
        }
    }
    None
}

/// only used in cookie name
pub const LOGIN_COOKIE_NAME: &'static str = "LOGIN";
