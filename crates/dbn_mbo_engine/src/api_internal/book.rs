use dbn::Action;
use std::collections::{BTreeMap, HashMap};

use crate::api_internal::order::Order;

#[derive(Debug, Default)]
pub struct Book {
    orders_by_id: HashMap<(u64, Action), Order>,
    queue: BTreeMap<u64, Order>,
    active: BTreeMap<u64, Order>,
}

impl Book {
    pub fn apply(&mut self, action: Action, order: Order) {
        match action {
            Action::Trade => {},
            Action::Cancel => {},
            Action::Modify => {},
            _ => {},
        }
    }

    fn trade(&mut self) {
        todo!()
    }

    fn cancel(&mut self) {
        todo!()
    }

    fn modify(&mut self) {
        todo!()
    }

    fn clear(&mut self) {
        self.orders_by_id.clear()
    }
}
