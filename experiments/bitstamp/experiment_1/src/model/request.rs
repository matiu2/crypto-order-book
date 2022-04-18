use parse_display::Display;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Request {
    event: Event,
    data: Data,
}

impl Request {
    /// Generate the data for a request to subscribe to orders
    pub fn subscribe_to_orders(currency_pair: CurrencyPair) -> Request {
        Request {
            event: Event::Subscribe,
            data: Data {
                channel: Channel {
                    channel_type: ChannelType::LiveOrders,
                    pair: currency_pair,
                },
            },
        }
    }
    /// Generate the data for a request to unsubscribe to orders
    pub fn unsubscribe_from_orders(currency_pair: CurrencyPair) -> Request {
        let mut out = Request::subscribe_to_orders(currency_pair);
        out.event = Event::Unsubscribe;
        out
    }
}

#[derive(Serialize, Debug)]
pub struct Data {
    channel: Channel,
}

#[derive(Display, PartialEq, Debug)]
#[display(style = "snake_case")]
pub enum Event {
    #[display("bts:{}")]
    Subscribe,
    #[display("bts:{}")]
    Unsubscribe,
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

#[derive(Display, PartialEq, Debug)]
#[display(style = "snake_case")]
pub enum ChannelType {
    LiveOrders,
}

impl Default for ChannelType {
    fn default() -> Self {
        ChannelType::LiveOrders
    }
}

#[derive(Debug)]
pub struct Channel {
    channel_type: ChannelType,
    pair: CurrencyPair,
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}_{}", &self.channel_type, &self.pair))
    }
}

#[derive(Display, PartialEq, Debug, Clone, Copy)]
#[display(style = "snake_case")]
pub enum CurrencyPair {
    Aavebtc,
    Aaveeur,
    Aaveusd,
    Adabtc,
    Adaeur,
    Adausd,
    Algobtc,
    Algoeur,
    Algousd,
    Alphaeur,
    Alphausd,
    Ampeur,
    Ampusd,
    Audiobtc,
    Audioeur,
    Audiousd,
    Avaxeur,
    Avaxusd,
    Axseur,
    Axsusd,
    Batbtc,
    Bateur,
    Batusd,
    Bchbtc,
    Bcheur,
    Bchgbp,
    Bchusd,
    Btceur,
    Btcgbp,
    Btcpax,
    Btcusd,
    Btcusdc,
    Btcusdt,
    Celeur,
    Celusd,
    Chzeur,
    Chzusd,
    Compbtc,
    Compeur,
    Compusd,
    Crvbtc,
    Crveur,
    Crvusd,
    Ctsieur,
    Ctsiusd,
    Cvxeur,
    Cvxusd,
    Daiusd,
    Dydxeur,
    Dydxusd,
    Enjeur,
    Enjusd,
    Eth2eth,
    Ethbtc,
    Etheur,
    Ethgbp,
    Ethpax,
    Ethusd,
    Ethusdc,
    Ethusdt,
    Eurteur,
    Eurtusd,
    Eurusd,
    Feteur,
    Fetusd,
    Ftmeur,
    Ftmusd,
    Ftteur,
    Fttusd,
    Galaeur,
    Galausd,
    Gbpeur,
    Gbpusd,
    Grteur,
    Grtusd,
    Gusdusd,
    Hbareur,
    Hbarusd,
    Imxeur,
    Imxusd,
    Kncbtc,
    Knceur,
    Kncusd,
    Linkbtc,
    Linketh,
    Linkeur,
    Linkgbp,
    Linkusd,
    Ltcbtc,
    Ltceur,
    Ltcgbp,
    Ltcusd,
    Maticeur,
    Maticusd,
    Mkrbtc,
    Mkreur,
    Mkrusd,
    Nexoeur,
    Nexousd,
    Omgbtc,
    Omgeur,
    Omggbp,
    Omgusd,
    Paxeur,
    Paxgbp,
    Paxusd,
    Perpeur,
    Perpusd,
    Rgteur,
    Rgtusd,
    Sandeur,
    Sandusd,
    Sgbeur,
    Sgbusd,
    Skleur,
    Sklusd,
    Slpeur,
    Slpusd,
    Snxbtc,
    Snxeur,
    Snxusd,
    Storjeur,
    Storjusd,
    Sushieur,
    Sushiusd,
    Sxpeur,
    Sxpusd,
    Umabtc,
    Umaeur,
    Umausd,
    Unibtc,
    Unieur,
    Uniusd,
    Usdceur,
    Usdcusd,
    Usdcusdt,
    Usdteur,
    Usdtusd,
    Usteur,
    Ustusd,
    Wbtcbtc,
    Xlmbtc,
    Xlmeur,
    Xlmgbp,
    Xlmusd,
    Xrpbtc,
    Xrpeur,
    Xrpgbp,
    Xrppax,
    Xrpusd,
    Xrpusdt,
    Yfibtc,
    Yfieur,
    Yfiusd,
    Zrxbtc,
    Zrxeur,
    Zrxusd,
}

#[cfg(test)]
mod unit_test {
    use super::{Channel, ChannelType, CurrencyPair, Data, Event, Request};

    #[test]
    fn test_serialize() {
        let request = Request {
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
            let subscribe = Request::subscribe_to_orders(pair);
            assert_eq!(subscribe.event, Event::Subscribe);
            assert_eq!(subscribe.data.channel.channel_type, ChannelType::LiveOrders);
            assert_eq!(subscribe.data.channel.pair, pair);
        }

        let unsubscribe = Request::unsubscribe_from_orders(pair);
        assert_eq!(unsubscribe.event, Event::Unsubscribe);
        assert_eq!(
            unsubscribe.data.channel.channel_type,
            ChannelType::LiveOrders
        );
        assert_eq!(unsubscribe.data.channel.pair, pair);
    }
}
