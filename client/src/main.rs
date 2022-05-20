use client::api::{orderbook_aggregator_client::OrderbookAggregatorClient, Empty};
use tonic::Request;

#[tokio::main]
async fn main() {
    let mut client = OrderbookAggregatorClient::connect("ws://127.0.0.1:8000")
        .await
        .expect("connect");
    let mut stream = client
        .book_summary(Request::new(Empty {}))
        .await
        .expect("Getting stream")
        .into_inner();
    while let Some(summary) = stream.message().await.expect("next message") {
        println!("{summary:?}");
    }
}
