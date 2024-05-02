use embedded_hal::digital::InputPin;

pub trait PosSensor: crate::types::MovementOrientation {
    fn position(&self) -> ();
    fn velocity(&self) -> ();
    // fn acceleration(&self) -> ();

    // for now, assuming an auto-magic update using pulse-counting hardware
    // fn update_pos(&mut self);

    fn is_homed(&self) -> bool;
}

pub struct ABEncoder<A: InputPin, B: InputPin> {
    enc_a: A,
    enc_b: B,
    ppr: u16,
    // Index pin
    // todo: compile time configured presence of idx pin

    // counter setup
    pulse_counter: u32,
    pulse_timestamp: u64,

    // velocity calc
    prev_th: u32,
    pulse_per_second: u32,
    prev_ts_us: u64,
}
