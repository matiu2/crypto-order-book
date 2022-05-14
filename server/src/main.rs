use anyhow::Result;
use bitstamp::model::CurrencyPair;
use server::{serve, SummaryServer};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8000".parse().unwrap();
    let service = SummaryServer::new(CurrencyPair::Ethbtc);

    serve(addr, service).await
}
