mod error;
pub use error::BitstampError as Error;
pub type Result<T> = std::result::Result<T, Error>;
mod model;
// TODO: mod subscribe_guard;
//use futures::{SinkExt, StreamExt};
//use serde_json::to_string;
//use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::debug!("Building websocket");
    /*
       let (mut client, _response) = connect_async("wss://ws.bitstamp.net/").await?;
       // Subscribe
       let subscribe = Request::subscribe(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc);
       let subscribe = to_string(&subscribe).expect("Unable to serialize the subscribe message");
       client
           .send(Message::Text(subscribe))
           .await
           .expect("Unable to subscribe");

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
       let unsubscribe = Request::unsubscribe(ChannelType::DetailOrderBook, CurrencyPair::Ethbtc);
       let unsubscribe = to_string(&unsubscribe).expect("Unable to serialize the subscribe message");
       client
           .send(Message::Text(unsubscribe))
           .await
           .expect("Unable to unsubscribe");
    */
    Ok(())
}
