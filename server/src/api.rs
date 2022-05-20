tonic::include_proto!("orderbook");

#[cfg(test)]
pub mod web_test {

    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use futures::Future;
    use tokio::spawn;
    use tonic::{codegen::futures_core::Stream, Response, Status};

    use super::{
        orderbook_aggregator_client::OrderbookAggregatorClient,
        orderbook_aggregator_server::OrderbookAggregator, Empty, Level, Summary,
    };

    /// Just a Simple server that streams a single summary, then ends
    pub struct Simple {
        single: Option<Summary>,
    }

    impl Simple {
        pub fn new() -> Simple {
            let summary = Summary {
                spread: 1.1,
                bids: vec![Level {
                    exchange: "binance".to_string(),
                    price: 1.1,
                    amount: 50.0,
                }],
                asks: vec![Level {
                    exchange: "bitstamp".to_string(),
                    price: 2.2,
                    amount: 50.0,
                }],
            };
            Simple {
                single: Some(summary),
            }
        }
    }

    impl Default for Simple {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Stream for Simple {
        type Item = Result<Summary, Status>;

        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.single.take().map(Ok))
        }
    }

    #[tonic::async_trait]
    impl OrderbookAggregator for Simple {
        type BookSummaryStream = Simple;

        fn book_summary<'life0, 'async_trait>(
            &'life0 self,
            _request: tonic::Request<Empty>,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<tonic::Response<Self::BookSummaryStream>, tonic::Status>>
                    + core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async { Ok(Response::new(Simple::new())) })
        }
    }

    #[tokio::test]
    async fn test_simple_server() -> anyhow::Result<()> {
        // Make the server
        let addr = "0.0.0.0:8000".parse().unwrap();
        let simple = Simple::new();
        let server = tokio::spawn(crate::serve(addr, simple));
        dbg!(&server);

        let client = spawn(async move {
            // Connect to the server and recieve one message
            let mut client = OrderbookAggregatorClient::connect("http://127.0.0.1:8000")
                .await
                .unwrap();

            let mut s = client
                .book_summary(tonic::Request::new(Empty {}))
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
