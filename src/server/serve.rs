use anyhow::Result;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};
use tokio::net::TcpListener;

use crate::router::Router;

pub struct Serve {
    listener: TcpListener,
    router: Router,
}

impl Serve {
    pub fn new(tcp_listener: TcpListener, router: Router) -> Self {
        Self {
            listener: tcp_listener,
            router,
        }
    }
}

impl IntoFuture for Serve {
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        return Box::pin(async move {
            loop {
                let (stream, _) = self.listener.accept().await?;
                let router = self.router.clone();
                tokio::task::spawn(async move {
                    auto::Builder::new(TokioExecutor::new())
                        .serve_connection(TokioIo::new(stream), router)
                        .await
                        .unwrap();
                });
            }
        });
    }
}
