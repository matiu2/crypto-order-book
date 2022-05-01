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
    #[error("Decoding error: {reason}")]
    DecodingGeneral { reason: String },
    #[error("Encoding: \"{context}\" Input: \"{input:?}\", Source: \"{source:?}\"")]
    Encoding {
        context: &'static str,
        input: Box<dyn std::fmt::Debug>,
        source: Box<dyn std::error::Error>,
    },
    #[error("WebSocket: \"{context}\" Source: \"{source:?}\"")]
    WebSocket {
        context: &'static str,
        source: Box<dyn std::error::Error>,
    },
    #[error("WebSocket Send: \"{context}\" Message: \"{message:?}\" Source: \"{source:?}\"")]
    WebSocketSend {
        context: &'static str,
        message: TMessage,
        source: Box<dyn std::error::Error>,
    },
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
    pub fn decoding_general(reason: String) -> BitstampError {
        BitstampError::DecodingGeneral { reason }
    }
}

pub trait Context<T> {
    fn context(self, context: &'static str) -> Result<T, BitstampError>;
    fn message_context(self, message: TMessage, context: &'static str) -> Result<T, BitstampError>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: std::error::Error + 'static,
{
    fn context(self, context: &'static str) -> Result<T, BitstampError> {
        match self {
            Ok(result) => Ok(result),
            Err(source) => Err(BitstampError::WebSocket {
                context,
                source: Box::new(source) as Box<dyn std::error::Error>,
            }),
        }
    }

    fn message_context(self, message: TMessage, context: &'static str) -> Result<T, BitstampError> {
        match self {
            Ok(result) => Ok(result),
            Err(source) => Err(BitstampError::WebSocketSend {
                context,
                message,
                source: Box::new(source) as Box<dyn std::error::Error>,
            }),
        }
    }
}
