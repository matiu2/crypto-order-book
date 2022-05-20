use chrono::{DateTime, Utc};
use std::num::ParseFloatError;

use serde::Deserialize;

/// Partial market depth stream
/// This is what binance gives us - we later convert it into `Depth`
/// See: https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#general-wss-information
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawDepth {
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

/// A message from binance, showing the most recent market depth for a
/// particular symbol
#[derive(Deserialize, Debug)]
#[serde(try_from = "RawDepth")]
pub struct Depth {
    pub timestamp: DateTime<Utc>,
    pub bids: Vec<Price>,
    pub asks: Vec<Price>,
}

/// The amount and quantity of a particular bid or a ask
#[derive(Debug)]
pub struct Price {
    pub amount: f64,
    pub quantity: f64,
}

impl TryFrom<RawDepth> for Depth {
    type Error = ParseFloatError;

    fn try_from(value: RawDepth) -> Result<Self, Self::Error> {
        let bids: Result<Vec<Price>, Self::Error> = value
            .bids
            .into_iter()
            .map(|(amount, quantity)| {
                Ok(Price {
                    amount: amount.parse()?,
                    quantity: quantity.parse()?,
                })
            })
            .collect();
        let bids = bids?;
        let asks: Result<Vec<Price>, Self::Error> = value
            .asks
            .into_iter()
            .map(|(amount, quantity)| {
                Ok(Price {
                    amount: amount.parse()?,
                    quantity: quantity.parse()?,
                })
            })
            .collect();
        let asks = asks?;
        Ok(Depth {
            bids,
            asks,
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod unit_test {
    use super::Depth;
    use serde_json::from_str;

    #[test]
    fn test_parse() {
        let input = r#"{"lastUpdateId":5144117438,"bids":[["0.07530500","38.24170000"],["0.07530400","0.12670000"],["0.07530100","8.14710000"],["0.07529600","0.22860000"],["0.07529500","2.70550000"],["0.07528900","0.60620000"],["0.07528700","0.00420000"],["0.07528500","2.81950000"],["0.07528400","3.83370000"],["0.07527300","1.25770000"],["0.07527200","3.93890000"],["0.07527000","11.91820000"],["0.07526900","11.66480000"],["0.07526600","0.17940000"],["0.07526500","10.03450000"],["0.07526400","11.91650000"],["0.07526300","14.16510000"],["0.07526200","60.80000000"],["0.07526100","1.18930000"],["0.07525800","1.12460000"]],"asks":[["0.07530600","3.81910000"],["0.07530700","8.25850000"],["0.07530800","0.10000000"],["0.07531200","2.74780000"],["0.07531300","0.09120000"],["0.07531800","2.36240000"],["0.07531900","0.16480000"],["0.07532100","10.25780000"],["0.07532200","15.12800000"],["0.07532300","16.20000000"],["0.07532400","0.04760000"],["0.07532500","1.00630000"],["0.07532800","3.64420000"],["0.07532900","3.77510000"],["0.07533000","0.53120000"],["0.07533100","2.15300000"],["0.07533200","10.30650000"],["0.07533300","1.32790000"],["0.07533400","23.50000000"],["0.07533900","5.30560000"]]}"#;
        let depth: Depth = from_str(&input).unwrap();
        // Make sure it got the amount and quantity the right way around
        dbg!(&depth);
        let super::Price { amount, quantity } = &depth.bids[0];
        assert_eq!(*amount, 0.07530500);
        assert_eq!(*quantity, 38.24170000);
    }
}
