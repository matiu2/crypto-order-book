use serde::{Deserialize, Serialize};

use crate::Error;

use super::CurrencyPair;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display(style = "snake_case")]
pub enum ChannelType {
    DetailOrderBook,
    // LiveOrders, // TODO: Implement other channel types
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub struct Channel {
    pub channel_type: ChannelType,
    pub pair: CurrencyPair,
}

impl Into<String> for Channel {
    fn into(self) -> String {
        format!("{}_{}", &self.channel_type, &self.pair)
    }
}

impl TryFrom<&str> for Channel {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.rsplitn(2, '_').collect();
        // Because we're using rsplitn, the order is reversed; ie. ct_pair => pair, ct
        match parts.as_slice() {
            [pair, channel_type] => Ok(Channel {
                channel_type: channel_type.parse().map_err(|source| {
                    Error::decoding(
                        "Invalid channel-type-name",
                        channel_type.to_string(),
                        source,
                    )
                })?,
                pair: pair.parse().map_err(|source| {
                    Error::decoding(
                        "Unspported Channel name (due to channel currency pair / suffix)",
                        pair.to_string(),
                        source,
                    )
                })?,
            }),
            _ => Err(Error::decoding_general(format!(
                "Expected input to contain at least 2 parts, separated by `_`: Input: \"{value}\""
            ))),
        }
    }
}
