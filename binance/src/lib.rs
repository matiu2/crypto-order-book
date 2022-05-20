use futures::StreamExt;
pub mod model;
use model::Depth;
use serde_json::de::from_str;
use tokio_stream::Stream;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod error;
pub use error::BinanceError as Error;
pub type Result<T> = std::result::Result<T, Error>;

/// A tokio-stream that simply converts the incoming Json to usable structs
/// Connect to binance and return a new stream
/// `instrument` should come from binance's instrument list, eg. "ethbtc"
pub async fn binance_stream(
    instrument: &str,
) -> Result<impl Stream<Item = Result<Depth>> + Send + 'static> {
    let url = format!("wss://stream.binance.com:9443/ws/{instrument}@depth20@100ms");
    let (client, _response) = connect_async(&url)
        .await
        .map_err(|error| Error::Connect { url, error })?;
    Ok(Box::pin(client.filter_map(|result| async move {
        match result {
            // Incoming message is text; parse it
            Ok(Message::Text(msg)) => Some(from_str::<Depth>(&msg).map_err(|error| Error::Json {
                error,
                original: msg,
            })),
            // Filter out and log warnings for non-text messages
            Ok(unexpected_message) => {
                log::warn!("Unexpceted message type (not text): {unexpected_message:?}");
                None
            }
            // Convert all errors
            Err(err) => Some(Err(err.into())),
        }
    })))
}

#[cfg(test)]
mod web_test {
    use futures::StreamExt;

    /// Test if we can connect to binance and start downloading ethbtc
    #[tokio::test]
    async fn test_ethbtc() {
        let mut stream = super::binance_stream("ethbtc")
            .await
            .expect("Unable to connect to binance");
        match stream.next().await {
            // Got a depth book
            Some(Ok(first)) => dbg!(first),
            Some(Err(err)) => panic!("First message was an error: {err:?}"),
            None => panic!("No first message!"),
        };
    }
}
