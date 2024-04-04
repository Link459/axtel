pub mod serve;

use crate::server::serve::Serve;
use tokio::net::TcpListener;

use crate::router::{ Router};

pub fn serve(listener: TcpListener, router: Router) -> Serve {
    return Serve::new(listener, router);
}
