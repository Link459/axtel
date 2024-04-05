use hyper::http;
use hyper::http::StatusCode;

pub type Response<T = String> = http::Response<T>;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for () {
    fn into_response(self) -> Response {
        http::Response::builder()
            .status(StatusCode::OK)
            .body(String::new())
            .unwrap()
    }
}

impl<T> IntoResponse for Result<T, StatusCode>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(t) => return t.into_response(),
            Err(e) => {
                return http::Response::builder()
                    .status(e)
                    .body(String::new())
                    .unwrap()
            }
        }
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(t) => return t.into_response(),
            Err(e) => return e.into_response(),
        }
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        return http::Response::builder()
            .status(StatusCode::OK)
            .body(self)
            .unwrap();
    }
}

impl IntoResponse for &str {
    fn into_response(self) -> Response {
        return http::Response::builder()
            .status(StatusCode::OK)
            .body(self.to_string())
            .unwrap();
    }
}

impl IntoResponse for anyhow::Error {
    fn into_response(self) -> Response {
        return http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(self.to_string())
            .unwrap();
    }
}
