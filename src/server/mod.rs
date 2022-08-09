use std::net::SocketAddr;

mod routes;

/// Starts the API server
pub async fn start(addr: impl Into<SocketAddr>) {
    warp::serve(routes::make_routes()).run(addr).await;
}
