//! Type-safe authentication information extractors

use actori_web::dev::ServiceRequest;
use actori_web::Error;
use futures::future::Future;

pub mod basic;
pub mod bearer;
mod config;
mod errors;

pub use self::config::AuthExtractorConfig;
pub use self::errors::AuthenticationError;

/// Trait implemented by types that can extract
/// HTTP authentication scheme credentials from the request.
///
/// It is very similar to actori' `FromRequest` trait,
/// except it operates with a `ServiceRequest` struct instead,
/// therefore it can be used in the middlewares.
///
/// You will not need it unless you want to implement your own
/// authentication scheme.
pub trait AuthExtractor: Sized {
    /// The associated error which can be returned.
    type Error: Into<Error>;

    /// Future that resolves into extracted credentials type.
    type Future: Future<Output = Result<Self, Self::Error>>;

    /// Parse the authentication credentials from the actori' `ServiceRequest`.
    fn from_service_request(req: &ServiceRequest) -> Self::Future;
}
