use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize, Serializer};

use crate::Error;

/// Events look like "bts:subscribe" - we're just formalizing that in rust here
#[derive(Display, FromStr, PartialEq, Debug, Deserialize)]
#[display(style = "snake_case")]
#[serde(try_from = "&str")]
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

impl TryFrom<&str> for Event {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .parse()
            .map_err(|source| Error::decoding("Parse event", value.to_string(), source))
    }
}
