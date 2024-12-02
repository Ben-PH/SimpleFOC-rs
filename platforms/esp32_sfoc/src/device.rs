use core::marker::PhantomData;
use embedded_hal::pwm::SetDutyCycle;
use esp_backtrace as _;
use esp_hal::{
    clock::Clocks,
    gpio::interconnect::PeripheralOutput,
    mcpwm::{
        operator::{PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        timer::PwmWorkingMode,
        McPwm, PeripheralClockConfig, PwmPeripheral,
    },
    pcnt::{channel, unit, Pcnt},
    peripheral::Peripheral,
    peripherals,
    prelude::*,
};

use sfoc_rs::{
    bldc_driver::MotorPins,
    common::helpers::{DutyCycle, Triplet},
    foc_control::FOController,
};

use crate::posn_encoder::EncoderPosn;

pub struct Esp3PWM<'d, PwmOp, Pos> {
    // mcpwm_periph: McPwm<'d, PwmOp>,
    motor_triplet:
        Triplet<PwmPin<'d, PwmOp, 0, true>, PwmPin<'d, PwmOp, 1, true>, PwmPin<'d, PwmOp, 2, true>>,
    pulse_counter: PhantomData<Pos>,
    // time_src: PhantomData<Tim>,
    // _motion_ctrl: M,
}

impl<
        'd,
        PwmOp: PwmPeripheral,
        const UNIT_NUM: usize,
        // T: TimerGroupInstance,
    >
    Esp3PWM<
        'd,
        PwmOp,
        EncoderPosn<'_, UNIT_NUM>,
        // TimerGroup<'d, T, Blocking>,
        // DefaultMotionCtrl<Timer0<T>, EncoderPosn>,
    >
{
    /// Takes in the peripherals needed in order run a motor:
    ///  - pin_a/b/c: These pins will be attached to the mcpwm peripheral
    ///  - enc_a/b: these pins will be attached to the Pcnt peripheral.
    ///  - esp32 has two timer groups and two mcpwm peripherals. you can pass in one of either
    ///  - the timer group will use one of its timers for the mcpwm operators.
    ///
    pub fn new(
        // timg_choice: impl Peripheral<P = T> + 'd,
        mcpwm_choice: impl Peripheral<P = PwmOp> + 'd,
        pcnt_periph: impl Peripheral<P = peripherals::PCNT> + 'd,
        motor_pins: (
            impl Peripheral<P = impl PeripheralOutput> + 'd,
            impl Peripheral<P = impl PeripheralOutput> + 'd,
            impl Peripheral<P = impl PeripheralOutput> + 'd,
        ),
        encoder_pins: (
            impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
            impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
        ),
    ) -> Self {
        // set up the peripherals for our specific usecase
        // let timg0 = TimerGroup::new(timg_choice, clocks, None);
        // let time_src = Timer0::init(timg0.timer0);
        let clock_cfg = PeripheralClockConfig::with_frequency(40.MHz()).unwrap();

        // Boiler-plate configuration...
        let pin_config =
            || PwmPinConfig::<true>::new(PwmActions::empty(), PwmUpdateMethod::empty());
        let mut mcpwm_periph = McPwm::new(mcpwm_choice, clock_cfg);
        mcpwm_periph.operator0.set_timer(&mcpwm_periph.timer0);
        mcpwm_periph.operator1.set_timer(&mcpwm_periph.timer0);
        mcpwm_periph.operator2.set_timer(&mcpwm_periph.timer0);

        // Give each operator a pin.
        let pin_a: PwmPin<'d, _, 0, true> = mcpwm_periph
            .operator0
            .with_pin_a(motor_pins.0, pin_config());
        let pin_b: PwmPin<'d, _, 1, true> = mcpwm_periph
            .operator1
            .with_pin_a(motor_pins.1, pin_config());
        let pin_c: PwmPin<'d, _, 2, true> = mcpwm_periph
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

        let pcnt = Pcnt::new(pcnt_periph);
        let mut pcnt_unit0 = pcnt.unit0;
        pcnt_unit0.set_low_limit(Some(-100));
        pcnt_unit0.set_high_limit(Some(100));

        let mut pcnt_chann0 = &pcnt_unit0.channel0;
        pcnt_chann0.set_ctrl_signal(encoder_pins.0);
        pcnt_chann0.set_edge_signal(encoder_pins.1);
        pcnt_chann0.set_input_mode(channel::EdgeMode::Decrement, channel::EdgeMode::Increment);

        Self {
            motor_triplet,
            pulse_counter: PhantomData,
        }
    }
}

impl<'d, PwmOp, Pos> MotorPins for Esp3PWM<'d, PwmOp, Pos>
where
    PwmPin<'d, PwmOp, 0, true>: SetDutyCycle,
    PwmPin<'d, PwmOp, 1, true>: SetDutyCycle,
    PwmPin<'d, PwmOp, 2, true>: SetDutyCycle,
{
    fn set_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle) {
        self.motor_triplet.set_pwms(dc_a, dc_b, dc_c)
    }

    fn set_zero(&mut self) {
        self.motor_triplet.set_zero()
    }
}

impl<'d, PwmOp, Pos> FOController for Esp3PWM<'d, PwmOp, Pos>
where
    PwmPin<'d, PwmOp, 0, true>: SetDutyCycle,
    PwmPin<'d, PwmOp, 1, true>: SetDutyCycle,
    PwmPin<'d, PwmOp, 2, true>: SetDutyCycle,
{
    fn set_psu_millivolt(&self, mv: u16) {
        todo!()
    }
}
