#![no_std]
#![no_main]

mod device;
mod posn_encoder;
mod time_source;

use device::Esp3PWM;
use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks},
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
};

use fixed::types::I16F16;
use sfoc_rs::base_traits::{
    bldc_driver::MotorPins,
    foc_control::{FOController, PhaseAngle},
};

#[esp_hal::entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clock_ctrl = ClockControl::boot_defaults(system.clock_control);
    let clocks: Clocks = clock_ctrl.freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = io.pins;

    let mut driver = Esp3PWM::new(
        &clocks,
        peripherals.MCPWM0,
        peripherals.PCNT,
        (pins.gpio1, pins.gpio2, pins.gpio3),
        (pins.gpio4, pins.gpio5),
    );

    FOController::set_phase_voltage(
        &mut driver,
        foc::park_clarke::MovingReferenceFrame {
            d: I16F16::ZERO,
            q: I16F16::ONE,
        },
        PhaseAngle(I16F16::PI),
    );
    MotorPins::set_zero(&mut driver);
    loop {}
}
