use super::*;
use dbn::Side;

use crate::api_internal::{market::Books, order::Order};

#[derive(Debug)]
pub struct TradeRequest {
    pub instrument_id: u32,
    pub side: Side,
    pub price: i64,
    pub size: u32,
}

impl TradeRequest {
    pub fn new(instrument_id: u32, side: Side, price: i64, size: u32) -> Self {
        Self {
            instrument_id,
            side,
            price,
            size,
        }
    }

    pub fn check_price(&self) -> Ack {
        if self.price < 0 {
            return Ack::Rejected;
        }
        Ack::Accepted
    }
}

impl Submit for TradeRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        let ts_event = mbo.ts_recv;
        let ts_recv = latency.ts_recv_sim(ts_event);
        let order = Order::new(ts_recv, ts_event, self.side, Some(self.price), Some(self.size));
        match self.check_request() {
            Ack::Accepted => {
                Books::apply(self.instrument_id, order);
                Ack::Accepted
            },
            Ack::Rejected => Ack::Rejected,
        }
    }

    fn check_request(&self) -> Ack {
        self.check_price()
    }
}
