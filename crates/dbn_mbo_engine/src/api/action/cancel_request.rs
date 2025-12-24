use super::*;
use dbn::{Action, Side};

use crate::api_internal::{market::Market, order::Order};

#[derive(Debug)]
pub struct CancelRequest {
    pub instrument_id: u32,
    pub order_id: u64,
}

impl CancelRequest {}

impl Submit for CancelRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        let ts_event = mbo.ts_recv;
        let ts_recv = latency.ts_recv_sim(ts_event);
        let order = Order::new(ts_recv, ts_event, Side::None, None, None);
        match self.check_request() {
            Ack::Accepted => {
                Market::apply(self.instrument_id, Action::Cancel, order);
                Ack::Accepted
            },
            Ack::Rejected => Ack::Rejected,
        }
    }

    fn check_request(&self) -> Ack {
        Ack::Accepted
    }
}
