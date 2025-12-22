use rand::{
    Rng,
    distr::{OpenClosed01, StandardUniform},
};
use std::f64::consts::PI;

pub trait LatencyModel {
    fn ts_recv_sim(&mut self, ts_event: u64) -> u64;
}

/// Standard Normal Distributed latency model with a given a base latency and standard deviation,
/// base_latency in nano seconds
pub struct UnitNormalLatency {
    base_latency: f64,
    sigma: f64,
}

impl UnitNormalLatency {
    pub fn new(base_latency: u64, sigma: u64) -> Self {
        Self {
            base_latency: base_latency as f64,
            sigma: sigma as f64,
        }
    }

    fn sample_unit_normal(&mut self) -> f64 {
        let u1: f64 = rand::rng().sample(OpenClosed01);
        let u2: f64 = rand::rng().sample(StandardUniform);
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}

impl LatencyModel for UnitNormalLatency {
    fn ts_recv_sim(&mut self, ts_event: u64) -> u64 {
        let z = self.sample_unit_normal();
        let jitter = self.sigma * z;
        let latency = self.base_latency + jitter;
        let latency_ns = latency.max(0.0) as u64;
        ts_event + latency_ns
    }
}
