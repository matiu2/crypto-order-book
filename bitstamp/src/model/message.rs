mod channel;
mod currency_pair;
use crate::{Error, OrderBookData, Result};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub use self::channel::{Channel, ChannelType};
pub use self::currency_pair::CurrencyPair;
use tokio_tungstenite::tungstenite::protocol::Message as TMessage;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "event")]
pub enum Message {
    #[serde(rename = "bts:subscribe")]
    Subscribe {
        data: ChannelData,
    },
    #[serde(rename = "bts:subscription_succeeded")]
    SubscriptionSucceeded {
        channel: Channel,
    },
    Data {
        data: OrderBookData,
    },
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    #[serde(rename = "bts:error")]
    Error {
        data: ErrorData,
    },
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct ErrorData {
    code: Option<u32>,
    message: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct ChannelData {
    channel: Channel,
}

impl Message {
    /// Generate the request message to subscribe to a channel
    pub fn subscribe(channel_type: ChannelType, currency_pair: CurrencyPair) -> Result<TMessage> {
        let message = Message::Subscribe {
            data: ChannelData {
                channel: Channel {
                    channel_type,
                    pair: currency_pair,
                },
            },
        };
        let as_str = to_string(&message).map_err(|source| {
            Error::encoding("web socket -> creating subscribe message", message, source)
        })?;
        Ok(TMessage::Text(as_str))
    }
}

impl TryFrom<TMessage> for Message {
    type Error = Error;

    fn try_from(value: TMessage) -> Result<Self> {
        match value {
            TMessage::Text(data) => Ok(serde_json::from_str(&data)
                .map_err(|source| Error::decoding("Incoming Message", data, source))?),
            TMessage::Ping(data) => Ok(Message::Ping(data)),
            other => Err(Error::decoding_general(format!(
                "Expected tungstenite::Message::Text, but got {other:?}"
            ))),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use crate::{
        model::{message::ErrorData, Price},
        OrderBookData,
    };

    use super::{Channel, ChannelType, CurrencyPair, Message};
    use chrono::{DateTime, NaiveDate, Utc};
    use tokio_tungstenite::tungstenite::Message as TMessage;

    #[test]
    fn test_serialize() {
        let request = Message::Subscribe {
            data: crate::model::message::ChannelData {
                channel: Channel {
                    channel_type: ChannelType::DetailOrderBook,
                    pair: CurrencyPair::Ethbtc,
                },
            },
        };
        let out = serde_json::ser::to_string(&request).expect("Unable to Serialize");
        assert_eq!(
            &out,
            r#"{"event":"bts:subscribe","data":{"channel":"detail_order_book_ethbtc"}}"#
        )
    }

    #[test]
    fn test_subscribe_render() {
        let pair = CurrencyPair::Aavebtc;
        let raw = Message::subscribe(ChannelType::DetailOrderBook, pair).unwrap();
        let decoded = if let TMessage::Text(data) = raw {
            let decoded: Message = serde_json::from_str(&data).unwrap();
            decoded
        } else {
            panic!("It should be a text websocket message");
        };
        let expected = r#"{
            "event": "bts:subscribe",
            "data": {
                "channel": "detail_order_book_aavebtc"
            }
        }"#;
        let expected: Message = serde_json::from_str(expected).unwrap();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_parse() {
        let input = r#"
            {"data":
            {"timestamp":"1651388616",
            "microtimestamp":"1651388616274565",
                "bids":[["0.07315713","0.40000000","1485019713925121"]],
                "asks":[["0.07320505","0.40000000","1485019610763265"]]
            },
            "channel":"detail_order_book_ethbtc",
            "event":"data"}"#;
        let expected_time = NaiveDate::from_ymd(2022, 05, 1).and_hms_micro(7, 3, 36, 274565);
        let message: Message = serde_json::from_str(input).unwrap();
        assert_eq!(
            message,
            Message::Data {
                data: OrderBookData {
                    timestamp: DateTime::from_utc(expected_time, Utc),
                    bids: vec![Price {
                        price: 0.07315713,
                        quantity: 0.40000000,
                        order_id: 1485019713925121,
                    },],
                    asks: vec![Price {
                        price: 0.07320505,
                        quantity: 0.4,
                        order_id: 1485019610763265,
                    },]
                }
            }
        )
    }

    #[test]
    fn test_parse_error() {
        let input = "{\"event\":\"bts:error\",\"channel\":\"\",\"data\":{\"code\":null,\"message\":\"Bad subscription string.\"}}";
        let message: Message = serde_json::from_str(input).unwrap();
        assert_eq!(
            message,
            Message::Error {
                data: ErrorData {
                    code: None,
                    message: "Bad subscription string.".to_string(),
                }
            }
        )
    }
}
