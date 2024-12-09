#![no_std]
#![no_main]

use defmt::warn;
use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};

use defmt::info;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let delay = Delay::new();
    loop {
        warn!("Hello world!");
        delay.delay(500.millis());
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin
}
