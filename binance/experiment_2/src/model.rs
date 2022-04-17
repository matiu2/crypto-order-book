use serde::{Deserialize, Serialize};

/// See https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#subscribe-to-a-stream
#[derive(Serialize)]
pub struct Message {
    pub method: Method,
    pub params: Vec<&'static str>,
    pub id: usize,
}
#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Method {
    Subscribe,
    Unsubscribe,
}

impl Message {
    /// Generate a subscribe message
    /// [See docs](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#subscribe-to-a-stream)
    /// Example stream name: ETHBTC@bookTicker
    pub fn subscribe(stream_name: &'static str, id: usize) -> Message {
        Message {
            method: Method::Subscribe,
            params: vec![stream_name],
            id,
        }
    }
    /// Generate an un-subscribe message
    pub fn unsubscribe(stream_name: &'static str, id: usize) -> Message {
        Message {
            method: Method::Unsubscribe,
            params: vec![stream_name],
            id,
        }
    }
}

/// Book ticker incoming message
/// See: https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#individual-symbol-book-ticker-streams
#[derive(Deserialize)]
pub struct BookTicker {
    /// Order book updateId
    #[serde(rename = "u")]
    pub update_id: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid_price: String,
    #[serde(rename = "B")]
    pub best_bid_qty: String,
    #[serde(rename = "a")]
    pub best_ask_price: String,
    #[serde(rename = "A")]
    pub best_ask_qty: String,
}
