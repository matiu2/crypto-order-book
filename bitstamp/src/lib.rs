pub mod error;
pub use error::BitstampError as Error;
pub use error::Context;
pub mod order_book_client;

pub type Result<T> = std::result::Result<T, Error>;
pub mod model;

#[cfg(test)]

mod web_test {
    use crate::{
        error::Context,
        model::{ChannelType, CurrencyPair, Message},
        Result,
    };
    use futures::{SinkExt, StreamExt};
    use tokio_tungstenite::connect_async;

    #[tokio::test]
    async fn test_api() -> Result<()> {
        pretty_env_logger::init();
        log::debug!("Building websocket");
        let (mut client, _response) = connect_async("wss://ws.bitstamp.net/")
            .await
            .context("Connecting")?;

        // Subscribe
        let subscribe = Message::subscribe(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc)?;
        client
            .send(subscribe.clone())
            .await
            .message_context(subscribe, "Sending subscribe message")?;

        log::debug!("Listening");
        if let Some(next) = client.next().await {
            let next = next.unwrap();
        }

        // Unsubscribe
        let unsubscribe = Message::unsubscribe(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc)?;
        client
            .send(unsubscribe.clone())
            .await
            .message_context(unsubscribe, "Unsubscribe")?;
        Ok(())
    }
}
