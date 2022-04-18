mod model;
// TODO: mod subscribe_guard;
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde_json::to_string;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::model::{CurrencyPair, Request};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::debug!("Building websocket");
    let (mut client, _response) = connect_async("wss://ws.bitstamp.net/").await?;
    // Subscribe
    let subscribe = Request::subscribe_to_orders(CurrencyPair::Ethbtc);
    let subscribe = to_string(&subscribe).expect("Unable to serialize the subscribe message");
    client.send(Message::Text(subscribe)).await;

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
    let unsubscribe = Request::unsubscribe_from_orders(CurrencyPair::Ethbtc);
    let unsubscribe = to_string(&unsubscribe).expect("Unable to serialize the subscribe message");
    client.send(Message::Text(unsubscribe)).await;

    Ok(())
}
