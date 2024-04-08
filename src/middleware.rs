use hyper_util::service::TowerToHyperService;
use tower::{
    layer::util::{Identity, Stack},
    Layer,
};

use crate::router::{method_router::Route, Router};

pub struct Middleware<L> {
    pub(crate) layer: L,
    pub(crate) router: Router,
}

impl Middleware<Identity> {
    pub fn new(router: Router) -> Self {
        Self {
            layer: Identity::new(),
            router,
        }
    }
}

impl<L> Middleware<L> {
    pub fn layer<T>(self, layer: T) -> Middleware<Stack<T, L>> {
        Middleware {
            layer: Stack::new(layer, self.layer),
            router: self.router,
        }
    }

    ///adds a route to the router see ['Router::route'](Router) for more info
    pub fn route(mut self, path: &str, route: (Route, hyper::http::Method)) -> Self {
        self.router = self.router.route(path, route);
        return self;
    }

    pub fn service(self) -> L::Service
    where
        L: Layer<Router>,
    {
        self.layer.layer(self.router)
    }
}
