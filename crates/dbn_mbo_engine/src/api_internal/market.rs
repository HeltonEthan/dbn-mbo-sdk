use std::collections::HashMap;

use crate::api_internal::{
    book::{Active, Inactive, Queue},
    order::Order,
};

#[derive(Debug, Default)]
pub struct Books {
    queue: HashMap<u32, Queue>,
    active: HashMap<u32, Active>,
    inactive: HashMap<u32, Inactive>,
}

impl Books {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(instrument_id: u32, order: Order) {
        todo!()
    }
}
