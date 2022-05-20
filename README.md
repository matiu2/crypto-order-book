# Key rock assignment

## Code walkthrough

 * binance - binance client library
 * bitstamp - bitstamp client library
 * server - Merges the streams of binance and bitstamp into a single order-book-summary stream
 * client - attaches to the server and prints out the orderbooks as they arrive
 * experiments - experiments done during development

## Demo

### Start the server

in a terminal:

```bash
cd server
cargo run --release
```

### Start the client

In a separate terminal start the client:

```bash
cd client
cargo run --relaese
```
