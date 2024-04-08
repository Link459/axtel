pub mod serve;

use std::error::Error;

use crate::{
    http::{request::Request, response::Response},
    server::serve::Serve,
};
use hyper::{body::Incoming, service::Service};
use tokio::net::TcpListener;

pub fn serve<S>(listener: TcpListener, service: S) -> Serve<S>
where
    S: tower::Service<Request<Incoming>, Response = Response> + Send + Sync + 'static,
    S::Future: 'static + Send,
    S::Error: Into<Box<dyn Error + Send + Sync>>,
{
    return Serve::new(listener, service);
}
