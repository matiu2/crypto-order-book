//! Models for the json interface as described by https://www.bitstamp.net/websocket/v2/
// Messages we send out
pub mod message;
pub use message::{Channel, ChannelType, CurrencyPair, Message};

// Messages we receive
pub mod order_book;
pub use order_book::{OrderBookData, Price};
