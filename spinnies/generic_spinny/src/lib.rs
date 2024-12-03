#![no_std]

use fugit::Duration;
use sfoc_rs::{
    common::helpers::DutyCycle,
    foc_control::FOController,
    rexports::{
        cordic,
        discrete_count::{
            re_exports::{
                fixed::types::I16F16,
                typenum::{NonZero, Unsigned},
            },
            CountReader, Counter,
        },
        foc::{self, park_clarke, pwm::Modulation},
    },
};
pub fn main_loop<R, FOC, TIM, PdNumer, PdDenom>(mut motor_pins: FOC, timer: TIM) -> !
where
    PdNumer: Unsigned,
    PdDenom: NonZero + Unsigned,
    FOC: FOController,
    R: CountReader,
    TIM: Counter<Reader = R, Measure = Duration<u64, PdNumer, PdDenom>>,
{
    loop {
        let elapsed: Duration<u64, PdNumer, PdDenom> = timer
            .try_read_measure()
            .unwrap_or_else(|_| panic!("should be never-fail for all implemented so far"));
        let millis = Duration::<u64, PdNumer, PdDenom>::to_millis(&elapsed) % 6000;

        let ms_fixed = I16F16::from_num(millis as u64);
        let ratio = ms_fixed / I16F16::from_num(6000);
        let angle = ratio * I16F16::TAU;
        let (sin, cos) = cordic::sin_cos(angle);

        // we are setting 10% in the quadrature (torque) and 0% direct-axis
        let qd = park_clarke::RotatingReferenceFrame {
            d: I16F16::ZERO,
            q: I16F16::from_num(0.1),
        };

        // Orient our q/d values to the phase-angle we desire
        let inv_parke = foc::park_clarke::inverse_park(cos, sin, qd);
        // and use the ideal, though computationally heavy, scale-vector calculation for desired
        // pwm duty-cycles, i.e. the percentage-of-maximum that each motor-phase will be set to
        let [dc_a, dc_b, dc_c] = foc::pwm::SpaceVector::modulate(inv_parke);

        //... and set them.
        motor_pins.set_pwms(DutyCycle(dc_a), DutyCycle(dc_b), DutyCycle(dc_c));
    }
}
