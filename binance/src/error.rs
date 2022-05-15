use thiserror::Error;
use tungstenite::Error as WSError;

#[derive(Error, Debug)]
pub enum BinanceError {
    #[error("Unable to connect to websocket: url: \"{url}\" error: \"{error:?}\"")]
    Connect { url: String, error: WSError },
    #[error("We connected OK, but later got an error while trying to read a message: {0:?}")]
    MessageError(#[from] WSError),
    #[error("Unable to parse json. Error: \"{error:?}\" Original: \"{original}\"")]
    Json {
        error: serde_json::Error,
        original: String,
    },
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
