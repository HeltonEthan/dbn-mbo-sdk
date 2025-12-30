use hashbrown::HashMap;
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

pub struct Queue {
    pub by_id: HashMap<u64, (i8, i64)>,
    pub bids: BTreeMap<i64, VecDeque<Order>>,
    pub asks: BTreeMap<i64, VecDeque<Order>>,
}

impl Queue {
    pub fn new() -> Self {
        todo!()
    }
}

pub struct FillLog {
    pub fills: Vec<Fill>,
    pub by_order: HashMap<u64, Vec<u32>>,
    pub by_instrument: HashMap<u32, Vec<u32>>,
    pub seq: u64,
}

pub struct Fill {
    pub ts_event: u64,
    pub instrument_id: u32,
    pub order_id: u64,
    pub side: i8,
    pub price: i64,
    pub qty: u32,
    pub fee: i64,
    pub liquidity_role: i8,
    pub match_id: u64,
}

pub struct Order {
    pub ts_recv: u64,
    pub order_id: u64,
    pub instrument_id: u32,
    pub side: i8,
    pub price: i64,
    pub qty: u32,
}

impl Order {
    pub fn new() -> Self {
        todo!()
    }
}
