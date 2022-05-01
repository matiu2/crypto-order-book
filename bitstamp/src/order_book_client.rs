//! A complete order_book subscribing client
//!
//! I'd really like for AsyncDrop trait to exist, but it doesn't so here's my work around
use std::task::Poll;

use futures::{SinkExt, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::{
    error::Context,
    model::{ChannelType, CurrencyPair, Message},
    Result,
};

/// Client to receive messages from bitstamp
pub struct Client {
    client: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    /// Returns a connected and subscribed order book stream
    pub async fn new(channel_type: ChannelType, currency_pair: CurrencyPair) -> Result<Client> {
        // Connect
        log::debug!("Building websocket");
        let (mut client, _response) = connect_async("wss://ws.bitstamp.net/")
            .await
            .context("Connecting")?;
        // Subscribe
        let subscribe = Message::subscribe(channel_type, currency_pair)?;
        client
            .send(subscribe.clone())
            .await
            .message_context(subscribe, "Sending subscribe message")?;
        Ok(Client { client })
    }
}

impl Stream for Client {
    type Item = Result<Message>;

    /// Returns the next message, or an error.
    /// If it returns Some(None), that means that's the end of the stream; you
    /// should `unsubscribe()` and drop this instance
    ///
    /// TODO: Return an enum, with whatever data you subscribed for. Currently
    /// only OrderBookData is implemented
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let message = self.client.poll_next_unpin(cx);
        match message {
            std::task::Poll::Ready(option) => Poll::Ready(option.map(|result| {
                result
                    .context("Reading Reading message") // Error message context
                    .and_then(|message| message.try_into())
            })),
            std::task::Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod web_test {
    use crate::model::{ChannelType, CurrencyPair};
    use futures::StreamExt;

    use super::Client;

    #[tokio::test]
    async fn test_everything() {
        // Test connect
        let mut book = Client::new(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc)
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
