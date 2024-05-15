#![no_std]
#![no_main]

use embedded_hal::{digital::OutputPin, pwm::SetDutyCycle};
use embedded_time::{rate::Fraction, Instant};
use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks},
    gpio::IO,
    mcpwm::{
        operator::{PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        timer::PwmWorkingMode,
        PeripheralClockConfig, PwmPeripheral, MCPWM,
    },
    pcnt::unit::Unit,
    peripheral::Peripheral,
    peripherals::{Peripherals, MCPWM0, PCNT},
    prelude::*,
    timer::{Enable, TimerGroup, TimerGroupInstance},
    Blocking,
};

use sfoc_rs::{base_traits::bldc_driver::BLDCDriver, common::helpers::PinTriplet};

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

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = io.pins;

    let _driver = Esp3PWM::new(
        &clocks,
        peripherals.TIMG0,
        peripherals.MCPWM0,
        peripherals.PCNT,
        pins.gpio1,
        pins.gpio2,
        pins.gpio3,
        // pins.gpio4,
        // pins.gpio5,
    );

    let mut v_pid =
        sfoc_rs::common::types::VelocityPID(sfoc_rs::pid_reexported::Pid::new(0.0, 6.0));
    v_pid.0.kp = 0.2;
    v_pid.0.ki = 2.0;
    v_pid.0.kd = 0.0;

    loop {}
}

struct Esp3PWM<
    'd,
    PwmOp: PwmPeripheral,
    PinA: esp_hal::gpio::OutputPin,
    PinB: esp_hal::gpio::OutputPin,
    PinC: esp_hal::gpio::OutputPin,
    // const EnA: u8,
    // const EnB: u8,
> {
    // mcpwm_periph: MCPWM<'d, PwmOp>,
    pin_a: PwmPin<'d, PinA, PwmOp, 0, true>,
    pin_b: PwmPin<'d, PinB, PwmOp, 1, true>,
    pin_c: PwmPin<'d, PinC, PwmOp, 2, true>,
    // enc_a: GpioPin<Input<PullUp>, EnA>,
    // enc_b: GpioPin<Input<PullUp>, EnA>,
}

impl<
        'd,
        PwmOp: PwmPeripheral,
        PinA: esp_hal::gpio::OutputPin,
        PinB: esp_hal::gpio::OutputPin,
        PinC: esp_hal::gpio::OutputPin,
        // const EnA: u8,
        // const EnB: u8,
    > Esp3PWM<'d, PwmOp, PinA, PinB, PinC>
{
    fn new(
        clocks: &Clocks<'_>,
        timg_choice: impl Peripheral<P = impl TimerGroupInstance> + 'd,
        mcpwm_choice: impl Peripheral<P = PwmOp> + 'd,
        _pcnt: impl Peripheral<P = PCNT> + 'd,
        pin_a: impl Peripheral<P = PinA> + 'd,
        pin_b: impl Peripheral<P = PinB> + 'd,
        pin_c: impl Peripheral<P = PinC> + 'd,
        // enc_a: impl Peripheral<P = GpioPin<Unknown, EnA>> + 'd,
        // enc_b: impl Peripheral<P = GpioPin<Unknown, EnB>> + 'd,
    ) -> Self {
        let timg0 = TimerGroup::new(timg_choice, &clocks, None);
        let _time_src = Timer0::init(timg0.timer0);
        let clock_cfg = PeripheralClockConfig::with_frequency(&clocks, 40.MHz()).unwrap();

        let pin_config =
            || PwmPinConfig::<true>::new(PwmActions::empty(), PwmUpdateMethod::empty());
        let mut mcpwm_periph = MCPWM::new(mcpwm_choice, clock_cfg);
        mcpwm_periph.operator0.set_timer(&mcpwm_periph.timer0);
        mcpwm_periph.operator1.set_timer(&mcpwm_periph.timer0);
        mcpwm_periph.operator2.set_timer(&mcpwm_periph.timer0);
        let pin_a: PwmPin<'d, PinA, _, 0, true> =
            mcpwm_periph.operator0.with_pin_a(pin_a, pin_config());
        let pin_b: PwmPin<'d, PinB, _, 1, true> =
            mcpwm_periph.operator1.with_pin_a(pin_b, pin_config());
        let pin_c: PwmPin<'d, PinC, _, 2, true> =
            mcpwm_periph.operator2.with_pin_a(pin_c, pin_config());

        let pw_timer_cfg = clock_cfg
            .timer_clock_with_frequency(99, PwmWorkingMode::UpDown, 20.kHz())
            .unwrap();
        mcpwm_periph.timer0.start(pw_timer_cfg);

        // let enc_a = enc_a.into_pullup_input();
        // let enc_b = enc_b.into_pullup_input();

        let res = Self {
            // mcpwm_periph,
            pin_a,
            pin_b,
            pin_c,
            //     enc_a,
            //     enc_b,
        };
        // res.disable();
        res
    }
}

