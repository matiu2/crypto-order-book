mod channel;
mod currency_pair;
mod event;
use serde::{Deserialize, Serialize};

pub use self::currency_pair::CurrencyPair;
use self::{
    channel::{Channel, ChannelType},
    event::Event,
};

#[derive(Deserialize, Serialize)]
pub struct Message {
    event: Event,
    data: Data,
}

impl Message {
    /// Generate the request message to subscribe to a channel
    pub fn subscribe(channel_type: ChannelType, currency_pair: CurrencyPair) -> Message {
        Message {
            event: Event::Subscribe,
            data: Data {
                channel: Channel {
                    channel_type,
                    pair: currency_pair,
                },
            },
        }
    }
    /// Generate the request message to unsubscribe from a channel
    pub fn unsubscribe(channel_type: ChannelType, currency_pair: CurrencyPair) -> Message {
        let mut out = Message::subscribe(channel_type, currency_pair);
        out.event = Event::Unsubscribe;
        out
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
