use anyhow::Result;
use bitstamp::model::CurrencyPair;
use server::{serve, SummaryServer};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let addr = "127.0.0.1:8000".parse().unwrap();
    let service = SummaryServer::new(CurrencyPair::Ethbtc);

    serve(addr, service).await
}

#[cfg(test)]
mod web_test {
    use bitstamp::model::CurrencyPair;
    use server::{
        api::{orderbook_aggregator_client::OrderbookAggregatorClient, Empty},
        SummaryServer,
    };
    use tokio::spawn;

    #[tokio::test]
    async fn test_live_stream() {
        pretty_env_logger::try_init().ok();
        let addr = "127.0.0.1:8000".parse().unwrap();
        let service = SummaryServer::new(CurrencyPair::Ethbtc);
        let _server = spawn(crate::serve(addr, service));
        let client = spawn(async move {
            // Connect to the server and recieve one message
            log::info!("Client connecting");
            let mut client = OrderbookAggregatorClient::connect("http://127.0.0.1:8000")
                .await
                .unwrap();

            log::info!("Client calling book_summary");
            let mut s = client
                .book_summary(tonic::Request::new(Empty {}))
                .await
                .unwrap()
                .into_inner();

            // Get one orderbook
            log::info!("Getting one orderbook from the stream");
            let msg1 = s.message().await.unwrap();

            log::info!("Here's your message: {:?}", msg1);
        });
        log::info!("Waiting for client");
        client.await.unwrap();
        log::info!("Done");
    }
}
