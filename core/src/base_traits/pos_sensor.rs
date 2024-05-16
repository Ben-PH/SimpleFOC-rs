use embedded_hal::digital::InputPin;
use embedded_time::{Clock, Instant};

pub trait PosSensor<C: Clock> {
    // This would platform-specific compatable. for now, my thinking is based on esp32 pulse-count peripheral
    type Peripheral;
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

pub trait ABEncoder {
    /// e.g. for esp32, this will be an i32
    type RawOutput;
    fn read(&self) -> Self::RawOutput;
}

