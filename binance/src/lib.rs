use std::task::Poll;

use futures::StreamExt;
pub mod model;
use model::Depth;
use serde_json::de::from_str;
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

mod error;
pub use error::BinanceError as Error;
pub type Result<T> = std::result::Result<T, Error>;

/// A tokio-stream that simply converts the incoming Json to usable structs
pub struct DepthStream {
    input: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Stream for DepthStream {
    type Item = Result<Depth>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let msg = self.input.poll_next_unpin(cx);
        match msg {
            Poll::Ready(None) => Poll::Ready(None),
            // I was going to have this return a stream of Result's and leave logging up to the app, but because it's a small project I'll just log it in the library layer
            // I'm going to abandon this code now and rewrite it in a proper library and have it return a stream of results
            Poll::Ready(Some(Ok(Message::Text(msg)))) => {
                let depth = from_str::<Depth>(&msg).map_err(|error| Error::Json {
                    error,
                    original: msg,
                });
                Poll::Ready(Some(depth))
            }
            Poll::Ready(Some(Ok(weird_message))) => {
                Poll::Ready(Some(Err(Error::MessageType(weird_message))))
            }
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(Error::MessageError(err)))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Connect to binance and return a new stream
/// `instrument` should come from binance's instrument list, eg. "ethbtc"
pub async fn binance_stream(instrument: &str) -> Result<DepthStream> {
    let url = format!("wss://stream.binance.com:9443/ws/{instrument}@depth20@100ms");
    let (client, _response) = connect_async(&url)
        .await
        .map_err(|error| Error::Connect { url, error })?;
    Ok(DepthStream { input: client })
}

#[cfg(test)]
mod test {
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
