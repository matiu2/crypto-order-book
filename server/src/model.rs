use std::cmp::Ordering;

use crate::api::Level;

impl From<binance::model::Price> for Level {
    fn from(input: binance::model::Price) -> Self {
        Level {
            exchange: "binance".to_string(),
            // TODO: rename amount and quantitiy in binance to reduce confusion
            price: input.amount,
            amount: input.quantity,
        }
    }
}

impl From<bitstamp::model::Price> for Level {
    fn from(input: bitstamp::model::Price) -> Self {
        Level {
            exchange: "bitstamp".to_string(),
            price: input.price,
            amount: input.quantity,
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
    bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
    bids.truncate(10);
    // Get the 10 best (lowest) asks
    let mut asks: Vec<Level> = a
        .asks
        .into_iter()
        .map(|price| price.into())
        .chain(b.asks.into_iter().map(|price| price.into()))
        .collect();
    asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
    asks.truncate(10);

    // Get the spread
    let spread = bids
        .first()
        .zip(asks.first())
        .map(|(bid, ask)| bid.price - ask.price)
        .unwrap_or(0.0);

    crate::api::Summary { spread, bids, asks }
}

#[cfg(test)]
mod unit_test {
    use crate::api::{Level, Summary};
    use binance::model::{Depth, Price};
    use bitstamp::{model::Price as BitPrice, OrderBookData};
    use chrono::Utc;
    use ordered_float::OrderedFloat;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_make_merged_market_depth() {
        let binance = Depth {
            timestamp: Utc::now(),
            bids: vec![
                0.067016, 0.067017, 0.067028, 0.067029, 0.067035, 0.067039, 0.067049, 0.067054,
                0.067056, 0.067058, 0.067064, 0.067065, 0.067066, 0.067067, 0.067068, 0.067072,
                0.067074, 0.067075, 0.067076, 0.067077,
            ]
            .into_iter()
            .map(|price| Price {
                amount: price,
                quantity: 1.0,
            })
            .collect(),
            asks: vec![
                0.067078, 0.067088, 0.06709, 0.067091, 0.067094, 0.067097, 0.067098, 0.067099,
                0.0671, 0.067101, 0.067103, 0.067104, 0.067105, 0.067111, 0.067113, 0.067117,
                0.067118, 0.06712, 0.067121, 0.067122,
            ]
            .into_iter()
            .map(|price| Price {
                amount: price,
                quantity: 1.0,
            })
            .collect(),
        };
        let bitstamp = OrderBookData {
            timestamp: Utc::now(),
            bids: vec![
                0.0665682, 0.06657, 0.06657, 0.06657408, 0.06657996, 0.06658, 0.06658, 0.06658317,
                0.06658584, 0.06659172, 0.0665976, 0.06660348, 0.0666044, 0.06660936, 0.06661524,
                0.06662112, 0.066627, 0.06663288, 0.06663659, 0.06663876, 0.06664464, 0.06665052,
                0.0666564, 0.06665725, 0.06666228, 0.06666301, 0.06666816, 0.06667404, 0.06667992,
                0.0666858, 0.06669168, 0.06669756, 0.06669919, 0.06670344, 0.06670932, 0.0667115,
                0.0667152, 0.06672108, 0.06672696, 0.06673284, 0.06673872, 0.0667446, 0.06675048,
                0.06675636, 0.06676224, 0.06676812, 0.066774, 0.06677487, 0.06677988, 0.06678576,
                0.06679164, 0.06679752, 0.0668034, 0.06680928, 0.06681516, 0.06682104, 0.06682692,
                0.06683, 0.0668308, 0.0668328, 0.06683868, 0.06684456, 0.06685044, 0.06685447,
                0.0668622, 0.06686808, 0.06686809, 0.06686912, 0.06687984, 0.06688572, 0.06688573,
                0.06688963, 0.0668952, 0.06689748, 0.06690336, 0.06690338, 0.06691512, 0.066921,
                0.06692101, 0.0669425, 0.06694339, 0.06694791, 0.06697938, 0.06700245, 0.06700851,
                0.06701951, 0.06702107, 0.06703619, 0.06703735, 0.06705188, 0.06705342, 0.06706638,
                0.06706819, 0.06707133, 0.06707185, 0.06708276, 0.06709148, 0.06709423, 0.06709444,
                0.06712,
            ]
            .into_iter()
            .map(|price| BitPrice {
                price,
                quantity: 1.0,
                order_id: 1,
            })
            .collect(),
            asks: vec![
                0.06713393, 0.06714474, 0.06714588, 0.06715689, 0.06715965, 0.06716962, 0.06717535,
                0.0671784, 0.06718384, 0.06718693, 0.06719566, 0.06719891, 0.06721067, 0.067212,
                0.0672157, 0.06721913, 0.06722843, 0.0672354, 0.06726183, 0.06727176, 0.0672894,
                0.06729431, 0.0672945, 0.06730704, 0.06732467, 0.06732468, 0.06733702, 0.06733703,
                0.06734232, 0.06735995, 0.06735996, 0.0673776, 0.06737764, 0.06739524, 0.06739528,
                0.06741288, 0.06741292, 0.06741868, 0.06743052, 0.06743056, 0.06744626, 0.0674465,
                0.06744816, 0.0674482, 0.0674482, 0.0674658, 0.0674658, 0.06746584, 0.06748344,
                0.06748348, 0.06749532, 0.06750108, 0.06750112, 0.06751296, 0.06751872, 0.06751876,
                0.06752311, 0.0675306, 0.06753636, 0.0675364, 0.06754824, 0.0675536, 0.067554,
                0.06755404, 0.06756588, 0.06757164, 0.06757168, 0.06758352, 0.06758636, 0.06758928,
                0.06758932, 0.06760116, 0.06760116, 0.06760692, 0.06760696, 0.0676188, 0.06762456,
                0.0676246, 0.06763644, 0.0676422, 0.06764224, 0.06765408, 0.06765984, 0.06765988,
                0.06766437, 0.06767172, 0.06767748, 0.06767752, 0.06768936, 0.06769512, 0.06769516,
                0.0676992, 0.067707, 0.06771276, 0.0677128, 0.06772464, 0.0677304, 0.06773044,
                0.06774228, 0.06774804,
            ]
            .into_iter()
            .map(|price| BitPrice {
                price,
                quantity: 1.0,
                order_id: 1,
            })
            .collect(),
        };

        let got = super::make_merged_market_depth(binance, bitstamp);

        let expected = Summary {
            spread: 4.200000000000037e-5,
            bids: vec![
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06712,
                    amount: 1.0,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06709444,
                    amount: 1.0,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06709423,
                    amount: 1.0,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06709148,
                    amount: 1.0,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06708276,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067077,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067076,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067075,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067074,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067072,
                    amount: 1.0,
                },
            ],
            asks: vec![
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067078,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067088,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.06709,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067091,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067094,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067097,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067098,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067099,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.0671,
                    amount: 1.0,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067101,
                    amount: 1.0,
                },
            ],
        };
        assert_eq!(got, expected);

        // Also make sure expected is sorted
        // Asks should have the smallest value first
        let mut sorted = expected.asks.clone();
        sorted.sort_by(|a, b| OrderedFloat(a.price).cmp(&OrderedFloat(b.price)));
        assert_eq!(sorted, expected.asks);

        // Bids should be sorted with the largest value first
        let mut sorted = expected.bids.clone();
        sorted.sort_by(|a, b| OrderedFloat(a.price).cmp(&OrderedFloat(b.price)));
        sorted.reverse();
        assert_eq!(sorted, expected.bids);

        // The Spread should be the highest (first) bid - the lowest (first) ask
        let lowest_ask = expected
            .asks
            .iter()
            .map(|level| OrderedFloat(level.price))
            .min()
            .unwrap();
        let highest_bid = expected
            .bids
            .iter()
            .map(|level| OrderedFloat(level.price))
            .max()
            .unwrap();
        let expected_spread = highest_bid - lowest_ask;
        assert_eq!(got.spread, expected_spread.into_inner());
    }
}
