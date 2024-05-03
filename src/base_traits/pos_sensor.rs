use crate::common::types::MovementOrientation;

pub trait PosSensor: MovementOrientation {
    // TODO: setup return type
    fn position(&self);
    // TODO: setup return type
    fn velocity(&self);
    // fn acceleration(&self) -> ();

    // for now, assuming an auto-magic update using pulse-counting hardware
    // fn update_pos(&mut self);

    fn is_homed(&self) -> bool;
}
