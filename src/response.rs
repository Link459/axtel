use crate::body::Body;

struct Response {
    status_code: u16,
    body: Body
}

trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}
