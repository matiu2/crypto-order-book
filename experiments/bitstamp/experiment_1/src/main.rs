mod model;
use anyhow::Result;
use futures::StreamExt;
use serde_json::from_str;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::debug!("Building websocket");
    /*
       let (mut client, _response) = connect_async("wss://ws.bitstamp.net/").await?;
       // Listening
       log::debug!("Listening");

       while let Some(next) = client.next().await {
           match next {
               Err(err) => {
                   log::error!("Error receiving message: {err}");
                   break;
               }
               Ok(Message::Text(j)) => {
                   log::debug!("text message: {j}");
                   let depth: Depth = from_str(&j)?;
                   dbg!(depth);
               }
               Ok(other) => {
                   log::error!("Unexpected message type received: {other:?}");
                   break;
               }
           }
       }
    */
    Ok(())
}
