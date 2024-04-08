use crate::http::request::Request;
use crate::http::response::Response;
use anyhow::Result;
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use hyper_util::service::TowerToHyperService;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};
use tokio::net::TcpListener;
use tower::Service;

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
    S: Service<Request<Incoming>, Response = Response> + Send + Sync + 'static,
    S::Future: 'static + Send,
    S::Error: Into<Box<dyn Error + Send + Sync>>,
{
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        return Box::pin(async move {
            let service = TowerToHyperService::new(ArcWrapper::new(self.service));
            loop {
                let (stream, _) = self.listener.accept().await?;
                let service = service.clone();
                tokio::task::spawn(async move {
                    match auto::Builder::new(TokioExecutor::new())
                        .serve_connection(TokioIo::new(stream), service)
                        .await
                    {
                        Ok(()) => (),
                        Err(err) => {
                            eprintln!("encounterd an error: {}", err);
                        }
                    }
                })
                .await?;
            }
        });
    }
}

// somehow avoid wrapping all the middleware in a mutex? copying would be easier... but not all
// services implement Clone
struct ArcWrapper<T>(pub Arc<Mutex<T>>);

impl<T> ArcWrapper<T> {
    pub fn new(data: T) -> Self {
        ArcWrapper(Arc::new(Mutex::new(data)))
    }
}

impl<T> Clone for ArcWrapper<T> {
    fn clone(&self) -> Self {
        ArcWrapper(self.0.clone())
    }
}

impl<T> Service<Request<Incoming>> for ArcWrapper<T>
where
    T: Service<Request<Incoming>>,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.0.lock().unwrap().poll_ready(cx)
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        self.0.lock().unwrap().call(req)
    }
}
