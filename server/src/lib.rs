use anyhow::Result;
use std::net::SocketAddr;

use api::orderbook_aggregator_server::OrderbookAggregator;
use tonic::transport::Server;

pub mod api;
pub mod model;

pub async fn serve<S>(addr: SocketAddr, service: S) -> Result<()>
where
    S: OrderbookAggregator + Send + Sync + 'static,
{
    println!("Orderbook server listening on {:?}", addr);

    let service = api::orderbook_aggregator_server::OrderbookAggregatorServer::new(service)
        .send_gzip()
        .accept_gzip();

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
