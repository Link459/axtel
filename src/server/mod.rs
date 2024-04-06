pub mod serve;

use crate::server::serve::Serve;
use tokio::net::TcpListener;

pub fn serve<S>(listener: TcpListener, service: S) -> Serve<S> {
    return Serve::new(listener, service);
}
