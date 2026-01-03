pub mod model;

pub trait LatencyModel {
    fn ts_recv(&self, ts_send: &u64) -> u64;
}
