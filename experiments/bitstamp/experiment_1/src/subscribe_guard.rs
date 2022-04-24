//! Implements a guard so you don't forget to unsubscribe
//! I'm abandoning this because you can't have AsyncDrop

use futures::{Sink, SinkExt};

use crate::{
    model::{ChannelType, CurrencyPair, Message},
    Context, Result,
};
use tokio_tungstenite::tungstenite::Message as TMessage;

pub struct SubscribeGuard<S> {
    client: S,
    channel_type: ChannelType,
    pair: CurrencyPair,
}

impl<S> SubscribeGuard<S>
where
    S: Sink<TMessage> + SinkExt<TMessage> + Unpin,
    <S as Sink<TMessage>>::Error: std::error::Error + 'static,
{
    pub async fn new(
        mut client: S,
        channel_type: ChannelType,
        pair: CurrencyPair,
    ) -> Result<SubscribeGuard<S>> {
        let message = Message::subscribe(channel_type, pair)?;
        client
            .send(message.clone())
            .await
            .message_context(message, "Sending subscribe message")?;
        Ok(SubscribeGuard {
            client,
            channel_type,
            pair,
        })
    }
}

impl<S> Drop for SubscribeGuard<S>
where
    S: Sink<TMessage> + SinkExt<TMessage> + Unpin,
{
    fn drop(&mut self) {
        let message = Message::unsubscribe(self.channel_type, self.pair).unwrap();
        client.send(message.clone()).await.ok();
    }
}
