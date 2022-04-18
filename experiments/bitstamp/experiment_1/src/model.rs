//! Models for the json interface as described by https://www.bitstamp.net/websocket/v2/
// Messages we send out
pub mod request;
pub use request::{CurrencyPair, Request};

// Messages we receive
pub mod response;
