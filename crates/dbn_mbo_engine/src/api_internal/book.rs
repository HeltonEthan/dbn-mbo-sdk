use dbn::Action;
use std::collections::HashMap;

use crate::api_internal::order::Order;

#[derive(Debug, Default)]
pub struct Queue {
    orders_by_id: HashMap<u64, (Action, Order)>,
}

impl Queue {
    pub fn apply(&mut self) {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Active {
    orders_by_id: HashMap<u64, (Action, Order)>,
}

impl Active {
    pub fn apply(&mut self) {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Inactive {
    orders_by_id: HashMap<u64, (Action, Order)>,
}

impl Inactive {
    pub fn apply(&mut self) {
        todo!()
    }
}
