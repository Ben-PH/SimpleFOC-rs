use embedded_hal::pwm::SetDutyCycle;
use typenum::{IsGreater, True, Unsigned, U0};

use crate::common::helpers::{DutyCycle, Triplet};

/// Describes what a given phase/coil/inductor is doing
#[derive(Default, Copy, Clone)]
pub struct PhaseState {
    pub hi_side: bool,
    pub lo_side: bool,
}

pub trait MotorHiPins {
    fn set_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle);
    fn set_zero(&mut self);
}
pub trait VLimitedHiPins: MotorHiPins {
    /// TODO: go nightly, or wait for assosciated type defaults to stabalise, and set this to 120
    /// This is Deci-volts. easier than setting up a fixed point setup.
    /// If the scale here changes, account for the notion of users over-volting.
    type DeciVLimit: Unsigned + IsGreater<U0, Output = True>;
    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn set_limited_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle) {
        todo!("do the voltage clamp thing here");
        <Self as MotorHiPins>::set_pwms(self, dc_a, dc_b, dc_c)
    }
}

impl<A: SetDutyCycle, B: SetDutyCycle, C: SetDutyCycle> MotorHiPins for Triplet<A, B, C> {
    fn set_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle) {
        let _ = SetDutyCycle::set_duty_cycle_percent(&mut self.member_a, (dc_a.0 * 100.0) as u8);
        let _ = SetDutyCycle::set_duty_cycle_percent(&mut self.member_b, (dc_b.0 * 100.0) as u8);
        let _ = SetDutyCycle::set_duty_cycle_percent(&mut self.member_c, (dc_c.0 * 100.0) as u8);
    }

    fn set_zero(&mut self) {
        let _ = SetDutyCycle::set_duty_cycle_fully_off(&mut self.member_a);
        let _ = SetDutyCycle::set_duty_cycle_fully_off(&mut self.member_a);
        let _ = SetDutyCycle::set_duty_cycle_fully_off(&mut self.member_a);
    }
}
