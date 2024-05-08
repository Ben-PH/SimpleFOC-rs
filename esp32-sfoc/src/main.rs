#![no_std]
#![no_main]

use core::marker::PhantomData;

use embedded_hal::digital::InputPin;
use embedded_time::rate::Fraction;
use embedded_time::Instant;
use esp_backtrace as _;
use esp_hal::clock::Clocks;
use esp_hal::delay::Delay;
use esp_hal::timer::{Enable, TimerGroup, TimerGroupInstance};
use esp_hal::Blocking;
use esp_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*};
use fugit::Rate;
use sfoc_rs::base_traits::bldc_driver::BLDCDriver;
use sfoc_rs::base_traits::foc_motor::FOCMotor;
use sfoc_rs::base_traits::pos_sensor::ABEncoder;
use sfoc_rs::common::helpers::PinTriplet;
use sfoc_rs::common::types::Velocity;
use typenum::{IsEqual, Unsigned};

struct Timer0<
    TG: TimerGroupInstance,
> {
    timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>,
}

impl<TG: TimerGroupInstance>
    Timer0<TG>
{
    fn init(timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>) -> Self {
        timer.enable_peripheral();
        Self {
            timer,
        }
    }
}
impl<TG: TimerGroupInstance>
    embedded_time::Clock for Timer0<TG>
{
    type T = u64;

    const SCALING_FACTOR: Fraction = <Fraction>::new(1, 80_000_000);

    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        Ok(Instant::new(esp_hal::timer::Instance::now(&*self.timer)))
    }
}
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clock_ctrl = ClockControl::boot_defaults(system.clock_control);
    let clocks: Clocks = clock_ctrl.freeze();
    let group = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timer = Timer0::init(group.timer0);

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut ina = io.pins.gpio8.into_pull_up_input();
    let mut inb = io.pins.gpio9.into_pull_up_input();
    let enc_init_data = MyABInit { ina, inb };
    let encoder = MyABEncoder::init(enc_init_data);

    let mut phase_a = io.pins.gpio10.into_push_pull_output();
    let mut phase_b = io.pins.gpio11.into_push_pull_output();
    let mut phase_c = io.pins.gpio12.into_push_pull_output();
    let phase_pins = PinTriplet {
        pin_a: phase_a,
        pin_b: phase_b,
        pin_c: phase_c,
    };



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
