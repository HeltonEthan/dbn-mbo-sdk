use super::*;
use dbn::Side;

use crate::api_internal::{market::Books, order::Order};

#[derive(Debug)]
pub struct ModifyRequest {
    pub instrument_id: u32,
    pub order_id: u64,
    pub new_price: Option<i64>,
    pub new_size: Option<u32>,
}

impl ModifyRequest {
    pub fn new(instrument_id: u32, order_id: u64, new_price: i64, new_size: u32) -> Self {
        Self {
            instrument_id,
            order_id,
            new_price: Some(new_price),
            new_size: Some(new_size),
        }
    }
}

impl Submit for ModifyRequest {
    // Need to make a getter that checks if the order exist and lets me set the side
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        let ts_event = mbo.ts_recv;
        let ts_recv = latency.ts_recv_sim(ts_event);
        let order = Order::new(ts_recv, ts_event, Side::None, self.new_price, self.new_size);
        match self.check_request() {
            Ack::Accepted => {
                Books::apply(self.instrument_id, order);
                Ack::Accepted
            },
            Ack::Rejected => Ack::Rejected,
        }
    }

    fn check_request(&self) -> Ack {
        Ack::Accepted
    }
}
