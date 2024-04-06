use std::{fmt::Write, net::SocketAddr, println, time::Duration};

use anyhow::Result;
use axtel::{
    http::{
        request::{Path, Request},
        response::IntoResponse,
    },
    json::Json,
    router::{method_router::get, Router},
};
use hyper::body::Incoming;
use hyper_util::service::TowerToHyperService;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;

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

async fn loop_inf() -> impl IntoResponse {
    tokio::time::sleep(Duration::new(10, 0)).await;
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
        .route("/loop", get(loop_inf))
        .route("/complex", get(complex));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on: {}", addr);
    println!("routes: {}", router.clone());
    let svc = ServiceBuilder::new()
        .timeout(Duration::new(11, 0))
        .service(router);
    let svc = TowerToHyperService::new(svc);
    axtel::server::serve(listener, svc).await?;
    Ok(())
}
