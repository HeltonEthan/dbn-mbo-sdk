use dbn::{Action, Side};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderState {
    Pending,
    Live,
    Done,
    Rejected,
    Canceled,
}

#[derive(Debug)]
pub struct Order {
    ts_recv: u64,
    ts_event: u64,
    instrument_id: u32,
    action: Action,
    side: Side,
    price: Option<i64>,
    size: Option<u32>,
    order_id: Option<u64>,
    state: OrderState,
}

impl Order {
    pub fn new(
        ts_event: u64,
        ts_recv: u64,
        instrument_id: u32,
        action: Action,
        side: Side,
        price: Option<i64>,
        size: Option<u32>,
    ) -> Self {
        Self {
            ts_recv,
            ts_event,
            instrument_id,
            action,
            side,
            price,
            size,
            order_id: None,
            state: OrderState::Pending,
        }
    }
}
