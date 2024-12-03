#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::peripheral::Peripheral;
use esp_hal::peripherals::TIMG0;
use esp_hal::prelude::*;

use defmt_rtt as _;
// use defmt::info;

// use fixed::types::I16F16;
use sfoc_rs::esp32_sfoc::device::Esp3PWM;
use sfoc_rs::esp32_sfoc::time_source::TimerHolder;
use sfoc_rs::{
    bldc_driver::MotorPins,
    foc_control::{FOController, PhaseAngle},
    rexports::{discrete_count::re_exports::fixed::types::I16F16, foc},
};

use esp_hal::{
    timer::timg::{Timer, Timer0, TimerGroup},
    Blocking,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0: TimerGroup<'_, <TIMG0 as Peripheral>::P, Blocking> =
        TimerGroup::new(peripherals.TIMG0);
    let timer0: Timer<Timer0<<TIMG0 as Peripheral>::P>, Blocking> = timg0.timer0;
    let timer0: TimerHolder<<TIMG0 as Peripheral>::P> = TimerHolder::init(timer0);

    let pin1 = peripherals.GPIO1;
    let pin2 = peripherals.GPIO2;
    let pin3 = peripherals.GPIO3;
    // let pin4 = peripherals.GPIO4;
    // let pin5 = peripherals.GPIO5;

    let mut driver: Esp3PWM<'_, _> = Esp3PWM::init(
        peripherals.MCPWM0,
        peripherals.PCNT,
        (pin1, pin2, pin3),
        // (pin4, pin5),
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
    // driver.get_position_um();
    #[allow(clippy::empty_loop)]
    generic_spinny::main_loop(driver, timer0);
}
