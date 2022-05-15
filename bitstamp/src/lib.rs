pub mod error;

use futures::StreamExt;
use std::pin::Pin;

pub use error::BitstampError as Error;
pub use error::Context;
pub mod subscribe;
use futures::Stream;
use model::ChannelType;
use model::CurrencyPair;
use model::Message;
pub use subscribe::subscribe;

pub type Result<T> = std::result::Result<T, Error>;
pub mod model;
pub use crate::model::OrderBookData;

/// A stream of bitstamp OrderBookData
pub async fn bitstamp_detail_market_depth_stream(
    instrument: CurrencyPair,
) -> Result<Pin<Box<dyn Stream<Item = Result<OrderBookData>> + Send + 'static>>> {
    // TODO: One day, support more types of streams (other than DetailOrderBook)
    let stream = subscribe(ChannelType::DetailOrderBook, instrument)
        .await?
        // Filter all the incoming messages, because we only care about OrderBookData
        .filter_map(|result| async move {
            result
                .map(|message| match message {
                    Message::Data { data } => Some(data),
                    Message::SubscriptionSucceeded { channel } => {
                        log::info!("Subscribed to {channel:?}");
                        None
                    }
                    Message::Error { data } => {
                        log::error!("Bitstamp server error returned: {data:?}");
                        None
                    }
                    other => {
                        log::warn!("Unexpected message: {other:?}");
                        None
                    }
                })
                .transpose()
        });
    Ok(Box::pin(stream))
}

#[cfg(test)]
mod web_test {
    use futures::StreamExt;

    use crate::model::CurrencyPair;

    #[tokio::test]
    async fn test_orderbook_stream() {
        pretty_env_logger::try_init().ok();
        // Test connect
        let mut book = super::bitstamp_detail_market_depth_stream(CurrencyPair::Ethbtc)
            .await
            .unwrap();

        // Test getting messages
        log::debug!("Listening");
        if let Some(next) = book.next().await {
            let next = next.unwrap();
            log::info!("Here's your message {next:?}");
        }
    }
}
