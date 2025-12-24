use std::collections::HashMap;

use dbn::Action;

use crate::api_internal::{
    book::Book,
    order::Order,
};

#[derive(Debug, Default)]
pub struct Market {
    queue: HashMap<u32, Book>,
}

impl Market {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(instrument_id: u32, action: Action, order: Order) {
        todo!()
    }
}
