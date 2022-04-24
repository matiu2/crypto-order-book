//! Model the json data we get as responses
//! Example input:
//!
//! [src/main.rs:26] data = Text(
//!     "{\"event\":\"bts:subscription_succeeded\",\"channel\":\"live_orders_ethbtc\",\"data\":{}}",
//! )
//! [src/main.rs:26] data = Text(
//!     "{\"data\":{\"id\":1480343732187136,\"id_str\":\"1480343732187136\",\"order_type\":1,\"datetime\":\"1650247020\",\"microtimestamp\":\"1650247020470000\",\"amount\":5.84717518,\"amount_str\":\"5.84717518\",\"price\":0.07534527,\"price_str\":\"0.07534527\"},\"channel\":\"live_orders_ethbtc\",\"event\":\"order_deleted\"}",
//! )

use std::{num::ParseFloatError, time::Duration};

use crate::Error;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

use serde::Deserialize;

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
#[derive(Deserialize)]
struct OrderBookDataRaw {
    timestamp: String,
    microtimestamp: String,
    bids: Vec<(String, String, String)>,
    asks: Vec<(String, String, String)>,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "OrderBookDataRaw")]
pub struct OrderBookData {
    pub timestamp: chrono::DateTime<Utc>,
    pub bids: Vec<Price>,
    pub asks: Vec<Price>,
}

#[derive(Debug)]
pub struct Price {
    price: f64,
    quantity: f64,
}

impl TryFrom<OrderBookDataRaw> for OrderBookData {
    type Error = Error;

    fn try_from(value: OrderBookDataRaw) -> Result<Self, Self::Error> {
        let to_price = |(price, quantity, _order_id): (String, String, String)| {
            Ok(Price {
                price: price
                    .parse()
                    .map_err(|source| Error::decoding("Parse order book price", price, source))?,
                quantity: quantity.parse().map_err(|source| {
                    Error::decoding("Parse order book quantity", quantity, source)
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
