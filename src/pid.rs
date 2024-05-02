use embedded_time::{
    clock::Error as ClockError,
    duration::{Microseconds, Nanoseconds},
    Clock, Instant, rate::Fraction,
};
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
            .ok_or(PIDError::NegativeTimeDelta)?.integer();
        let proportional = self.p * self.lookback.prev_error;

        let mut integral =
            self.lookback.prev_integral + self.i * (delta as f32) * 0.5 * (error + self.lookback.prev_error);

        integral.clamp(self.upper_limit, -self.upper_limit);

        todo!()
    }
}
