//! Model the json data we get as responses
//! Example input:
//!
//! [src/main.rs:26] data = Text(
//! "{\"event\":\"bts:subscription_succeeded\",\"channel\":\"live_orders_ethbtc\",\"data\":{}}",
//! )
//! [src/main.rs:26] data = Text(
//!     "{\"data\":{\"id\":1480340619440128,\"id_str\":\"1480340619440128\",\"order_type\":0,\"datetime\":\"1650246259\",\"microtimestamp\":\"1650246259036000\",\"amount\":10,\"amount_str\":\"10.00000000\",\"price\":0.07517475,\"price_str\":\"0.07517475\"},\"channel\":\"live_orders_ethbtc\",\"event\":\"order_deleted\"}",
//! )
//! [src/main.rs:26] data = Text(
//!     "{\"data\":{\"id\":1480340621578240,\"id_str\":\"1480340621578240\",\"order_type\":0,\"datetime\":\"1650246259\",\"microtimestamp\":\"1650246259222000\",\"amount\":10,\"amount_str\":\"10.00000000\",\"price\":0.07517475,\"price_str\":\"0.07517475\"},\"channel\":\"live_orders_ethbtc\",\"event\":\"order_created\"}",
//! )
//! [src/main.rs:26] data = Text(
//!     "{\"data\":{\"id\":1480340621578240,\"id_str\":\"1480340621578240\",\"order_type\":0,\"datetime\":\"1650246261\",\"microtimestamp\":\"1650246261101000\",\"amount\":10,\"amount_str\":\"10.00000000\",\"price\":0.07517475,\"price_str\":\"0.07517475\"},\"channel\":\"live_orders_ethbtc\",\"event\":\"order_deleted\"}",
//! )
//! [src/main.rs:26] data = Text(
//!     "{\"data\":{\"id\":1480340630052864,\"id_str\":\"1480340630052864\",\"order_type\":0,\"datetime\":\"1650246261\",\"microtimestamp\":\"1650246261291000\",\"amount\":10,\"amount_str\":\"10.00000000\",\"price\":0.07517475,\"price_str\":\"0.07517475\"},\"channel\":\"live_orders_ethbtc\",\"event\":\"order_created\"}",



/// Example json
/// {
///   "data": {
///     "id": 1480340630052864,
///     "id_str": "1480340630052864",
///     "order_type": 0,
///     "datetime": "1650246261",
///     "microtimestamp": "1650246261291000",
///     "amount": 10,
///     "amount_str": "10.00000000",
///     "price": 0.07517475,
///     "price_str": "0.07517475"
///   },
///   "channel": "live_orders_ethbtc",
///   "event": "order_created"
/// }
/// This struct just models the inner "data" part
pub struct Orders {
///     "id": 1480340630052864,
///     "id_str": "1480340630052864",
///     "order_type": 0,
///     "datetime": "1650246261",
///     "microtimestamp": "1650246261291000",
///     "amount": 10,
///     "amount_str": "10.00000000",
///     "price": 0.07517475,
///     "price_str": "0.07517475"

}
