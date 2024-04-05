use anyhow::Result;
use hyper::http::{self, request::Parts};

pub type Method = http::Method;

impl FromRequestParts for Method {
    fn from_request_parts(parts: &Parts) -> Result<Self> {
        return Ok(parts.method.clone());
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path(pub String);

impl FromRequestParts for Path {
    fn from_request_parts(parts: &Parts) -> Result<Self> {
        return Ok(Path(parts.uri.path().to_string()));
    }
}

pub type Body = String;

impl FromRequest for Body {
    fn from_request(request: Request) -> Result<Self> {
        let (_, body) = request.into_parts();
        return Ok(body);
    }
}

pub type Request<T = Body> = http::Request<T>;

impl FromRequest for Request {
    fn from_request(request: Request) -> Result<Self> {
        return Ok(request);
    }
}

pub trait FromRequest
where
    Self: Sized,
{
    fn from_request(request: Request) -> Result<Self>;
}

pub trait FromRequestParts
where
    Self: Sized,
{
    fn from_request_parts(parts: &Parts) -> Result<Self>;
}

impl<T> FromRequest for T
where
    T: FromRequestParts,
{
    fn from_request(request: Request) -> Result<Self> {
        let (parts, _) = request.into_parts();
        return Self::from_request_parts(&parts);
    }
}
