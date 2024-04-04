use std::{net::SocketAddr, println};

use anyhow::Result;
use axtel::{
    http::{request::Path, response::IntoResponse},
    router::{method_router::get, Router},
};
use tokio::net::TcpListener;

async fn hello() -> impl IntoResponse {
    "hello"
}

async fn complex() -> impl IntoResponse {
    "hello from other"
}

async fn path(Path(path): Path) -> impl IntoResponse {
    path
}

async fn html() -> impl IntoResponse {
    tokio::fs::read_to_string("index.html").await.unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route("/", get(hello))
        .route("/path", get(path))
        .route("/index.html", get(html))
        .route("/complex", get(complex));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on: {}", addr);

    axtel::server::serve(listener, router).await?;
    Ok(())
}
