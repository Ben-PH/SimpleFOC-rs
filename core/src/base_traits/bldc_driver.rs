use embedded_hal::pwm::SetDutyCycle;

use crate::common::helpers::{DutyCycle, Triplet};

/// Describes what a given phase/coil/inductor is doing
#[derive(Default, Copy, Clone)]
pub struct PhaseState {
    pub hi_side: bool,
    pub lo_side: bool,
}

pub trait MotorHiPins: Sized {
    fn set_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle);
    fn set_zero(&mut self);
}

impl<A: SetDutyCycle, B: SetDutyCycle, C: SetDutyCycle> MotorHiPins for Triplet<A, B, C> {
    fn set_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle) {
        let _ = SetDutyCycle::set_duty_cycle_fraction(
            &mut self.member_a,
            dc_a.numer(),
            dc_a.denom().into(),
        );
        let _ = SetDutyCycle::set_duty_cycle_fraction(
            &mut self.member_b,
            dc_b.numer(),
            dc_b.denom().into(),
        );
        let _ = SetDutyCycle::set_duty_cycle_fraction(
            &mut self.member_c,
            dc_c.numer(),
            dc_c.denom().into(),
        );
    }

    fn set_zero(&mut self) {
        let _ = SetDutyCycle::set_duty_cycle_fully_off(&mut self.member_a);
        let _ = SetDutyCycle::set_duty_cycle_fully_off(&mut self.member_a);
        let _ = SetDutyCycle::set_duty_cycle_fully_off(&mut self.member_a);
    }
}

// impl<A: SetDutyCycle, B: SetDutyCycle, C: SetDutyCycle> WriteDutyCycles for Triplet<A, B, C> {
//     type SetError = ();
//     fn write_pwm_duty(
//         &mut self,
//         duty_a: DutyCycle,
//         duty_b: DutyCycle,
//         duty_c: DutyCycle,
//     ) -> Result<(), Self::SetError> {
//         self.member_a
//             .set_duty_cycle_fraction(duty_a.numer(), duty_a.denom().into())
//             .map_err(|_| ())?;
//         self.member_b
//             .set_duty_cycle_fraction(duty_b.numer(), duty_b.denom().into())
//             .map_err(|_| ())?;
//         self.member_c
//             .set_duty_cycle_fraction(duty_c.numer(), duty_c.denom().into())
//             .map_err(|_| ())?;
//         Ok(())
//     }
// }
