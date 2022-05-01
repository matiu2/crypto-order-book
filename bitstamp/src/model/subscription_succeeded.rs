use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionSucceeded {
    channel: String,
}