impl<
        'd,
        PwmOp: PwmPeripheral,
        PinA: SetDutyCycle + esp_hal::gpio::OutputPin,
        PinB: SetDutyCycle + esp_hal::gpio::OutputPin,
        PinC: SetDutyCycle + esp_hal::gpio::OutputPin,
        // const EnA: u8,
        // const EnB: u8,
    > BLDCDriver for Esp3PWM<'d, PwmOp, PinA, PinB, PinC>
{
    fn enable(&mut self) {
        todo!()
    }
    fn disable(&mut self) {
        todo!()
    }
    fn set_pwms(
        &mut self,
        dc_a: sfoc_rs::common::helpers::DutyCycle,
        dc_b: sfoc_rs::common::helpers::DutyCycle,
        dc_c: sfoc_rs::common::helpers::DutyCycle,
    ) {
        let _ = SetDutyCycle::set_duty_cycle_fraction(
            &mut self.pin_a,
            dc_a.numer(),
            dc_a.denom().into(),
        );
        let _ = SetDutyCycle::set_duty_cycle_fraction(
            &mut self.pin_b,
            dc_b.numer(),
            dc_b.denom().into(),
        );
        let _ = SetDutyCycle::set_duty_cycle_fraction(
            &mut self.pin_c,
            dc_c.numer(),
            dc_c.denom().into(),
        );
    }
    fn set_phase_state(
        &mut self,
        _ps_a: sfoc_rs::base_traits::bldc_driver::PhaseState,
        _ps_b: sfoc_rs::base_traits::bldc_driver::PhaseState,
        _ps_c: sfoc_rs::base_traits::bldc_driver::PhaseState,
    ) {
        todo!()
    }
}

fn _init_motor_pins<'d, PA, PB, PC>(
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

struct _EspPulsCounter {
    _reader_periph: Unit,
}

impl sfoc_rs::base_traits::pos_sensor::ABEncoder for _EspPulsCounter {
    type RawOutput = i16;

    fn read(&self) -> Self::RawOutput {
        todo!()
    }
}

// impl<'d> EspPulsCounter {
//     fn new(
//         pcnt_periph: PCNT,
//         pin_a: impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
//         pin_b: impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
//     ) -> Self {
//         let mut u0 = pcnt_periph.get_unit(unit::Number::Unit0);
//         u0.configure(unit::Config {
//             low_limit: i16::MIN,
//             high_limit: i16::MAX,
//             filter: None,
//             ..Default::default()
//         })
//         .unwrap();
//
//         println!("setup channel 0");
//         let mut ch0 = u0.get_channel(channel::Number::Channel0);
//
//         ch0.configure(
//             PcntSource::from_pin(pin_a),
//             PcntSource::from_pin(pin_b),
//             channel::Config {
//                 lctrl_mode: channel::CtrlMode::Reverse,
//                 hctrl_mode: channel::CtrlMode::Keep,
//                 pos_edge: channel::EdgeMode::Decrement,
//                 neg_edge: channel::EdgeMode::Increment,
//                 invert_ctrl: false,
//                 invert_sig: false,
//             },
//         );
//
//         println!("setup channel 1");
//         let mut ch1 = u0.get_channel(channel::Number::Channel1);
//         ch1.configure(
//             PcntSource::from_pin(pin_b),
//             PcntSource::from_pin(pin_a),
//             channel::Config {
//                 lctrl_mode: channel::CtrlMode::Reverse,
//                 hctrl_mode: channel::CtrlMode::Keep,
//                 pos_edge: channel::EdgeMode::Increment,
//                 neg_edge: channel::EdgeMode::Decrement,
//                 invert_ctrl: false,
//                 invert_sig: false,
//             },
//         );
//         let ticks: i16 = u0.get_value();
//         Self { reader_periph: u0 }
//     }
// }
