mod channel;
mod currency_pair;
mod event;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub use self::currency_pair::CurrencyPair;
pub use self::{
    channel::{Channel, ChannelType},
    event::Event,
};
use tokio_tungstenite::tungstenite::protocol::Message as TMessage;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Message {
    event: Event,
    data: Data,
}

impl Message {
    fn new(event: Event, channel_type: ChannelType, currency_pair: CurrencyPair) -> Message {
        Message {
            event,
            data: Data {
                channel: Channel {
                    channel_type,
                    pair: currency_pair,
                },
            },
        }
    }
    /// Generate the request message to subscribe to a channel
    pub fn subscribe(channel_type: ChannelType, currency_pair: CurrencyPair) -> Result<TMessage> {
        let message = Message::new(Event::Subscribe, channel_type, currency_pair);
        let as_str = to_string(&message).map_err(|source| {
            Error::encoding("web socket -> creating subscribe message", message, source)
        })?;
        Ok(TMessage::Text(as_str))
    }
    /// Generate the request message to unsubscribe from a channel
    pub fn unsubscribe(channel_type: ChannelType, currency_pair: CurrencyPair) -> Result<TMessage> {
        let message = Message::new(Event::Unsubscribe, channel_type, currency_pair);
        let as_str = to_string(&message).map_err(|source| {
            Error::encoding("web socket -> creating subscribe message", message, source)
        })?;
        Ok(TMessage::Text(as_str))
    }
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct Data {
    pub channel: Channel,
}

#[cfg(test)]
mod unit_test {
    use super::{Channel, ChannelType, CurrencyPair, Data, Event, Message};
    use tokio_tungstenite::tungstenite::Message as TMessage;

    #[test]
    fn test_serialize() {
        let request = Message {
            event: Event::Subscribe,
            data: Data {
                channel: Channel {
                    channel_type: ChannelType::LiveOrders,
                    pair: CurrencyPair::Ethbtc,
                },
            },
        };
        let out = serde_json::ser::to_string(&request).expect("Unable to Serialize");
        assert_eq!(
            &out,
            r#"{"event":"bts:subscribe","data":{"channel":"live_orders_ethbtc"}}"#
        )
    }

    #[test]
    fn test_subscribe() {
        let pair = CurrencyPair::Aavebtc;
        let raw = Message::subscribe(ChannelType::DetailOrderBook, pair).unwrap();
        let decoded = if let TMessage::Text(data) = raw {
            let decoded: Message = serde_json::from_str(&data).unwrap();
            decoded
        } else {
            panic!("It should be a text websocket message");
        };
        let expected = Message {
            event: Event::Subscribe,
            data: Data {
                channel: Channel {
                    channel_type: ChannelType::DetailOrderBook,
                    pair,
                },
            },
        };
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_unsubscribe() {
        let pair = CurrencyPair::Aavebtc;
        let raw = Message::unsubscribe(ChannelType::LiveOrders, pair).unwrap();
        let decoded = if let TMessage::Text(data) = raw {
            let decoded: Message = serde_json::from_str(&data).unwrap();
            decoded
        } else {
            panic!("It should be a text websocket message");
        };
        let expected = Message {
            event: Event::Unsubscribe,
            data: Data {
                channel: Channel {
                    channel_type: ChannelType::LiveOrders,
                    pair,
                },
            },
        };
        assert_eq!(decoded, expected);
    }
}
