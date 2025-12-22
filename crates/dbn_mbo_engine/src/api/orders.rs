use std::collections::HashMap;

use crate::api::action::OrderRequest;

pub struct Orders {
    _queue_by_id: HashMap<u64, OrderRequest>,
    _active_by_id: HashMap<u64, OrderRequest>,
}
