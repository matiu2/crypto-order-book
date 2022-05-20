//! Model the json data we get as responses
//! Example input:
//!
//! {"data":
//!   {"timestamp":"1651388616",
//!    "microtimestamp":"1651388616274565",
//!    "bids":[["0.07315713","0.40000000","1485019713925121"], ...],
//!    "asks":[["0.07320505","0.40000000","1485019610763265"], ...]
//!   },
//!   "channel":"detail_order_book_ethbtc",
//!   "event":"data"}

use std::time::Duration;

use crate::Error;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

use serde::{Deserialize, Serialize};

/// order book data - this just models the 'data' part
/// {
///   "data": {
///     "timestamp": "1650247261",
///     "microtimestamp": "1650247261276311",
///     "bids": [
///       [
///         "0.07517475",
///         "10.00000000",
///         "1480344725942276"
///       ],
///       ...
///       [
///         "0.07457290",
///         "53.79000000",
///         "1480325387767809"
///       ]
///     ],
///     "asks": [
///       [
///         "0.07520827",
///         "0.05000000",
///         "1480344701804544"
///       ],
///       ...
///       [
///         "0.07637348",       # Price
///         "0.00268195",       # Quantity
///         "1477883311575044"  # Order ID
///       ]
///     ]
///   },
///   "channel": "detail_order_book_ethbtc",
///   "event": "data"
/// }
#[derive(Deserialize, Serialize)]
struct OrderBookDataRaw {
    microtimestamp: String,
    bids: Vec<(String, String, String)>,
    asks: Vec<(String, String, String)>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "OrderBookDataRaw", into = "OrderBookDataRaw")]
pub struct OrderBookData {
    pub timestamp: chrono::DateTime<Utc>,
    pub bids: Vec<Price>,
    pub asks: Vec<Price>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Price {
    pub price: f64,
    pub quantity: f64,
    pub order_id: u64,
}

impl From<OrderBookData> for OrderBookDataRaw {
    fn from(data: OrderBookData) -> Self {
        let microtimestamp = (data.timestamp.timestamp_nanos() as u64) / 1000;
        let price_to_str = |price: &Price| {
            (
                format!("{}", price.price),
                format!("{}", price.quantity),
                format!("{}", price.order_id),
            )
        };
        OrderBookDataRaw {
            microtimestamp: format!("{microtimestamp}"),
            bids: data.bids.iter().map(price_to_str).collect(),
            asks: data.asks.iter().map(price_to_str).collect(),
        }
    }
}

impl TryFrom<OrderBookDataRaw> for OrderBookData {
    type Error = Error;

    fn try_from(value: OrderBookDataRaw) -> Result<Self, Self::Error> {
        let to_price = |(price, quantity, order_id): (String, String, String)| {
            Ok(Price {
                price: price
                    .parse()
                    .map_err(|source| Error::decoding("Parse order book price", price, source))?,
                quantity: quantity.parse().map_err(|source| {
                    Error::decoding("Parse order book quantity", quantity, source)
                })?,
                order_id: order_id.parse().map_err(|source| {
                    Error::decoding("Parse order book order_id", order_id, source)
                })?,
            })
        };
        let bids = value
            .bids
            .into_iter()
            .map(to_price)
            .collect::<Result<Vec<Price>, Error>>()?;
        let asks = value
            .asks
            .into_iter()
            .map(to_price)
            .collect::<Result<Vec<Price>, Error>>()?;

        // Parse the timestamp
        let duration = value.microtimestamp.parse().map_err(|source| {
            Error::decoding(
                "Parse a micro second timestamp into u64 from order book data",
                value.microtimestamp,
                source,
            )
        })?;
        let micro_secs = chrono::Duration::from_std(Duration::from_micros(duration))
            .map_err(|source| Error::encoding("read micro seconds duration", duration, source))?;
        let epoch: NaiveDateTime = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
        let dt = epoch + micro_secs;

        Ok(OrderBookData {
            timestamp: DateTime::from_utc(dt, Utc),
            bids,
            asks,
        })
    }
}

#[cfg(test)]
mod unit_test {
    use chrono::{DateTime, NaiveDate, Utc};

    use super::OrderBookData;

    #[test]
    fn test_parse() {
        let data = r#"{
     "timestamp": "1650247261",
     "microtimestamp": "1650247261276311",
     "bids": [ [ "0.07517475", "10.00000000", "1480344725942276" ], [ "0.07457290", "53.79000000", "1480325387767809" ] ],
     "asks": [ [ "0.07520827", "0.05000000", "1480344701804544" ], [ "0.07637348", "0.00268195", "1477883311575044" ] ]
   }"#;
        let data: OrderBookData = serde_json::from_str(data).unwrap();
        dbg!(&data);
        // Should read: Monday, April 18, 2022 2:01:01.276311 AM UTC
        // Converted with https://www.epochconverter.com/
        let expected_time = NaiveDate::from_ymd(2022, 04, 18).and_hms_micro(2, 1, 1, 276311);
        let expected_time = DateTime::<Utc>::from_utc(expected_time, Utc);
        assert_eq!(&data.timestamp, &expected_time);

        // Make sure price and quantity are the right way around
        assert_eq!(data.bids[0].price, 0.07517475);
        assert_eq!(data.bids[0].quantity, 10.0);
    }
}
