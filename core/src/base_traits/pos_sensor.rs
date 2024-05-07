pub trait PosSensor {
    /// Degrees, thetea, etc for rotary. mm, fractional, etc. for linear.
    /// Some consideration is being made that this is actually the output of the encoder, such
    /// as pulse-counts, and it's up to hardware-specific impl to xform that to a more real measure
    type PosUnit: Copy;
    /// For determining velocity. use seconds, clock oscilations, etc at your leasure
    type TimeUnit: Copy;
    fn position(&self) -> Self::PosUnit;
    fn velocity(&self) -> (Self::PosUnit, Self::TimeUnit);
    // fn acceleration(&self) -> ();

    // for now, assuming an auto-magic update using pulse-counting hardware
    // fn update_pos(&mut self);
}

struct VelocityBook<PosUnit, TimeUnit> {
    pos_prev: PosUnit,
    _pos_prev_ts: TimeUnit,
}

impl<PosUnit: Copy, TimeUnit: Copy> PosSensor for VelocityBook<PosUnit, TimeUnit> {
    type PosUnit = PosUnit;
    type TimeUnit = TimeUnit;

    fn position(&self) -> Self::PosUnit {
        self.pos_prev
    }

    fn velocity(&self) -> (Self::PosUnit, Self::TimeUnit) {
        todo!("there seems to be some delicacy to this. need to do this with care");
    }
}
