//! A complete order_book subscribing client
//!
//! I'd really like for AsyncDrop trait to exist, but it doesn't so here's my work around
use futures::SinkExt;
use tokio::{net::TcpStream, runtime::Handle};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::{
    error::Context,
    model::{ChannelType, CurrencyPair, Message},
    Result,
};

pub struct OrderBookClient {
    channel_type: ChannelType,
    currency_pair: CurrencyPair,
    client: WebSocketStream<MaybeTlsStream<TcpStream>>,
    // True once we've unsubscribed
    unsubscribed: bool,
}

impl OrderBookClient {
    pub async fn new(
        channel_type: ChannelType,
        currency_pair: CurrencyPair,
    ) -> Result<OrderBookClient> {
        // Connect
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
        Ok(OrderBookClient {
            channel_type,
            currency_pair,
            client,
            unsubscribed: false,
        })
    }

    /// Call this before dropping this instance. If you don't it'll be called on, drop, but the whole async thread will freeze until it completes
    pub async fn unsubscribe(&mut self) -> Result<()> {
        let message = Message::unsubscribe(self.channel_type, self.currency_pair)?;
        self.client
            .send(message.clone())
            .await
            .message_context(message, "Unsubscribe")
        self.unsubscribed = true;
    }
}

impl Drop for OrderBookClient {
    fn drop(&mut self) {
        // If nobody unsubscribed; do it now, but the penalty is that the whole tokio thread freezes until it finishes
        let handle = Handle::current();
        let result = handle.block_on(self.unsubscribe());
        if let Err(err) = result {
            log::error!("Error unsubscribing from OrderbookClient: {err:?}");
        }
    }
}


#[cfg(test)]
mod web_test {

    #[tokio::test]
    async fn test_unsubscribe() {

    }
}