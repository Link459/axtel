use crate::router::handler::{Handler, IntoHandler};
use hyper::http;
use std::sync::Arc;

#[derive(Clone)]
pub struct Route(pub Arc<dyn Handler>);

macro_rules! impl_method_router_methods {
    ($name:ident,$upper:ident) => {
        pub fn $name<T, U>(handler: T) -> (Route, http::Method)
        where
            T: IntoHandler<U> + 'static,
            U: 'static,
        {
            //let mut router = MethodRouter::new();
            let route = Arc::new(handler.into_handler());
            return (Route(route), hyper::http::Method::$upper);
        }
    };
}

impl_method_router_methods!(get, GET);
impl_method_router_methods!(head, HEAD);
impl_method_router_methods!(post, POST);
impl_method_router_methods!(put, PUT);
impl_method_router_methods!(delete, DELETE);
impl_method_router_methods!(connect, CONNECT);
impl_method_router_methods!(options, OPTIONS);
impl_method_router_methods!(trace, TRACE);
impl_method_router_methods!(patch, PATCH);
