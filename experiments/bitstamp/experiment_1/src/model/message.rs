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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub channel: Channel,
}

#[cfg(test)]
mod unit_test {
    use super::{Channel, ChannelType, CurrencyPair, Data, Event, Message};

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
    fn test_subscribe_unsubscribe() {
        let pair = CurrencyPair::Aavebtc;
        {
            let subscribe = Message::subscribe(ChannelType::DetailOrderBook, pair);
            assert_eq!(subscribe.event, Event::Subscribe);
            assert_eq!(
                subscribe.data.channel.channel_type,
                ChannelType::DetailOrderBook
            );
            assert_eq!(subscribe.data.channel.pair, pair);
        }

        let unsubscribe = Message::unsubscribe(ChannelType::LiveOrders, pair);
        assert_eq!(unsubscribe.event, Event::Unsubscribe);
        assert_eq!(
            unsubscribe.data.channel.channel_type,
            ChannelType::LiveOrders
        );
        assert_eq!(unsubscribe.data.channel.pair, pair);
    }
}
