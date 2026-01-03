use rand::Rng;
use rand::distr::Uniform;
use std::f64::consts::PI;

use crate::execution::latency::LatencyModel;

#[derive(Clone, Copy, Debug)]
pub struct ShiftedLogNormal {
    mean_latency: f64,
    shift: f64,
    mu: f64,
    sigma: f64,
}

impl ShiftedLogNormal {
    pub fn new(mean_latency: f64) -> Self {
        Self {
            mean_latency,
            shift: 0.75,
            mu: -0.75,
            sigma: 0.75,
        }
    }

    fn sample_shifted_lognormal(&self) -> f64 {
        self.shift + (self.mu + self.sigma * Self::sample_box_muller()).exp()
    }

    fn sample_box_muller() -> f64 {
        let mut rng = rand::rng();
        let u1: f64 = rng.sample(Uniform::new(0.0, 1.0).unwrap());
        let u2: f64 = rng.sample(Uniform::new(0.0, 1.0).unwrap());
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}

impl LatencyModel for ShiftedLogNormal {
    fn ts_recv(&self, ts_send: &u64) -> u64 {
        let jitter = self.sample_shifted_lognormal() + self.mean_latency;
        *ts_send + jitter as u64
    }
}
