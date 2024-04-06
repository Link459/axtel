use anyhow::Result;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use std::error::Error;
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};
use tokio::net::TcpListener;

use crate::http::request::Request;
use crate::http::response::Response;

pub struct Serve<S> {
    listener: TcpListener,
    service: S,
}

impl<S> Serve<S> {
    pub fn new(listener: TcpListener, service: S) -> Self {
        Self { listener, service }
    }
}

impl<S> IntoFuture for Serve<S>
where
    S: Service<Request<Incoming>, Response = Response> + Clone + Send + 'static,
    S::Future: 'static + Send,
    S::Error: Into<Box<dyn Error + Send + Sync>>,
{
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        return Box::pin(async move {
            loop {
                let (stream, _) = self.listener.accept().await?;
                let router = self.service.clone();
                tokio::task::spawn(async move {
                    match auto::Builder::new(TokioExecutor::new())
                        .serve_connection(TokioIo::new(stream), router)
                        .await
                    {
                        Ok(()) => (),
                        Err(err) => {
                            eprintln!("encounterd an error: {}", err);
                        }
                    }
                });
            }
        });
    }
}
