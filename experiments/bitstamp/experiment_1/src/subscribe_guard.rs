//! Implements a guard so you don't forget to unsubscribe

use crate::model::{Data, Event};

pub struct Guard {
    data: Data,
}

impl Guard {
    pub fn new() -> Guard {}
}
