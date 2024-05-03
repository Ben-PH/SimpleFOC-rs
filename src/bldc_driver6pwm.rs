use embedded_hal::{digital::OutputPin, pwm::SetDutyCycle};

use crate::{
    base_traits::bldc_driver::{BLDCDriver, PhaseState},
    common::helpers::{PhaseVoltages, HBridgePins},
};

pub struct BldcDriver6PWM<
    Ah: SetDutyCycle,
    Al: SetDutyCycle,
    Bh: SetDutyCycle,
    Bl: SetDutyCycle,
    Ch: SetDutyCycle,
    Cl: SetDutyCycle,
    En: OutputPin,
> {
    a_phase: HBridgePins<Ah, Al>,
    b_phase: HBridgePins<Bh, Bl>,
    c_phase: HBridgePins<Ch, Cl>,
    enable: Option<En>,

    phase_states: [PhaseState; 3],
    phase_voltages: PhaseVoltages,
    duty_cycles: (f32, f32, f32),
}

impl<
        Ah: SetDutyCycle,
        Al: SetDutyCycle,
        Bh: SetDutyCycle,
        Bl: SetDutyCycle,
        Ch: SetDutyCycle,
        Cl: SetDutyCycle,
        En: OutputPin,
    > BLDCDriver for BldcDriver6PWM<Ah, Al, Bh, Bl, Ch, Cl, En>
{
    fn enable(&mut self) {
        if let Some(mut enable_pin) = self.enable {
            enable_pin.set_high();
        }
        self.set_phasestate(
            [PhaseState {
                hi_side: true,
                lo_side: true,
            }; 3],
        );
        self.set_pwm(Default::default());
    }

    fn disable(&mut self) {
        self.set_phasestate([PhaseState::default(); 3]);
        self.set_pwm(Default::default());
        if let Some(mut enable_pin) = self.enable {
            enable_pin.set_low();
        }
    }

    fn set_pwm(&mut self, voltages: PhaseVoltages) {
        // TODO: clamp between 0 and voltage limit

        self.phase_voltages = PhaseVoltages {
            a: todo!(),
            b: todo!(),
            c: todo!(),
        };
        // TODO: write in `_writeDutyCycle6PWM`
        todo!()
    }

    fn set_phasestate(&mut self, state: [PhaseState; 3]) {
        self.phase_states = state;
    }
}

impl<
        Ah: SetDutyCycle,
        Al: SetDutyCycle,
        Bh: SetDutyCycle,
        Bl: SetDutyCycle,
        Ch: SetDutyCycle,
        Cl: SetDutyCycle,
        En: OutputPin,
    > BldcDriver6PWM<Ah, Al, Bh, Bl, Ch, Cl, En>
{
    pub fn init(
        a_phase: HBridgePins<Ah, Al>,
        b_phase: HBridgePins<Bh, Bl>,
        c_phase: HBridgePins<Ch, Cl>,
        enable: Option<En>,
    ) -> Result<Self, ()> {
        // TODO: account for voltage-limit config-vals
        // TODO: hardware-specific configuration
        Ok(Self {
            a_phase,
            b_phase,
            c_phase,
            enable,

            duty_cycles: (0.0, 0.0, 0.0),
            phase_states: Default::default(),
            phase_voltages: Default::default(),
        })
    }
}
