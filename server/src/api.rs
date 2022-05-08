tonic::include_proto!("orderbook");

// TODO: send this back to test land, once main is working
//#[cfg(test)]
pub mod test_util {

    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use futures::Future;
    use tonic::{codegen::futures_core::Stream, Response, Status};

    use super::*;

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

    impl Stream for Simple {
        type Item = Result<Summary, Status>;

        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.single.take().map(|s| Ok(s)))
        }
    }

    #[tonic::async_trait]
    impl orderbook_aggregator_server::OrderbookAggregator for Simple {
        type BookSummaryStream = Simple;

        fn book_summary<'life0, 'async_trait>(
            &'life0 self,
            _request: tonic::Request<self::Empty>,
        ) -> core::pin::Pin<
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
}
