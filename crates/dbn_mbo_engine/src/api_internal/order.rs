use dbn::Side;

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
    side: Side,
    price: Option<i64>,
    size: Option<u32>,
    state: OrderState,
}

impl Order {
    pub fn new(ts_event: u64, ts_recv: u64, side: Side, price: Option<i64>, size: Option<u32>) -> Self {
        Self {
            ts_recv,
            ts_event,
            side,
            price,
            size,
            state: OrderState::Pending,
        }
    }
}
