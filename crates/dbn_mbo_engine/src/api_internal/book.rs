use std::collections::HashMap;

use crate::api_internal::order::Order;

#[derive(Debug, Default)]
pub struct Queue {
    orders_by_id: HashMap<u64, Order>,
}

#[derive(Debug, Default)]
pub struct Active {
    orders_by_id: HashMap<u64, Order>,
}

#[derive(Debug, Default)]
pub struct Inactive {
    orders_by_id: HashMap<u64, Order>,
}
