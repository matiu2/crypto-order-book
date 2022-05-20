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

## Other notes

 * The server listens on 127.0.0.1:8000
 * When testing it listens on the same port, so tests will fail if the server is running
 * tests come in two categories:
   + cargo test unit_test - Just run the offline tests - fast
   + cargo test web_test - Just run the online tests
   + cargo test - Run all the tests
