use std::collections::{BTreeMap, VecDeque};

use crate::enums::Request;

#[derive(Debug, Default)]
pub struct WireQueue {
    pub queue: BTreeMap<u64, VecDeque<Request>>,
}

impl WireQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, request: Request) {
        let ts = request.ts_recv();
        self.queue.entry(ts).or_default().push_back(request);
    }
}

pub struct SimOrder {
    pub order_id: u64,
    pub instrument_id: u32,
    pub side: i8,
    pub price: i64,
    pub qty: u32,

    // state
    pub ts_recv: u64,
    pub ts_live: u64,
}

impl SimOrder {
    pub fn new() -> Self {
        todo!()
    }
}
