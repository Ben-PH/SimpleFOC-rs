#![no_std]
#![no_main]

use core::marker::PhantomData;

use embedded_hal::{
    digital::{InputPin, OutputPin},
    pwm::SetDutyCycle,
};
use embedded_time::{rate::Fraction, Instant};
use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks},
    gpio::{GpioPin, Input, PullUp, Unknown, IO},
    mcpwm::{
        operator::{Operator, PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        timer::Timer as McTimer,
        PeripheralClockConfig, MCPWM,
    },
    pcnt::{
        channel,
        channel::PcntSource,
        unit::{self, Unit},
        PCNT,
    },
    peripheral::Peripheral,
    peripherals::{Peripherals, MCPWM0},
    prelude::*,
    timer::{Enable, TimerGroup, TimerGroupInstance},
    Blocking,
};
use esp_println::println;

use sfoc_rs::{
    base_traits::{
        bldc_driver::BLDCDriver,
        foc_control::{FOController, UnimpFOController},
    },
    common::helpers::PinTriplet,
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

    let enable_pins = (
        pins.gpio6.into_push_pull_output(),
        pins.gpio7.into_push_pull_output(),
        pins.gpio8.into_push_pull_output(),
    );
    let motor_pins = (
        pins.gpio3.into_push_pull_output(),
        pins.gpio4.into_push_pull_output(),
        pins.gpio5.into_push_pull_output(),
    );
    let clk_cfg = PeripheralClockConfig::with_frequency(&clocks, 40.MHz()).unwrap();
    let motor_pins = init_motor_pins(
        peripherals.MCPWM0,
        clk_cfg,
        motor_pins.0,
        motor_pins.1,
        motor_pins.2,
    );

    let mut v_pid =
        sfoc_rs::common::types::VelocityPID(sfoc_rs::pid_reexported::Pid::new(0.0, 6.0));
    v_pid.0.kp = 0.2;
    v_pid.0.ki = 2.0;
    v_pid.0.kd = 0.0;

    let _motor: UnimpFOController =
        FOController::init_fo_control(encoder_pins, motor_pins, v_pid, time_src).unwrap();

    loop {}
}

struct Esp3PWM<
    A: SetDutyCycle,
    B: SetDutyCycle,
    C: SetDutyCycle,
    EnA: OutputPin,
    EnB: OutputPin,
    EnC: OutputPin,
> {
    op0: Operator<0, MCPWM0>,
    timer0: McTimer<0, MCPWM0>,
    en_pins: (EnA, EnB, EnC),
    pwm_pins: (A, B, C),
}

impl<
        A: SetDutyCycle,
        B: SetDutyCycle,
        C: SetDutyCycle,
        EnA: OutputPin,
        EnB: OutputPin,
        EnC: OutputPin,
    > BLDCDriver for Esp3PWM<A, B, C, EnA, EnB, EnC>
{
    fn enable(&mut self) {
        self.en_pins.0.set_high();
        self.en_pins.1.set_high();
        self.en_pins.2.set_high();
    }
    fn disable(&mut self) {
        self.en_pins.0.set_low();
        self.en_pins.1.set_low();
        self.en_pins.2.set_low();
    }
    fn set_pwms(
        &mut self,
        dc_a: sfoc_rs::common::helpers::DutyCycle,
        dc_b: sfoc_rs::common::helpers::DutyCycle,
        dc_c: sfoc_rs::common::helpers::DutyCycle,
    ) {
        self.pwm_pins
            .0
            .set_duty_cycle_fraction(dc_a.numer(), dc_a.denom().into());
        self.pwm_pins
            .1
            .set_duty_cycle_fraction(dc_b.numer(), dc_b.denom().into());
        self.pwm_pins
            .2
            .set_duty_cycle_fraction(dc_c.numer(), dc_c.denom().into());
    }
    fn set_phase_state(
        &mut self,
        ps_a: sfoc_rs::base_traits::bldc_driver::PhaseState,
        ps_b: sfoc_rs::base_traits::bldc_driver::PhaseState,
        ps_c: sfoc_rs::base_traits::bldc_driver::PhaseState,
    ) {
        todo!()
    }
}

fn init_motor_pins<'d, PA, PB, PC>(
    pwm_peripheral: impl Peripheral<P = MCPWM0> + 'd,
    clk_cfg: PeripheralClockConfig,
    pa: impl Peripheral<P = PA> + 'd,
    pb: impl Peripheral<P = PB> + 'd,
    pc: impl Peripheral<P = PC> + 'd,
) -> PinTriplet<
    PwmPin<'d, PA, MCPWM0, 0, true>,
    PwmPin<'d, PB, MCPWM0, 1, true>,
    PwmPin<'d, PC, MCPWM0, 2, true>,
>
where
    PA: OutputPin + esp_hal::gpio::OutputPin,
    PB: OutputPin + esp_hal::gpio::OutputPin,
    PC: OutputPin + esp_hal::gpio::OutputPin,
{
    let pin_config = || PwmPinConfig::<true>::new(PwmActions::empty(), PwmUpdateMethod::empty());
    let mcpwm = MCPWM::new(pwm_peripheral, clk_cfg);
    let pin_a = mcpwm.operator0.with_pin_a(pa, pin_config());
    let pin_b = mcpwm.operator1.with_pin_a(pb, pin_config());
    let pin_c = mcpwm.operator2.with_pin_a(pc, pin_config());
    PinTriplet {
        pin_a,
        pin_b,
        pin_c,
    }
}

struct EspPulsCounter {
    reader_periph: Unit,
}

impl sfoc_rs::base_traits::pos_sensor::ABEncoder for EspPulsCounter {
    type RawOutput = i16;

    fn read(&self) -> Self::RawOutput {
        todo!()
    }
}

impl<'d> EspPulsCounter {
    fn new(
        pcnt_periph: PCNT,
        pin_a: impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
        pin_b: impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
    ) -> Self {
        let mut u0 = pcnt_periph.get_unit(unit::Number::Unit0);
        u0.configure(unit::Config {
            low_limit: i16::MIN,
            high_limit: i16::MAX,
            filter: None,
            ..Default::default()
        })
        .unwrap();

        println!("setup channel 0");
        let mut ch0 = u0.get_channel(channel::Number::Channel0);

        ch0.configure(
            PcntSource::from_pin(pin_a),
            PcntSource::from_pin(pin_b),
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
            PcntSource::from_pin(pin_b),
            PcntSource::from_pin(pin_a),
            channel::Config {
                lctrl_mode: channel::CtrlMode::Reverse,
                hctrl_mode: channel::CtrlMode::Keep,
                pos_edge: channel::EdgeMode::Increment,
                neg_edge: channel::EdgeMode::Decrement,
                invert_ctrl: false,
                invert_sig: false,
            },
        );
        let ticks: i16 = u0.get_value();
        Self { reader_periph: u0 }
    }
}
