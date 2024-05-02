use embedded_time::{clock::Error as ClockError, rate::Fraction, Clock, Instant};
enum PIDError {
    Clock(ClockError),
    NegativeTimeDelta,
}
impl From<ClockError> for PIDError {
    fn from(value: ClockError) -> Self {
        Self::Clock(value)
    }
}

pub struct PID<C: Clock> {
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub output_ramp: Option<f32>,
    pub upper_limit: f32,
    lookback: PIDLookBack<C>,
}

struct PIDLookBack<C: Clock> {
    prev_error: f32,
    prev_output: f32,
    prev_integral: f32,
    prev_timestamp: Instant<C>,
}

impl<C: Clock> PIDLookBack<C> {
    fn new(clock: &C) -> Self {
        Self {
            prev_error: 0.0,
            prev_output: 0.0,
            prev_integral: 0.0,
            prev_timestamp: clock.try_now().unwrap(),
        }
    }

    fn reset(&self) {
        self.prev_error = 0.0;
        self.prev_output = 0.0;
        self.prev_integral = 0.0;
    }
}

struct ClockStub;
impl Clock for ClockStub {
    type T = u32;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1);

    fn try_now(&self) -> Result<Instant<Self>, ClockError> {
        todo!()
    }
}

impl PID<ClockStub> {
    pub fn init(
        p: f32,
        i: f32,
        d: f32,
        output_ramp: Option<f32>,
        upper_limit: f32,
        clock: &ClockStub,
    ) -> Self {
        Self {
            p,
            i,
            d,
            output_ramp,
            upper_limit,
            lookback: PIDLookBack::new(clock),
        }
    }
    pub fn run(&self, clock: &ClockStub, error: f32) -> Result<f32, PIDError> {
        let now = clock.try_now()?;
        let delta: u32 = now
            .checked_duration_since(&self.lookback.prev_timestamp)
            .ok_or(PIDError::NegativeTimeDelta)?
            .integer();
        let proportional = self.p * self.lookback.prev_error;

        let mut integral = {
            let mut stashed = self.lookback.prev_integral
                + self.i * (delta as f32) * 0.5 * (error + self.lookback.prev_error);
            stashed.clamp(self.upper_limit, -self.upper_limit);
            stashed
        };

        let derivitive = self.d * (error - self.lookback.prev_error) / (delta as f32);

        let mut output = {
            let mut stashed = proportional + integral + derivitive;
            stashed.clamp(self.upper_limit, -self.upper_limit);
            stashed
        };
        if let Some(ramp) = self.output_ramp {
            let mut op_rate = (output - self.lookback.prev_output) / (delta as f32);
            if op_rate > ramp {
                output = self.lookback.prev_output + ramp * (delta as f32);
            } else if op_rate < -ramp {
                output = self.lookback.prev_output - ramp * (delta as f32);
            }
        }

        self.lookback.prev_integral = integral;
        self.lookback.prev_output = output;
        self.lookback.prev_error = error;
        self.lookback.prev_timestamp = now;
        Ok(output)
    }
    pub fn reset(&mut self) {
        self.lookback.reset();
    }
}
