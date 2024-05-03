use embedded_time::{
    duration::{self, Microseconds},
    Clock,
};

use crate::types::HalClock;
type Timestamp = duration::Generic<u32>;

struct LPFilter {
    pub time_constant: Microseconds<u32>,
    timestamp_prev: Timestamp,
    y_prev: f32,
}

impl LPFilter {
    fn new(time_constant: Microseconds<u32>, clock: HalClock) -> Self {
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
    fn run(&mut self, clock: &HalClock) -> f32 {
        todo!()
    }
}
