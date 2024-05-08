use embedded_hal::pwm::SetDutyCycle;
use embedded_time::rate::Rate;

use crate::common::helpers::{DutyCycle, PhaseVoltages, PinTriplet};

/// Describes what a given phase/coil/inductor is doing
#[derive(Default, Copy, Clone)]
pub struct PhaseState {
    pub hi_side: bool,
    pub lo_side: bool,
}

// For hardware-specific cfg initialisation
pub trait ConfigPWM {
    type Params;
    fn config<A, B, C, R: Rate>(
        freq: R,
        pins: PinTriplet<A, B, C>,
    ) -> Result<Self::Params, PinTriplet<A, B, C>>;
}

// for bldc: 3 and 6 pins. For now, assuming just 3
#[allow(non_camel_case_types)]
pub trait BLDCDriver
// <
//     mVPsup: Unsigned,
//     mVLim: Unsigned + IsLessOrEqual<mVPsup, Output = typenum::True>,
// >
: Sized + WriteDutyCycles + ConfigPWM<Params = Self>
{
    // TODO: The constraints that I need to be able to encapsulate here:
    //  - the pins can be turned into pwm pins
    //  - the pins are to be moved into the returned self
    //  - there must be an encapsulation of hw-specifics. This is to be returned by
    //  `ConfigPWM::config`
    //   - in sfoc esp32, this is a pointer to `SP32MCPWMDriversParams`
    fn init_bldc_driver<A, B, C, R: Rate>(
        freq: R,
        pins: PinTriplet<A, B, C>,
    ) -> Result<Self, PinTriplet<A, B, C>> {
        // sfoc sets the pins to output, which is what SetDutyCycle constraint enforces
        // then does some things if the enable stuff is done
        // then sanity checks the v-limit config. Here, we do that at the type-level
        // 6pwm sets all phase-states to off
        // then defers to the hw-specific api to configure
        Self::config(freq, pins)
    }

    #[allow(unreachable_code)]
    fn set_pwm(
        &mut self,
        _voltages: PhaseVoltages,
    ) -> Result<(), <Self as WriteDutyCycles>::SetError> {
        todo!("constrain the phase voltages to be between 0 and 100%");
        todo!("apply the duty cycles as a coeficient to the phase voltages");
        let (_a, _b, _c): (DutyCycle, DutyCycle, DutyCycle) =
            todo!("constrain the duty cycles to between 0 and 100%");

        <Self as WriteDutyCycles>::write_pwm_duty(self, _a, _b, _c)
    }

    // In 3PWM, it's a bit weird. in PWM6, it's a simple `self.phasestate = state;`
    fn set_phasestate(&mut self, state: [PhaseState; 3]);
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
}

// TODO: Consider implementing "read current duty cycle"
pub trait WriteDutyCycles {
    type SetError;
    fn write_pwm_duty(
        &mut self,
        duty_a: DutyCycle,
        duty_b: DutyCycle,
        duty_c: DutyCycle,
    ) -> Result<(), Self::SetError>;
}

impl<A: SetDutyCycle, B: SetDutyCycle, C: SetDutyCycle> WriteDutyCycles for PinTriplet<A, B, C> {
    type SetError = ();
    fn write_pwm_duty(
        &mut self,
        duty_a: DutyCycle,
        duty_b: DutyCycle,
        duty_c: DutyCycle,
    ) -> Result<(), Self::SetError> {
        self.pin_a
            .set_duty_cycle_fraction(duty_a.numer(), duty_a.denom().into())
            .map_err(|_| ())?;
        self.pin_b
            .set_duty_cycle_fraction(duty_b.numer(), duty_b.denom().into())
            .map_err(|_| ())?;
        self.pin_c
            .set_duty_cycle_fraction(duty_c.numer(), duty_c.denom().into())
            .map_err(|_| ())?;
        Ok(())
    }
}
