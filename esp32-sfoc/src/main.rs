#![no_std]
#![no_main]

use embedded_hal::digital::InputPin;
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, gpio::IO};
use sfoc_rs::base_traits::pos_sensor::ABEncoder;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut ina = io.pins.gpio8.into_pull_up_input();
    let mut inb = io.pins.gpio9.into_pull_up_input();
    let timer_init = MyABInit{ ina, inb };
    let encoder = MyABEncoder::init(timer_init);

    let target_velocity = sfoc_rs::common::types::Velocity{displacement: 0, time: 0};

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

impl<InA: InputPin, InB: InputPin> ABEncoder<InA, InB>
    for MyABEncoder<InA, InB>
{
    type Output = i32;
    type InitData = MyABInit<InA, InB>;

    fn init(init_data: Self::InitData) -> Self {
        Self{ ina: init_data.ina, inb: init_data.inb }
    }

    fn read(&self) -> Self::Output {
        todo!()
    }
}
