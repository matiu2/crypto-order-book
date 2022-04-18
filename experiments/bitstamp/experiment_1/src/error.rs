use std::num::{ParseFloatError, ParseIntError};

use parse_display::ParseError;
use thiserror::Error;
use time::OutOfRangeError;

#[derive(Error, Debug)]
pub enum BitstampError {
    #[error("Unspported channel type: value: \"{value}\" original error: \"{error:?}\"")]
    ChannelType { value: String, error: ParseError },
    #[error("Unspported event type: value: \"{value}\" original error: \"{error:?}\"")]
    EventType { value: String, error: ParseError },
    #[error("Unspported Channel name (due to channel type / prefix): value: \"{value}\" original error: \"{error:?}\"")]
    ChannelNameChannelType { value: String, error: ParseError },
    #[error("Unspported Channel name (due to channel currency pair / suffix): value: \"{value}\" original error: \"{error:?}\"")]
    ChannelNamePair { value: String, error: ParseError },
    #[error("Unspported channel name: value: \"{value}\"")]
    ChannelName { value: String },
    #[error("Unable to parse bid prices from order book data: {error}")]
    OrderBookBids { error: ParseFloatError },
    #[error("Unable to parse ask prices from order book data: {error}")]
    OrderBookAsks { error: ParseFloatError },
    #[error("Unable to parse a micro second timestamp into u64 from order book data: {duration} error: {error:?}")]
    OrderBookDurationString {
        duration: String,
        error: ParseIntError,
    },
    #[error("Unable to parse a micro second timestamp from order book data: {duration} error: {error:?}")]
    OrderBookDuration { duration: u64, error: ParseError },
    #[error("Unable to convert a duration: {duration} error: {error:?}")]
    DurationConvert {
        duration: u64,
        error: OutOfRangeError,
    },
}
