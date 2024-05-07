use embedded_time::{duration::Microseconds, Clock, Instant};

#[allow(dead_code)]
pub struct LPFilter<C: Clock> {
    pub time_constant: Microseconds<u32>,
    timestamp_prev: Instant<C>,
    y_prev: f32,
}

impl<C: Clock> LPFilter<C> {
    pub fn new(time_constant: Microseconds<u32>, clock: &C) -> Self {
        Self {
            time_constant,
            timestamp_prev: clock.try_now().unwrap(),
            y_prev: 0.0,
        }
    }
    pub fn run(&mut self, clock: &C, _x: f32) -> f32 {
        let now = clock.try_now().unwrap();
        let _dt = now.checked_duration_since(&self.timestamp_prev).unwrap();
        todo!("work out what the limit should be");
        if _dt.integer() > todo!() {
            self.y_prev = _x;
            self.timestamp_prev = now;
            return _x;
        }

        let alpha = 1.0; // self.time_constant.0 as f32 / (self.time_constant.0 as f32 + u32::from(dt.integer().into()) as f32);
        let y = alpha * self.y_prev + (1.0 - alpha) * _x;
        self.y_prev = y;
        self.timestamp_prev = now;

        _x
    }
}
