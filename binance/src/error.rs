use thiserror::Error;
use tokio_tungstenite::tungstenite::Message;
use tungstenite::Error as WSError;

#[derive(Error, Debug)]
pub enum BinanceError {
    #[error("Unable to connect to websocket: url: \"{url}\" error: \"{error:?}\"")]
    Connect { url: String, error: WSError },
    #[error(
        "We connected OK, but later got an error while trying to read the next message: {0:?}"
    )]
    MessageError(WSError),
    #[error("Unable to parse json. Error: \"{error:?}\" Original: \"{original}\"")]
    Json {
        error: serde_json::Error,
        original: String,
    },
    #[error("Expected a Text Message, but got a different type {0}")]
    MessageType(Message),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
