use crate::common::types::MovementOrientation;

pub trait PosSensor: MovementOrientation {
    fn position(&self) -> ();
    fn velocity(&self) -> ();
    // fn acceleration(&self) -> ();

    // for now, assuming an auto-magic update using pulse-counting hardware
    // fn update_pos(&mut self);

    fn is_homed(&self) -> bool;
}
