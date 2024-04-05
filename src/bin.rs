use std::{
    fmt::{Write},
    net::SocketAddr,
    println,
};

use anyhow::Result;
use axtel::{
    http::{request::Path, response::IntoResponse},
    json::Json,
    router::{method_router::get, Router},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    nick_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Test {
    hello: String,
}

async fn hello() -> impl IntoResponse {
    "hello"
}

async fn complex() -> impl IntoResponse {
    "hello from other"
}

async fn path(Path(path): Path) -> impl IntoResponse {
    path
}

async fn create_user() -> impl IntoResponse {
    return Json(User {
        name: "foo".to_string(),
        nick_name: "bar".to_string(),
    });
}

async fn print_json(Json(data): Json<Test>) -> impl IntoResponse {
    dbg!(data);
}

async fn html() -> impl IntoResponse {
    tokio::fs::read_to_string("index.html").await.unwrap()
}

async fn date() -> impl IntoResponse {
    let mut date = String::new();
    write!(date, "{:?}", std::time::SystemTime::now()).unwrap();
    return date;
}

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route("/", get(hello))
        .route("/path", get(path))
        .route("/index.html", get(html))
        .route("/user", get(create_user))
        .route("/json", get(print_json))
        .route("/date", get(date))
        .route("/complex", get(complex));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on: {}", addr);
    println!("routes: {}", router.clone());
    axtel::server::serve(listener, router).await?;
    Ok(())
}
