use std::task::Poll;

use futures::StreamExt;
use serde_json::{from_str, Error};
// TODO: Make our own library error
use tokio_stream::Stream;
use tokio_tungstenite::tungstenite::Message;

use crate::model::Depth;

/// A tokio-stream that simply converts the incoming Json to usable structs
pub struct DepthStream<S: Stream<Item = Message> + Unpin> {
    input: S,
}

impl<S> Stream for DepthStream<S>
where
    S: Stream<Item = Message> + Unpin,
{
    type Item = Depth;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let msg = self.input.poll_next_unpin(cx);
        match msg {
            Poll::Ready(None) => Poll::Ready(None),
            // I was going to have this return a stream of Result's and leave logging up to the app, but because it's a small project I'll just log it in the library layer
            // I'm going to abandon this code now and rewrite it in a proper library and have it return a stream of results
            Poll::Ready(Some(Message::Text(msg))) => match from_str::<Depth>(&msg) {
                Ok(depth) => Poll::Ready(Some(depth)),
                Err(err) => {
                    log::error!("Unable to convert Market Depth struct from json: {msg}");
                    // We'll just skip this message in the interest of keeping the program running
                    // Hopefully someone will see it in the logs - We could send an SMS at this point
                    Poll::Pending
                }
            Poll::Ready(Some(weird_message)) => {
                log::warn!("Unexpected message received: {weird_message:?}");
                Poll::Pending
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
