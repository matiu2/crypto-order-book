mod error;
use std::task::Poll;

pub use error::BinanceError as Error;
use futures::StreamExt;
use model::Depth;
use serde_json::de::from_str;
use tokio_stream::Stream;
use tokio_tungstenite::tungstenite::Message;
/// A tokio-stream that simply converts the incoming Json to usable structs
pub struct DepthStream<S> {
    input: S,
}

impl<S> Stream for DepthStream<S>
where
    S: Stream<Item = Message> + Unpin,
{
    type Item = Result<Depth, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let msg = self.input.poll_next_unpin(cx);
        match msg {
            Poll::Ready(None) => Poll::Ready(None),
            // I was going to have this return a stream of Result's and leave logging up to the app, but because it's a small project I'll just log it in the library layer
            // I'm going to abandon this code now and rewrite it in a proper library and have it return a stream of results
            Poll::Ready(Some(Message::Text(msg))) => {
                let depth = from_str::<Depth>(&msg).map_err(|error| Error::Json {
                    error,
                    original: msg,
                });
                Poll::Ready(Some(depth))
            }
            Poll::Ready(Some(weird_message)) => {
                Poll::Ready(Some(Err(Error::MessageType(weird_message))))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
