use embedded_hal::digital::InputPin;
use embedded_time::{Clock, Instant};

pub trait PosSensor<C: Clock> {
    // TODO: encapsulate the notion of position as a type (mm, radians, encoder increments, etc.
    /// For determining velocity. use seconds, clock oscilations, etc at your leasure
    fn position(&mut self, clock: &C) -> u32;
    fn velocity(&self) -> (u32, u32);
    // fn acceleration(&self) -> ();

    // for now, assuming an auto-magic update using pulse-counting hardware
    // fn update_pos(&mut self);
}

struct VelocityBook<C: Clock> {
    pos_prev: u32,
    _pos_prev_ts: Instant<C>,
}

impl<C: Clock> PosSensor<C> for VelocityBook<C> {
    // get a timestamp/position pair
    // this is typically taken each time in the foc hot-loop
    fn position(&mut self, clock: &C) -> u32 {
        self._pos_prev_ts = clock.try_now().unwrap();
        // self.pos_prev = $read_position;

        self.pos_prev
    }

    // typically only sampled in the move-loop for the velocity PID
    // TODO: encapsulate velocity, and its units, within the type-system
    fn velocity(&self) -> (u32, u32) {
        todo!("there seems to be some delicacy to this. need to do this with care");
    }
}

pub trait ABEncoder<InA: InputPin, InB: InputPin> {
    /// e.g. for esp32, this will be an i32
    type Output;
    type InitData;
    fn init(init_data: Self::InitData) -> Self;
    fn read(&self) -> Self::Output;
}
