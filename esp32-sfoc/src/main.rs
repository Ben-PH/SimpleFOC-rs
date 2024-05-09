#![no_std]
#![no_main]

use embedded_hal::digital::InputPin;
use embedded_time::{rate::Fraction, Instant};
use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks},
    gpio::IO,
    mcpwm::{
        operator::{PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        PeripheralClockConfig, MCPWM,
    },
    peripheral::Peripheral,
    peripherals::{Peripherals, MCPWM0},
    prelude::{_esp_hal_gpio_OutputPin as EsOutputPin, *},
    timer::{Enable, TimerGroupInstance},
    Blocking,
};

use sfoc_rs::base_traits::{
    foc_control::{FOController, UnimpFOController},
    pos_sensor::ABEncoder,
};

struct Timer0<TG: TimerGroupInstance> {
    timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>,
}

impl<TG: TimerGroupInstance> Timer0<TG> {
    fn init(timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>) -> Self {
        timer.enable_peripheral();
        Self { timer }
    }
}
impl<TG: TimerGroupInstance> embedded_time::Clock for Timer0<TG> {
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
    // let group = TimerGroup::new(peripherals.TIMG0, &clocks, None);

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = io.pins;

    let encoder_pins = (
        pins.gpio1.into_pull_up_input(),
        pins.gpio2.into_pull_up_input(),
    );

    let motor_pins = (
        pins.gpio3.into_push_pull_output(),
        pins.gpio4.into_push_pull_output(),
        pins.gpio5.into_push_pull_output(),
    );
    let clk_cfg = PeripheralClockConfig::with_frequency(&clocks, 40.MHz()).unwrap();
    let (a, b, c) = init_motor_pins(
        peripherals.MCPWM0,
        clk_cfg,
        motor_pins.0,
        motor_pins.1,
        motor_pins.2,
    );


    let mut v_pid = sfoc_rs::common::types::VelocityPID(sfoc_rs::pid::Pid::new(0.0, 6.0));
    v_pid.0.kp = 0.2;
    v_pid.0.ki = 2.0;
    v_pid.0.kd = 0.0;

    let _motor: UnimpFOController =
        FOController::init_fo_control(encoder_pins, (a, b, c), v_pid).unwrap();

    loop {}
}

fn init_motor_pins<'d, PA, PB, PC, PerA, PerB, PerC>(
    pwm_peripheral: MCPWM0,
    clk_cfg: PeripheralClockConfig,
    pa: PerA,
    pb: PerB,
    pc: PerC,
) -> (
    PwmPin<'d, PA, MCPWM0, 0, true>,
    PwmPin<'d, PB, MCPWM0, 1, true>,
    PwmPin<'d, PC, MCPWM0, 2, true>,
)
where
    PA: EsOutputPin,
    PB: EsOutputPin,
    PC: EsOutputPin,
    PerA: Peripheral<P = PA> + 'd,
    PerB: Peripheral<P = PB> + 'd,
    PerC: Peripheral<P = PC> + 'd,
{
    let pin_config = || PwmPinConfig::<true>::new(PwmActions::empty(), PwmUpdateMethod::empty());
    let mcpwm = MCPWM::new(pwm_peripheral, clk_cfg);
    let pin_a = mcpwm.operator0.with_pin_a(pa, pin_config());
    let pin_b = mcpwm.operator1.with_pin_a(pb, pin_config());
    let pin_c = mcpwm.operator2.with_pin_a(pc, pin_config());
    (pin_a, pin_b, pin_c)
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
