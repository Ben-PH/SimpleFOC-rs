use embedded_hal::pwm::SetDutyCycle;

use crate::common::helpers::{DutyCycle, PhaseVoltages};

/// Describes what a given phase/coil/inductor is doing
#[derive(Default, Copy, Clone)]
pub struct PhaseState {
    pub hi_side: bool,
    pub lo_side: bool,
}

// for bldc: 3 and 6 pins.
pub trait BLDCDriver: Sized + WriteDutyCycles {
    // enable/disable pins are optional. will revisit this when I grok this
    // fn enable(&mut self) -> Result<(), ()> {
    //     self.set_pwm(
    //         DutyCycle::try_new(0, 1.into().unwrap()),
    //         DutyCycle::try_new(0, 1.into().unwrap()),
    //         DutyCycle::try_new(0, 1.into().unwrap()),
    //     )
    // }

    // fn disable(&mut self) -> Result<(), ()> {
    //     self.set_pwm(
    //         DutyCycle::try_new(0, 1.into().unwrap()),
    //         DutyCycle::try_new(0, 1.into().unwrap()),
    //         DutyCycle::try_new(0, 1.into().unwrap()),
    //     )
    // }
    fn set_pwm(
        &mut self,
        voltages: PhaseVoltages,
    ) -> Result<(), <Self as WriteDutyCycles>::SetError> {
        todo!("constrain the phase voltages to be between 0 and 100%");
        todo!("apply the duty cycles as a coeficient to the phase voltages");
        let (a, b, c): (DutyCycle, DutyCycle, DutyCycle) =
            todo!("constrain the duty cycles to between 0 and 100%");

        <Self as WriteDutyCycles>::write_pwm_duty(self, a, b, c)
    }

    // In 3PWM, it's a bit weird. in PWM6, it's a simple `self.phasestate = state;`
    fn set_phasestate(&mut self, state: [PhaseState; 3]);
}

pub trait WriteDutyCycles {
    type SetError;
    fn write_pwm_duty(
        &mut self,
        duty_a: DutyCycle,
        duty_b: DutyCycle,
        duty_c: DutyCycle,
    ) -> Result<(), Self::SetError>;
}

struct PWMPins<A: SetDutyCycle, B: SetDutyCycle, C: SetDutyCycle> {
    pin_a: A,
    pin_b: B,
    pin_c: C,
}

impl<A: SetDutyCycle<Error = ()>, B: SetDutyCycle<Error = ()>, C: SetDutyCycle<Error = ()>>
    WriteDutyCycles for PWMPins<A, B, C>
{
    type SetError = ();
    fn write_pwm_duty(
        &mut self,
        duty_a: DutyCycle,
        duty_b: DutyCycle,
        duty_c: DutyCycle,
    ) -> Result<(), Self::SetError> {
        self.pin_a
            .set_duty_cycle_fraction(duty_a.numer(), duty_a.denom().into())?;
        self.pin_b
            .set_duty_cycle_fraction(duty_b.numer(), duty_b.denom().into())?;
        self.pin_c
            .set_duty_cycle_fraction(duty_c.numer(), duty_c.denom().into())?;
        Ok(())
    }
}
