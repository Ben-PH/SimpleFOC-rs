use core::num::NonZeroU16;

pub const SQRT3: f32 = 1.732_050_807_57;
pub const _3PI_2: f32 = 4.712_388_980_38;
pub const RPM_TO_RADS: f32 = 0.104_719_755_12;
pub const _2_SQRT3: f32 = 1.154_700_538_38;
pub const _1_SQRT3: f32 = 0.577_350_269_19;
pub const SQRT3_2: f32 = 0.866_025_403_78;
pub const _120_D2R: f32 = 2.094_395_102_39;

#[derive(Default)]
pub struct Current(pub f32);
#[derive(Default)]
pub struct Voltage(pub f32);
pub struct DutyCycle {
    numer: u16,
    denom: NonZeroU16,
}

impl DutyCycle {
    pub const fn try_new(numer: u16, denom: NonZeroU16) -> Result<Self, ()> {
        if numer > denom.get() {
            Err(())
        } else {
            Ok(Self { numer, denom })
        }
    }
    pub fn numer(&self) -> u16 {
        self.numer
    }
    pub fn denom(&self) -> NonZeroU16 {
        self.denom
    }
}

/// Encapsulates the common pattern of three-way-coupling in 3-phase motors.
/// E.g. 3 pairs of pins to control an h-bridge. ADC reader pins. etc.
pub struct Triplet<A, B, C> {
    pub member_a: A,
    pub member_b: B,
    pub member_c: C,
}


/// Encapsulates the common pattern of two-way-coupling.
/// E.g. A pin-pair to control each side of an h-bridge, AB encoder pins, etc.
pub struct Couplet<A, B> {
    pub member_a: A,
    pub member_b: B,
}

pub struct DQCurrent {
    pub d: Current,
    pub q: Current,
}
#[derive(Default)]
pub struct PhaseVoltages {
    pub a: Voltage,
    pub b: Voltage,
    pub c: Voltage,
}
pub struct PhaseCurrent {
    pub a: Current,
    pub b: Current,
    pub c: Current,
}
// dq voltage structs
pub struct DQVoltage {
    pub d: Voltage,
    pub q: Voltage,
}
// alpha beta current structure
pub struct ABCurrent {
    pub alpha: Current,
    pub beta: Current,
}
