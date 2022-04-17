mod id_ticker;
mod model;
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use id_ticker::Ids;
use serde_json::to_string;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::debug!("Building websocket");
    let (mut client, response) =
        connect_async("wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms").await?;
    //assert!(response.status().is_success(), "{response:?}");
    /*
    let mut ids = Ids::new();
    log::debug!("Creating subscribe message");
    // See [binance reference](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#individual-symbol-book-ticker-streams)
    let subscribe = model::Message::subscribe("ETHBTC@bookTicker", ids.next());
    let subscribe = to_string(&subscribe)?;
    log::debug!("Sending subscribe message");
    client.send(Message::Text(subscribe)).await?;
    */
    // Listening
    log::debug!("Listening");

    while let Some(next) = client.next().await {
        match next {
            Err(err) => {
                log::error!("Error receiving message: {err}");
                break;
            }
            Ok(Message::Text(j)) => {
                log::info!("text message: {j}")
            }
            Ok(other) => {
                log::error!("Unexpected message type received: {other:?}");
                break;
            }
        }
    }

    //let unsubscribe = model::Message::unsubscribe("ETHBTC@bookTicker", ids.next());
    Ok(())
}
