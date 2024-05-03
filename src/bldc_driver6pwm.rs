use embedded_hal::{
    digital::{InputPin, OutputPin},
    pwm::SetDutyCycle,
};

use crate::{bldc_driver::PhaseState, foc_utils::PhaseVoltages};

pub struct BldcDriver6PWM<
    Ah: SetDutyCycle,
    Al: SetDutyCycle,
    Bh: SetDutyCycle,
    Bl: SetDutyCycle,
    Ch: SetDutyCycle,
    Cl: SetDutyCycle,
    En: OutputPin,
> {
    ah_pin: Ah,
    al_pin: Al,
    bh_pin: Bh,
    bl_pin: Bl,
    ch_pin: Ch,
    cl_pin: Cl,
    enable: Option<En>,

    phase_states: (PhaseState, PhaseState, PhaseState), 
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
        ah_pin: Ah,
        al_pin: Al,
        bh_pin: Bh,
        bl_pin: Bl,
        ch_pin: Ch,
        cl_pin: Cl,
        enable: Option<En>,
    ) -> Self {
        Self {
            ah_pin,
            al_pin,
            bh_pin,
            bl_pin,
            ch_pin,
            cl_pin,
            enable,
            phase_states: Default::default(),
        }
    }

    pub fn set_pwm(&mut self, voltages: PhaseVoltages) {
        todo!()
    }
    pub fn set_phase_states(states: (PhaseState, PhaseState, PhaseState)) {
        todo!()
    }
}

// #[derive(serde::Deserialize)]
// struct Foo {
//     bar: &'static str
// }
//
// fn read_in(in_read: impl Read) -> Vec<Foo> {
//     let mut res = vec![];
//     while let Ok(next) = in_read.read(&mut some_buff) {
//         res.push(serde_json::deserialise(next)?);
//     }
// }
