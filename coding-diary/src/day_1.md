# Sessions

## Session 1 - Saturday 16 April 4:46-5:28

 * Read the challenge
 * Consider existing libraries for binance:
   + [binance-rs](https://github.com/wisespace-io/binance-rs) - Most recently updated (3 months ago) - personal project - **not async**
   + [binance_api_async](https://github.com/bigbizze/binance_api_async) - A lot more docos. Probably more than we need for the challenge - only support nightly rust due to [this line](https://github.com/bigbizze/binance_api_async/blob/master/src/lib.rs#L1)
     - I got something working in [experiment_1](./binance/experiment_1/src/main.rs)  working, but I prefer not to use nightly, and not to modify 3rd party libraries.
 * Consider websocket libraries
 * Read the [binance API reference](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md)
 * [Joined the discord](https://discord.com/channels/677525364829978625/677525365366980620)
 * Mucked around with the [example binance api](https://api.binance.com/api/v3/depth?symbol=ETHBTC)
 

## Session 2 - Sunday 17 April 08:01-08:48

### Plan

 1. Get websocket connection to binance
 2. Write model for binance
 3. Print out some binance data

## Session 3 - Sunday 17 April 12:41-14:35

 * Figured out the `websocket` crate didn't support moders futures - started with `tokio-tungstenite`
 * Got experiment_2 working - reading prices from binance

## Session 3 - Monday 18 April 08:20-09:30

 * Got binance library working with a test
 * Started bitstamp library

## Session 4 - Sat 23 April 10:40-

 * Changing the model library to support both binance and bitstamp
 * Changng binance to drop the last_updated_id, and instead add a timestamp of
   when the data arrived
