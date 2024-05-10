#![no_std]
#![no_main]

use core::marker::PhantomData;

use esp_println::println;
use embedded_hal::{digital::InputPin, pwm::SetDutyCycle};
use embedded_time::{rate::Fraction, Clock, Instant};
use esp_backtrace as _;
use esp_hal::{
    pcnt::{PCNT, channel, channel::PcntSource, unit::{self, Unit}, },
    clock::{ClockControl, Clocks},
    gpio::IO,
    mcpwm::{
        operator::{PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        PeripheralClockConfig, MCPWM,
    },
    peripheral::Peripheral,
    peripherals::{Peripherals, MCPWM0, PCNT as PCNTPerif},
    prelude::{_esp_hal_gpio_OutputPin as EsOutputPin, *},
    timer::{Enable, TimerGroup, TimerGroupInstance},
    Blocking,
};

use sfoc_rs::{
    base_traits::{
        foc_control::{FOController, UnimpFOController},
        pos_sensor::ABEncoder,
    },
    common::types::VelocityPID,
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
    let group = TimerGroup::new(peripherals.TIMG0, &clocks, None);

    let time_src = Timer0::init(group.timer0);

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

    let mut v_pid = sfoc_rs::common::types::VelocityPID(sfoc_rs::pid_reexported::Pid::new(0.0, 6.0));
    v_pid.0.kp = 0.2;
    v_pid.0.ki = 2.0;
    v_pid.0.kd = 0.0;

    let _motor: UnimpFOController =
        FOController::init_fo_control(encoder_pins, (a, b, c), v_pid, time_src).unwrap();

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


struct EspPulsCounter<PinA, PinB> {
    reader: Unit,
    _ina: PhantomData<PinA>,
    _inb: PhantomData<PinB>
}

impl<A, B, Src> sfoc_rs::base_traits::pos_sensor::ABEncoder<Src> for EspPulsCounter<A, B>
where EspPulsCounter<A, B>: From<Src>
{
    type RawOutput = i32;

    fn read(&self) -> Self::RawOutput {
        todo!()
    }
}

impl From<(PCNT<'d>, PinA, PinB)> for EspPulsCounter<PinA, PinB> {
    fn from(value: (PCNT, PinA, B)) -> Self {
        let mut u0 = value.0.get_unit(unit::Number::Unit0);
        u0.configure(unit::Config {
            low_limit: i16::MIN,
            high_limit: i16::MAX,
            filter: None,
            ..Default::default()
        })

        .unwrap();

        println!("setup channel 0");
        let mut ch0 = u0.get_channel(channel::Number::Channel0);
        let mut pin_a = value.1;
        let mut pin_b = value.2;

        ch0.configure(
            PcntSource::from_pin(&mut pin_a),
            PcntSource::from_pin(&mut pin_b),
            channel::Config {
                lctrl_mode: channel::CtrlMode::Reverse,
                hctrl_mode: channel::CtrlMode::Keep,
                pos_edge: channel::EdgeMode::Decrement,
                neg_edge: channel::EdgeMode::Increment,
                invert_ctrl: false,
                invert_sig: false,
            },
        );

        println!("setup channel 1");
        let mut ch1 = u0.get_channel(channel::Number::Channel1);
        ch1.configure(
            PcntSource::from_pin(&mut pin_b),
            PcntSource::from_pin(&mut pin_a),
            channel::Config {
                lctrl_mode: channel::CtrlMode::Reverse,
                hctrl_mode: channel::CtrlMode::Keep,
                pos_edge: channel::EdgeMode::Increment,
                neg_edge: channel::EdgeMode::Decrement,
                invert_ctrl: false,
                invert_sig: false,
            },
        );
        let ticks: i32 = u0.get_value();
        Self {
            reader: u0,
            ..Default::default()
        }


    }
}
