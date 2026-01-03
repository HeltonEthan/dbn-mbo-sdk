use crate::{execution::latency::LatencyModel, stream::hotloop::Mbo};

pub trait Process {
    fn submit<L: LatencyModel>(&mut self, mbo: &Mbo, latency: &L);
}

#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub ts_send: u64,
    pub instrument_id: u32,
    pub side: i8,
    pub price: i64,
    pub size: u32,
}

impl Trade {
    pub fn new(side: i8, price: i64, size: u32) -> Self {
        Self {
            ts_send: 0,
            instrument_id: 0,
            side,
            price,
            size,
        }
    }
}

impl Process for Trade {
    fn submit<L: LatencyModel>(&mut self, mbo: &Mbo, latency: &L) {
        self.ts_send = mbo.ts_recv;
        self.instrument_id = mbo.instrument_id;
        let ts_recv = latency.ts_recv(&self.ts_send);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Modify {
    pub ts_send: u64,
    pub instrument_id: u32,
    pub price: Option<i64>,
    pub size: Option<u32>,
    pub order_id: u64,
}

impl Modify {
    pub fn new(price: Option<i64>, size: Option<u32>, order_id: u64) -> Self {
        Self {
            ts_send: 0,
            instrument_id: 0,
            price,
            size,
            order_id,
        }
    }
}

impl Process for Modify {
    fn submit<L: LatencyModel>(&mut self, mbo: &Mbo, latency: &L) {
        self.ts_send = mbo.ts_recv;
        self.instrument_id = mbo.instrument_id;
        let ts_recv = latency.ts_recv(&self.ts_send);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cancel {
    pub ts_send: u64,
    pub instrument_id: u32,
    pub order_id: u64,
    pub time_delta: u64,
}

impl Cancel {
    pub fn new(order_id: u64) -> Self {
        Self {
            ts_send: 0,
            instrument_id: 0,
            order_id,
            time_delta: 0,
        }
    }
}

impl Process for Cancel {
    fn submit<L: LatencyModel>(&mut self, mbo: &Mbo, latency: &L) {
        self.ts_send = mbo.ts_recv;
        self.instrument_id = mbo.instrument_id;
        let ts_recv = latency.ts_recv(&self.ts_send);
    }
}
