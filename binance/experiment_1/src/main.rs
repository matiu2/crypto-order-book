use binance_api_async::api::Binance;
use binance_api_async::error::BinanceErr;
use binance_api_async::futures::TryStreamExt;
use binance_api_async::model::BookTickerEvent;
use binance_api_async::websocket::{
    Websocket, WebsocketAsync, WebsocketEvent, WebsocketStreamType,
};

#[tokio::main]
async fn main() -> Result<(), BinanceErr> {
    let symbols = vec!["ETHBTC".into(), "ADABTC".into()];

    let mut binance_ws: Websocket = Binance::new(None, None);

    // ** Change the variant wrapping the input symbols passed to the subscribe function to change the stream type!
    // For e.g., to do aggregated trades instead:
    // let sub_id = binance_ws.subscribe(WebsocketStreamType::AggregatedTrades(symbols)).await?;
    println!("Subscribing...");
    let sub_id = binance_ws
        .subscribe(WebsocketStreamType::BookTicker(symbols))
        .await?;

    println!("Waiting...");
    while let Some(event) = binance_ws
        .try_next()
        .await
        .expect("Didn't receive next transmit")
    {
        match event {
            // Other events that use same format:
            WebsocketEvent::BookTicker(BookTickerEvent {
                update_id,
                symbol,
                best_bid,
                ..
            }) => {
                println!("{best_bid}, {symbol}, {update_id}");
            }
            other => {
                println!("Other event received: {other:?}")
            }
        }
    }

    println!("Unsubscribing...");
    binance_ws.unsubscribe(sub_id);

    println!("Done");
    Ok(())
}
