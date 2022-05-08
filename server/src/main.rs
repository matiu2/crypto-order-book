use anyhow::Result;
use server::{api::test_util::Simple, serve};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8000".parse().unwrap();
    let service = Simple::new();

    serve(addr, service).await
}

#[cfg(test)]
mod self_test {

    use server::api::{orderbook_aggregator_client::OrderbookAggregatorClient, test_util::Simple};
    use tokio::spawn;

    #[tokio::test]
    async fn test_simple_server() -> anyhow::Result<()> {
        // Make the server
        let addr = "0.0.0.0:8000".parse().unwrap();
        let simple = Simple::new();
        let server = tokio::spawn(super::serve(addr, simple));
        dbg!(&server);

        let client = spawn(async move {
            // Connect to the server and recieve one message
            let mut client = OrderbookAggregatorClient::connect("http://127.0.0.1:8000")
                .await
                .unwrap();

            let mut s = client
                .book_summary(tonic::Request::new(server::api::Empty {}))
                .await
                .unwrap()
                .into_inner();

            let msg1 = s.message().await.unwrap();
            dbg!(msg1);

            // The second message should be none
            let msg2 = s.message().await.unwrap();
            assert_eq!(msg2, None);
            dbg!(msg2)
        });

        println!("Waiting for client");
        client.await.unwrap();
        println!("Done");
        Ok(())
    }
}
