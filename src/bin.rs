use std::{fmt::Write, net::SocketAddr, println, task::Context, task::Poll, time::Duration};

use anyhow::Result;
use axtel::{
    http::{request::Path, response::IntoResponse},
    json::Json,
    router::{method_router::get, Router},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::{
    limit::{RateLimit, RateLimitLayer},
    timeout::TimeoutLayer,
    Layer, Service,
};

pub struct LogLayer {
    target: &'static str,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service,
        }
    }
}

// This service implements the Log behavior
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}

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

async fn empty() -> () {}

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route("/", get(empty))
        .route("/hello", get(hello))
        .route("/path", get(path))
        .route("/index.html", get(html))
        .route("/user", get(create_user))
        .route("/json", get(print_json))
        .route("/date", get(date))
        .route("/loop", get(loop_inf))
        .route("/complex", get(complex))
        .layer(TimeoutLayer::new(Duration::new(1, 0)))
        .layer(RateLimitLayer::new(100, Duration::new(1, 0)))
        .layer(LogLayer {
            target: "axtel test",
        })
        .service();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on: {}", addr);

    axtel::server::serve(listener, router).await?;
    Ok(())
}
