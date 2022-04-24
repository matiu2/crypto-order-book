use thiserror::Error;
use tokio_tungstenite::tungstenite::Message as TMessage;

#[derive(Error, Debug)]
pub enum BitstampError {
    #[error("Decoding: \"{context}\" Input: \"{input}\", Source: \"{source:?}\"")]
    Decoding {
        context: &'static str,
        input: String,
        source: Box<dyn std::error::Error>,
    },
    #[error("Expected input to contain at least 2 parts, separated by `_`: Input: \"{input}\"")]
    DecodingSplit { input: String },
    #[error("Encoding: \"{context}\" Input: \"{input:?}\", Source: \"{source:?}\"")]
    Encoding {
        context: &'static str,
        input: Box<dyn std::fmt::Debug>,
        source: Box<dyn std::error::Error>,
    },
    #[error("WebSocket: \"{context}\" Source: \"{source:?}\"")]
    WebSocket {
        context: &'static str,
        source: tokio_tungstenite::tungstenite::Error,
    },
    #[error("WebSocket Send: \"{context}\" Message: \"{message:?}\" Source: \"{source:?}\"")]
    WebSocketSend {
        context: &'static str,
        message: TMessage,
        source: tokio_tungstenite::tungstenite::Error,
    },
    /*
       #[error("Unspported channel type: value: \"{value}\" original error: \"{source:?}\"")]
       ChannelType { value: String, source: ParseError },
       #[error("Unspported event type: value: \"{value}\" original error: \"{source:?}\"")]
       EventType { value: String, source: ParseError },
       #[error("Unspported Channel name (due to channel type / prefix): value: \"{value}\" original error: \"{source:?}\"")]
       ChannelNameChannelType { value: String, source: ParseError },
       #[error("Unspported Channel name (due to channel currency pair / suffix): value: \"{value}\" original error: \"{source:?}\"")]
       ChannelNamePair { value: String, source: ParseError },
       #[error("Unspported channel name: value: \"{value}\"")]
       ChannelName { value: String },
       #[error("Unable to parse bid prices from order book data: {source}")]
       OrderBookBids { source: ParseFloatError },
       #[error("Unable to parse ask prices from order book data: {source}")]
       OrderBookAsks { source: ParseFloatError },
       #[error("Unable to parse a micro second timestamp into u64 from order book data: {duration} error: {source:?}")]
       OrderBookDurationString {
           duration: String,
           source: ParseIntError,
       },
       #[error("Unable to parse a micro second timestamp from order book data: {duration} error: {source:?}")]
       OrderBookDuration { duration: u64, source: ParseError },
       #[error("Unable to convert a duration: {duration} error: {source:?}")]
       DurationConvert {
           duration: u64,
           source: OutOfRangeError,
       },
       #[error("JSON Encode message: {message:?} - Error: {source:?}")]
       EncodeMessage {
           message: Message,
           source: serde_json::Error,
       },
    */
}

impl BitstampError {
    /// Create an error when encoding an outgoin websocket message
    pub fn encoding(
        context: &'static str,
        input: impl std::fmt::Debug + 'static,
        source: impl std::error::Error + 'static,
    ) -> BitstampError {
        BitstampError::Encoding {
            context,
            input: Box::new(input),
            source: Box::new(source),
        }
    }
    /// Create an error when decoding an incoming websocket message
    pub fn decoding<E>(context: &'static str, input: String, source: E) -> BitstampError
    where
        E: std::error::Error + 'static,
    {
        BitstampError::Decoding {
            context,
            input,
            source: Box::new(source),
        }
    }
    /// A special decoding error, with no source
    pub fn decoding_split(input: String) -> BitstampError {
        BitstampError::DecodingSplit { input }
    }
}

pub trait Context<T> {
    fn context(self, context: &'static str) -> Result<T, BitstampError>;
    fn message_context(self, message: TMessage, context: &'static str) -> Result<T, BitstampError>;
}

impl<T> Context<T> for std::result::Result<T, tokio_tungstenite::tungstenite::Error> {
    fn context(self, context: &'static str) -> Result<T, BitstampError> {
        match self {
            Ok(result) => Ok(result),
            Err(source) => Err(BitstampError::WebSocket { context, source }),
        }
    }

    fn message_context(self, message: TMessage, context: &'static str) -> Result<T, BitstampError> {
        match self {
            Ok(result) => Ok(result),
            Err(source) => Err(BitstampError::WebSocketSend {
                context,
                message,
                source,
            }),
        }
    }
}
