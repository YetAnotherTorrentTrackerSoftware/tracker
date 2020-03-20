/// Bencode responder
use std::{fmt, ops};

use futures::future::{err, ok, Ready};
use serde::Serialize;

use actix_http::{http::StatusCode, Response, ResponseError};
use actix_web::{Error, HttpRequest, Responder};
use serde::export::Formatter;

/// Wrapper type for bencode response generation
pub struct Bencode<T>(pub T);

impl<T> ops::Deref for Bencode<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Bencode<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> fmt::Debug for Bencode<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bencode: {:?}", self.0)
    }
}

impl<T> fmt::Display for Bencode<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Wrapper around serde_bencode::Error to be able to implement traits for it
///
///
#[derive(Debug)]
struct BencodeErrorWrapper(serde_bencode::Error);

impl fmt::Display for BencodeErrorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ResponseError for BencodeErrorWrapper {}

impl<T: Serialize> Responder for Bencode<T> {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let body = match serde_bencode::to_bytes(&self.0) {
            Ok(body) => body,
            Err(e) => return err(BencodeErrorWrapper(e).into()),
        };

        ok(Response::build(StatusCode::OK)
            .content_type("application/octet-stream")
            .body(body))
    }
}
