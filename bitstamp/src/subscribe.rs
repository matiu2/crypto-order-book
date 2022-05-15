//! Subscribe to bitstamp order_book stream

use futures::{SinkExt, Stream, StreamExt};
use tokio_tungstenite::connect_async;

use crate::{
    error::Context,
    model::{ChannelType, CurrencyPair, Message},
    Result,
};

/// Subscribes to the bitstamp websocket and returns a stream of Message results
pub async fn subscribe(
    channel_type: ChannelType,
    currency_pair: CurrencyPair,
) -> Result<impl Stream<Item = Result<Message>>> {
    // Connect
    log::debug!("Building websocket");
    let (mut client, _response) = connect_async("wss://ws.bitstamp.net/")
        .await
        .context("Connecting")?;

    // Subscribe
    let subscribe = Message::subscribe(channel_type, currency_pair)?;
    log::debug!("Sending subscribe message: {:?}", &subscribe);
    client
        .send(subscribe.clone())
        .await
        .message_context(subscribe, "Sending subscribe message")?;

    // Convert the raw json message result into our internal format
    Ok(client.map(|result| {
        result
            // Convert the error to our library type
            .context("Reading message")
            // Convert the raw message into our internal type
            .and_then(|message| message.try_into())
    }))
}

#[cfg(test)]
mod web_test {
    use crate::model::{ChannelType, CurrencyPair};
    use futures::StreamExt;

    use super::subscribe;

    #[tokio::test]
    async fn test_client() {
        pretty_env_logger::try_init().ok();
        // Test connect
        let mut book = subscribe(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc)
            .await
            .unwrap();
        // Test getting messages
        log::debug!("Listening");
        if let Some(next) = book.next().await {
            let next = next.unwrap();
            dbg!(next);
        }
        if let Some(next) = book.next().await {
            let next = next.unwrap();
            dbg!(next);
        }
    }
}
