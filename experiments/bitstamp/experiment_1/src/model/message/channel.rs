use serde::{Deserialize, Serialize};

use crate::Error;

use super::CurrencyPair;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display(style = "snake_case")]
pub enum ChannelType {
    LiveOrders,
    DetailOrderBook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        match parts.as_slice() {
            [channel_type, pair] => Ok(Channel {
                channel_type: channel_type.parse().map_err(|error| {
                    Error::ChannelNameChannelType {
                        value: value.to_string(),
                        error,
                    }
                })?,
                pair: pair.parse().map_err(|error| Error::ChannelNamePair {
                    value: value.to_string(),
                    error,
                })?,
            }),
            _ => Err(Error::ChannelName {
                value: value.to_string(),
            }),
        }
    }
}
