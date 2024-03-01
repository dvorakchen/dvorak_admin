use actix_web::{
    body::BoxBody,
    http::StatusCode,
    FromRequest, HttpMessage, HttpResponse, ResponseError,
};
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

use crate::models::User;

#[derive(Debug)]
pub struct AuthenticatedError;

impl ResponseError for AuthenticatedError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Unauthorized().finish()
    }
}

impl Display for AuthenticatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "authenticate fail")
    }
}

/// Authenticated authentication
/// can be used in extractor
/// if current user logged in, this extractor can take [AuthenticationToken], otherwise throw error: [AuthenticatedError]
pub struct Authenticated(RequestAuthenticationToken);

impl FromRequest for Authenticated {
    type Error = AuthenticatedError;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req
            .extensions()
            .get::<RequestAuthenticationToken>()
            .cloned();
        let result = match value {
            Some(v) => Ok(Authenticated(v)),
            None => Err(AuthenticatedError),
        };
        futures::future::ready(result)
    }
}

impl Deref for Authenticated {
    type Target = RequestAuthenticationToken;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type RequestAuthenticationToken = Rc<AuthenticationToken>;

/// Authentication Token
/// indicates current logged in user informations,
/// normally get authenticate token from cookie by middleware
///
/// # example
/// ```
/// // in middleware
/// // req is ServiceRequest
/// if let Some(json) = req.cookie(LOGIN_COOKIE_NAME) {
///         // notice: if encrypted, decrypt first
///        AuthenticationToken::from_json(json)
/// } else {
///     None
/// }
/// ```
pub struct AuthenticationToken {
    /// current user id
    pub id: String,
    /// current username
    pub username: String,
}

impl AuthenticationToken {
    /// build AuthenticationToken from cookie
    /// if cookie is invalid or empty, it returns None
    pub fn from_json(json: &str) -> Option<Self> {
        let user: User = serde_json::from_str(json).ok()?;

        Some(Self {
            id: user.id,
            username: user.username,
        })
    }
}
