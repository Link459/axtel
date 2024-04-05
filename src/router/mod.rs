pub mod handler;
pub mod method_router;

use std::fmt;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use self::method_router::Route;
use crate::http::{request::Request, response::Response};
use anyhow::Result;
use http_body_util::BodyExt;
use hyper::{body::Incoming, http, service::Service, StatusCode};

#[derive(Clone)]
pub struct Router {
    router: Arc<InnerRouter>,
}

impl Router {
    pub fn new() -> Self {
        return Self {
            router: Arc::new(InnerRouter::new()),
        };
    }

    pub fn route(mut self, path: &str, route: (Route, http::Method)) -> Self {
        Arc::make_mut(&mut self.router).route(path, route);
        return self;
    }
}

impl Default for Router {
    fn default() -> Self {
        return Self::new();
    }
}

async fn incoming_to_string(req: Request<Incoming>) -> Result<Request> {
    let (parts, body) = req.into_parts();
    let bytes = body.collect().await?.to_bytes().to_vec();
    let body = String::from_utf8(bytes)?;
    let req = Request::from_parts(parts, body);
    return Ok(req);
}

impl Service<Request<Incoming>> for Router {
    type Response = Response<String>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        let router = self.clone();
        return Box::pin(async move {
            let request = incoming_to_string(request).await?;
            router.router.handle_request(request).await
        });
    }
}

impl fmt::Display for Router {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for k in self.router.routes.keys() {
            writeln!(f, "method: {},path: {}", k.0, k.1).unwrap();
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct InnerRouter {
    routes: HashMap<(http::Method, String), Route>,
}

impl InnerRouter {
    pub fn new() -> Self {
        return Self {
            routes: HashMap::new(),
        };
    }

    pub fn route(&mut self, path: &str, route: (Route, http::Method)) -> () {
        self.routes.insert((route.1, path.to_string()), route.0);
    }

    pub(crate) async fn handle_request(&self, request: Request) -> Result<Response> {
        //TODO: implement this
        let Some(route) =  self
            .routes
            .get(&(request.method().clone(), request.uri().path().to_string())) else {
                return Ok(hyper::Response::builder().status(StatusCode::NOT_FOUND).body(String::new())?);
            };
        let res = route.0.call(request).await;

        return Ok(res);
    }
}
