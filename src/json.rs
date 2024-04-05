use anyhow::Result;
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

use crate::http::{
    request::{FromRequest, Request},
    response::{IntoResponse, Response},
};

pub struct Json<T>(pub T);

impl<T> FromRequest for Json<T>
where
    T: DeserializeOwned,
{
    fn from_request(request: Request) -> Result<Self> {
        let body = request.into_body();
        let res = serde_json::from_str::<T>(&body)?;
        return Ok(Json(res));
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let builder = Response::builder();
        match serde_json::to_string(&self.0) {
            Ok(body) => builder
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
            Err(err) => builder
                .status(StatusCode::IM_A_TEAPOT)
                .body(err.to_string())
                .unwrap(),
        }
    }
}
