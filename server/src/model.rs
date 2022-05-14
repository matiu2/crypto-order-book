use std::cmp::Ordering;

use crate::api::Level;

impl Into<Level> for binance::model::Price {
    fn into(self) -> Level {
        Level {
            exchange: "binance".to_string(),
            // TODO: rename amount and quantitiy in binance to reduce confusion
            price: self.amount,
            amount: self.quantity,
        }
    }
}

impl Into<Level> for bitstamp::model::Price {
    fn into(self) -> Level {
        Level {
            exchange: "bitstamp".to_string(),
            price: self.price,
            amount: self.quantity,
        }
    }
}

/// Takes the two order_books from our two client libraries and make a new order_book, ready to serve
pub fn make_merged_market_depth(
    a: binance::model::Depth,
    b: bitstamp::model::OrderBookData,
) -> crate::api::Summary {
    // Get the top 10 (highest) bids
    let mut bids: Vec<Level> = a
        .bids
        .into_iter()
        .map(|price| price.into())
        .chain(b.bids.into_iter().map(|price| price.into()))
        .collect();
    bids.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
    bids.truncate(10);
    // Get the 10 best (lowest) asks
    let mut asks: Vec<Level> = a
        .asks
        .into_iter()
        .map(|price| price.into())
        .chain(b.asks.into_iter().map(|price| price.into()))
        .collect();
    asks.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
    asks.truncate(10);

    // Get the spread
    let spread = bids
        .first()
        .zip(asks.first())
        .map(|(bid, ask)| bid.price - ask.price)
        .unwrap_or(0.0);

    crate::api::Summary { spread, bids, asks }
}
