use core::marker::PhantomData;
use embedded_hal::pwm::SetDutyCycle;
use esp_backtrace as _;
use esp_hal::{
    clock::Clocks,
    mcpwm::{
        operator::{PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        timer::PwmWorkingMode,
        PeripheralClockConfig, PwmPeripheral, MCPWM,
    },
    pcnt::{channel, unit, PCNT},
    peripheral::Peripheral,
    peripherals,
    prelude::*,
};

use sfoc_rs::{
    base_traits::{bldc_driver::MotorPins, foc_control::FOController},
    common::helpers::Triplet,
};

use crate::posn_encoder::EncoderPosn;

pub struct Esp3PWM<'d, PwmOp, PinA, PinB, PinC, Pos> {
    // mcpwm_periph: MCPWM<'d, PwmOp>,
    motor_triplet: Triplet<
        PwmPin<'d, PinA, PwmOp, 0, true>,
        PwmPin<'d, PinB, PwmOp, 1, true>,
        PwmPin<'d, PinC, PwmOp, 2, true>,
    >,
    pulse_counter: PhantomData<Pos>,
    // time_src: PhantomData<Tim>,
    // _motion_ctrl: M,
}

impl<
        'd,
        PwmOp: PwmPeripheral,
        PinA: esp_hal::gpio::OutputPin,
        PinB: esp_hal::gpio::OutputPin,
        PinC: esp_hal::gpio::OutputPin,
        // T: TimerGroupInstance,
    >
    Esp3PWM<
        'd,
        PwmOp,
        PinA,
        PinB,
        PinC,
        EncoderPosn,
        // TimerGroup<'d, T, Blocking>,
        // DefaultMotionCtrl<Timer0<T>, EncoderPosn>,
    >
{
    /// Takes in the peripherals needed in order run a motor:
    ///  - pin_a/b/c: These pins will be attached to the mcpwm peripheral
    ///  - enc_a/b: these pins will be attached to the PCNT peripheral.
    ///  - esp32 has two timer groups and two mcpwm peripherals. you can pass in one of either
    ///  - the timer group will use one of its timers for the mcpwm operators.
    ///
    pub fn new(
        clocks: &Clocks<'_>,
        // timg_choice: impl Peripheral<P = T> + 'd,
        mcpwm_choice: impl Peripheral<P = PwmOp> + 'd,
        pcnt_periph: impl Peripheral<P = peripherals::PCNT> + 'd,
        motor_pins: (
            impl Peripheral<P = PinA> + 'd,
            impl Peripheral<P = PinB> + 'd,
            impl Peripheral<P = PinC> + 'd,
        ),
        encoder_pins: (
            impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
            impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
        ),
    ) -> Self {
        // set up the peripherals for our specific usecase
        // let timg0 = TimerGroup::new(timg_choice, clocks, None);
        // let time_src = Timer0::init(timg0.timer0);
        let clock_cfg = PeripheralClockConfig::with_frequency(clocks, 40.MHz()).unwrap();

        // Boiler-plate configuration...
        let pin_config =
            || PwmPinConfig::<true>::new(PwmActions::empty(), PwmUpdateMethod::empty());
        let mut mcpwm_periph = MCPWM::new(mcpwm_choice, clock_cfg);
        mcpwm_periph.operator0.set_timer(&mcpwm_periph.timer0);
        mcpwm_periph.operator1.set_timer(&mcpwm_periph.timer0);
        mcpwm_periph.operator2.set_timer(&mcpwm_periph.timer0);

        // Give each operator a pin.
        let pin_a: PwmPin<'d, PinA, _, 0, true> = mcpwm_periph
            .operator0
            .with_pin_a(motor_pins.0, pin_config());
        let pin_b: PwmPin<'d, PinB, _, 1, true> = mcpwm_periph
            .operator1
            .with_pin_a(motor_pins.1, pin_config());
        let pin_c: PwmPin<'d, PinC, _, 2, true> = mcpwm_periph
            .operator2
            .with_pin_a(motor_pins.2, pin_config());
        // Put that into a Triplet. Because the pins meets the impl-constraints for
        // `MotorPins` trait, it is now the pin-control driver/object
        let mut motor_triplet = Triplet {
            member_a: pin_a,
            member_b: pin_b,
            member_c: pin_c,
        };
        MotorPins::set_zero(&mut motor_triplet);

        // We want middle-out. I don't know about the pre-scaler or freq settings, but this is my
        // best initial guess
        let pw_timer_cfg = clock_cfg
            .timer_clock_with_frequency(99, PwmWorkingMode::UpDown, 20.kHz())
            .unwrap();

        // Let's get the party started.
        mcpwm_periph.timer0.start(pw_timer_cfg);

        let pcnt = PCNT::new(pcnt_periph, None);
        let mut pcnt_unit0 = pcnt.get_unit(unit::Number::Unit0);
        pcnt_unit0
            .configure(unit::Config {
                low_limit: -100,
                high_limit: 100,
                ..Default::default()
            })
            .unwrap();

        let mut pcnt_chann0 = pcnt_unit0.get_channel(channel::Number::Channel0);
        pcnt_chann0.configure(
            channel::PcntSource::from_pin(encoder_pins.0),
            channel::PcntSource::from_pin(encoder_pins.1),
            channel::Config {
                lctrl_mode: channel::CtrlMode::Reverse,
                hctrl_mode: channel::CtrlMode::Keep,
                pos_edge: channel::EdgeMode::Decrement,
                neg_edge: channel::EdgeMode::Increment,
                invert_ctrl: false,
                invert_sig: false,
            },
        );

        Self {
            motor_triplet,
            pulse_counter: PhantomData,
        }
    }
}

impl<'d, PwmOp, A, B, C, Pos> MotorPins for Esp3PWM<'d, PwmOp, A, B, C, Pos>
where
    PwmPin<'d, A, PwmOp, 0, true>: SetDutyCycle,
    PwmPin<'d, B, PwmOp, 1, true>: SetDutyCycle,
    PwmPin<'d, C, PwmOp, 2, true>: SetDutyCycle,
{
    fn set_pwms(
        &mut self,
        dc_a: sfoc_rs::common::helpers::DutyCycle,
        dc_b: sfoc_rs::common::helpers::DutyCycle,
        dc_c: sfoc_rs::common::helpers::DutyCycle,
    ) {
        self.motor_triplet.set_pwms(dc_a, dc_b, dc_c)
    }

    fn set_zero(&mut self) {
        self.motor_triplet.set_zero()
    }
}

impl<'d, PwmOp, A, B, C, Pos> FOController for Esp3PWM<'d, PwmOp, A, B, C, Pos>
where
    PwmPin<'d, A, PwmOp, 0, true>: SetDutyCycle,
    PwmPin<'d, B, PwmOp, 1, true>: SetDutyCycle,
    PwmPin<'d, C, PwmOp, 2, true>: SetDutyCycle,
{
}
