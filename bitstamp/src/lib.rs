pub mod error;
pub use error::BitstampError as Error;
pub use error::Context;
pub mod order_book_client;

pub type Result<T> = std::result::Result<T, Error>;
pub mod model;
pub use crate::model::OrderBookData;
