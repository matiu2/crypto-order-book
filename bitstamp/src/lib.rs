pub mod error;
use std::task::Poll;

pub use error::BitstampError as Error;
pub use error::Context;
pub mod order_book_client;
use futures::Stream;
use futures::StreamExt;
use model::ChannelType;
use model::CurrencyPair;
use model::Message;
pub use order_book_client::Client;

pub type Result<T> = std::result::Result<T, Error>;
pub mod model;
pub use crate::model::OrderBookData;

pub struct OrderBookDataStream {
    client: Client,
}

impl Stream for OrderBookDataStream {
    type Item = Result<OrderBookData>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let message = self.client.poll_next_unpin(cx);
        match message {
            Poll::Ready(Some(Ok(Message::Data { data }))) => Poll::Ready(Some(Ok(data))),
            Poll::Ready(Some(Ok(Message::SubscriptionSucceeded { channel: _ }))) => Poll::Pending,
            Poll::Ready(Some(Ok(msg))) => {
                log::warn!("Unexpected message received: {msg:?}");
                Poll::Pending
            }
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A stream of bitstamp OrderBookData
pub async fn bitstamp_detail_market_depth_stream(
    instrument: CurrencyPair,
) -> Result<OrderBookDataStream> {
    // TODO: One day, support more types of streams (other than DetailOrderBook)
    Client::new(ChannelType::DetailOrderBook, instrument)
        .await
        // TOOD: Implement Stream for Client directly - no more wrapper needed
        .map(|client| OrderBookDataStream { client })
}
