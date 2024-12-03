#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;

 use defmt_rtt as _;
 // use defmt::info;


// use fixed::types::I16F16;
use sfoc_rs::{

    bldc_driver::MotorPins, esp32_sfoc::posn_encoder, foc_control::{FOController, PhaseAngle}, rexports::{discrete_count::re_exports::fixed::types::I16F16, foc}
};
use sfoc_rs::esp32_sfoc::device::Esp3PWM;

#[entry]
fn main() -> ! {

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let pin1 = peripherals.GPIO1;
    let pin2 = peripherals.GPIO2;
    let pin3 = peripherals.GPIO3;
    let pin4 = peripherals.GPIO4;
    let pin5 = peripherals.GPIO5;

    let mut driver: Esp3PWM<'_, _, posn_encoder::EncoderPosn<'_, 0>> = Esp3PWM::new(
        peripherals.MCPWM0,
        peripherals.PCNT,
        (pin1, pin2, pin3),
        (pin4, pin5),
    );

    FOController::set_phase_voltage(
        &mut driver,
        foc::park_clarke::RotatingReferenceFrame {
            d: I16F16::ZERO,
            q: I16F16::ONE,
        },
        PhaseAngle(I16F16::PI),
    );
    MotorPins::set_zero(&mut driver);
    loop {}
}
