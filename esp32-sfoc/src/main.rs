#![no_std]
#![no_main]

use embedded_hal::digital::InputPin;
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*};
use sfoc_rs::base_traits::bldc_driver::BLDCDriver;
use sfoc_rs::base_traits::foc_motor::FOCMotor;
use sfoc_rs::base_traits::pos_sensor::ABEncoder;
use sfoc_rs::bldc_motor::BLDCMotor;
use sfoc_rs::common::helpers::PinTriplet;
use sfoc_rs::common::types::Velocity;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut ina = io.pins.gpio8.into_pull_up_input();
    let mut inb = io.pins.gpio9.into_pull_up_input();
    let mut phase_a = io.pins.gpio10.into_push_pull_output();
    let mut phase_b = io.pins.gpio11.into_push_pull_output();
    let mut phase_c = io.pins.gpio12.into_push_pull_output();
    let phase_pins = PinTriplet {
        pin_a: phase_a,
        pin_b: phase_b,
        pin_c: phase_c,
    };

    let timer_init = MyABInit { ina, inb };
    let encoder = MyABEncoder::init(timer_init);
    let motor = BLDCDriver::init_bldc_driver(
        todo!("Confirm `Rate` is the right trait here, and self-document"),
        phase_pins,
    );

    let foc = FOCMotor::init();

    let target_velocity = Velocity {
        displacement: 0u32,
        time: 1u16,
    };


    loop {}
}

struct MyABEncoder<InA: InputPin, InB: InputPin> {
    ina: InA,
    inb: InB,
}
struct MyABInit<InA: InputPin, InB: InputPin> {
    ina: InA,
    inb: InB,
}

impl<InA: InputPin, InB: InputPin> ABEncoder<InA, InB> for MyABEncoder<InA, InB> {
    type Output = i32;
    type InitData = MyABInit<InA, InB>;

    fn init(init_data: Self::InitData) -> Self {
        Self {
            ina: init_data.ina,
            inb: init_data.inb,
        }
    }

    fn read(&self) -> Self::Output {
        todo!()
    }
}
