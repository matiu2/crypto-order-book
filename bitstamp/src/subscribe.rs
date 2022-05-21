//! Subscribe to bitstamp order_book stream

use futures::{SinkExt, Stream, StreamExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage};

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

    // Spawn a task that can respond to pings, and forward relevant messages to our queue
    let (out_send, out_recv) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        while let Some(result) = client.next().await {
            let result = result.context("Receiving message");
            match result {
                Ok(TMessage::Ping(data)) => {
                    log::info!("Ping: {data:?}");
                    if let Err(err) = client.send(TMessage::Pong(data)).await {
                        log::error!("Unable to bitstamp pong: {err:?}")
                    }
                }
                result @ Ok(TMessage::Text(_)) => {
                    if let Err(err) = out_send.send(result.and_then(|tmsg| tmsg.try_into())) {
                        // Most likely the client has disconnected
                        log::error!("Unable to forward message to client: {err:?}");
                        return;
                    }
                }
                Err(err) => {
                    if let Err(err) = out_send.send(Err(err)) {
                        // Most likely the client has disconnected
                        log::error!("Unable to forward error to client: {err:?}");
                        return;
                    }
                }
                _ => unreachable!("We didn't expect a message of this type: {result:?}"),
            }
        }
    });

    Ok(UnboundedReceiverStream::new(out_recv))
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
