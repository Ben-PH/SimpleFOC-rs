use core::marker::PhantomData;

use embedded_hal::pwm::SetDutyCycle;
use esp_backtrace as _;
use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    mcpwm::{
        operator::{PwmActions, PwmPin, PwmPinConfig, PwmUpdateMethod},
        timer::PwmWorkingMode,
        McPwm, PeripheralClockConfig, PwmPeripheral,
    },
    pcnt::unit::Unit,
    peripheral::Peripheral,
    peripherals,
    prelude::*,
};

use sfoc_rs_core::{
    bldc_driver::MotorPins,
    common::helpers::{Couplet, DutyCycle, Triplet},
    foc_control::FOController,
};

// use crate::posn_encoder::EncoderPosn;

pub struct Esp3PWM<'d, PwmOp> {
    motor_triplet:
        Triplet<PwmPin<'d, PwmOp, 0, true>, PwmPin<'d, PwmOp, 1, true>, PwmPin<'d, PwmOp, 2, true>>,
}

impl<'d, PwmOp: PwmPeripheral> Esp3PWM<'d, PwmOp> {
    /// Takes in the peripherals needed in order run a motor:
    ///  - pin_a/b/c: These pins will be attached to the mcpwm peripheral
    ///  - enc_a/b: these pins will be attached to the Pcnt peripheral.
    ///  - esp32 has two timer groups and two mcpwm peripherals. you can pass in one of either
    ///  - the timer group will use one of its timers for the mcpwm operators.
    ///
    pub fn init(
        // timg_choice: impl Peripheral<P = T> + 'd,
        mcpwm_choice: impl Peripheral<P = PwmOp> + 'd,
        pcnt_periph: impl Peripheral<P = peripherals::PCNT> + 'd,
        motor_pins: (
            impl Peripheral<P = impl PeripheralOutput> + 'd,
            impl Peripheral<P = impl PeripheralOutput> + 'd,
            impl Peripheral<P = impl PeripheralOutput> + 'd,
        ),
        // encoder_pins: (
        //     impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
        //     impl Peripheral<P = impl esp_hal::gpio::InputPin> + 'd,
        // ),
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

        // let pcnt = Pcnt::new(pcnt_periph);
        // let pcnt_unit0 = pcnt.unit0;
        // let _ = pcnt_unit0.set_low_limit(Some(-100));
        // let _ = pcnt_unit0.set_high_limit(Some(100));
        //
        // let pcnt_chann0 = &pcnt_unit0.channel0;
        // pcnt_chann0.set_ctrl_signal(encoder_pins.0);
        // pcnt_chann0.set_edge_signal(encoder_pins.1);
        // pcnt_chann0.set_input_mode(channel::EdgeMode::Decrement, channel::EdgeMode::Increment);

        Self { motor_triplet }
    }
}

impl<'d, PwmOp> MotorPins for Esp3PWM<'d, PwmOp>
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

impl<'d, PwmOp> FOController for Esp3PWM<'d, PwmOp>
where
    PwmPin<'d, PwmOp, 0, true>: SetDutyCycle,
    PwmPin<'d, PwmOp, 1, true>: SetDutyCycle,
    PwmPin<'d, PwmOp, 2, true>: SetDutyCycle,
{
    fn set_psu_millivolt(&self, _mv: u16) {
        todo!()
    }
}

pub struct EspPcnt<'a, A, B, const UNIT_NUM: usize, Resolution, Measure> {
    ab_pins: Couplet<A, B>,
    unit: Unit<'a, UNIT_NUM>,
    _resolution: PhantomData<Resolution>,
    _measure: PhantomData<Measure>,
}

pub struct UnitReader<'a, const UNIT_NUM: usize>(pub Unit<'a, UNIT_NUM>);
impl<'a, const UNIT_NUM: usize> discrete_count::CountReader for UnitReader<'a, UNIT_NUM> {
    type ReadErr = ();

    type RawData = i16;

    fn read(&self) -> Result<Self::RawData, Self::ReadErr> {
        Ok(self.0.counter.get())
    }
}

impl<'a, A, B, const UNIT_NUM: usize, Resolution, Measure> discrete_count::Counter
    for EspPcnt<'a, A, B, UNIT_NUM, Resolution, Measure>
{
    type Reader = UnitReader<'a, UNIT_NUM>;

    type Resolution = Resolution;

    type Measure = Measure;

    fn update_count_state(
        &mut self,
        count: discrete_count::CountRaw<Self>,
    ) -> Result<(), <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn read_count_state(&self) -> &discrete_count::CountRaw<Self> {
        todo!()
    }

    fn try_update_count(
        &mut self,
    ) -> Result<(), <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn try_read_measure(
        &self,
    ) -> Result<Self::Measure, <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn measure_count_state(&self) -> Self::Measure {
        todo!()
    }

    fn try_update_and_measure(
        &mut self,
        count: &discrete_count::CountRaw<Self>,
    ) -> Result<Self::Measure, <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn measure_count(count: &discrete_count::CountRaw<Self>) -> Self::Measure {
        todo!()
    }
}
