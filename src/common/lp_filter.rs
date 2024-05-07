use embedded_time::{
    duration::{self, Microseconds}, Clock,
};

use super::types::HalClock;

type Timestamp = duration::Generic<u32>;

#[allow(dead_code)]
pub struct LPFilter {
    pub time_constant: Microseconds<u32>,
    timestamp_prev: Timestamp,
    y_prev: f32,
}

impl LPFilter {
    pub fn new(time_constant: Microseconds<u32>, clock: HalClock) -> Self {
        Self {
            time_constant,
            timestamp_prev: clock
                .try_now()
                .unwrap()
                .duration_since_epoch()
                .try_into()
                .unwrap(),
            y_prev: 0.0,
        }
    }
    pub fn run(&mut self, _clock: &HalClock) -> f32 {
        todo!()
    }
}
