mod error;
pub use error::BitstampError as Error;
pub use error::Context;
use tokio_tungstenite::connect_async;

pub type Result<T> = std::result::Result<T, Error>;
mod model;
use crate::model::{ChannelType, CurrencyPair, Message};
// TODO: mod subscribe_guard;
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
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

    let mut count = 0;
    while let Some(next) = client.next().await {
        match next {
            Ok(data) => {
                dbg!(data);
            }
            Err(err) => {
                dbg!(err);
            }
        };
        count += 1;
        if count >= 5 {
            break;
        }
    }

    // Unsubscribe
    let unsubscribe = Message::unsubscribe(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc)?;
    client
        .send(unsubscribe.clone())
        .await
        .message_context(unsubscribe, "Unsubscribe")?;
    Ok(())
}
