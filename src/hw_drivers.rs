// pub trait HWApi<U: typenum::Unsigned>
// {
//
//     // TODO: make `pins` a heterogenous list of pins, all implementing PwmControl, of length N.
//     // should be doable with `frunk::HList`
//     fn config_pwm(freq: embedded_time::rate::Hertz<u32>) -> Self;
//     fn write_duty_cycle<DC: j(mut self, duties: HC) -> Self;
// }

// impl HWApi<typenum::U2> for  Esp32MCPWM
// {
//
//
//     fn config_pwm(freq: embedded_time::rate::Hertz<u32>, pins: ()) -> Self {
//         todo!();
//         // cpp impl finds an ampty pwm-motor slot, and puts pin a in it, then puts that into the
//         // local m_slot variable
//
//         // depending on which slot number used, it marks certain stepper/bldc slots as taken
//
//         // run `mcpwm_gpio_init`, passing in that slots mcpwm unit, its mcpwm_a/b, and pinA/B
//
//         // then runs configuration of the timer frequency
//
//         // returns the esp32MCPWMDriverParams struct
//     }
//
//     fn write_duty_cycle(&mut self, duties: ()) {
//         todo!();
//         // Okay, so the esp32 has a nice way of organising the different structs. We have:
//         //
//         // mcpwm::MCPWM struct with the three timers, and the three operators
//         //  - this has the PwmPeripheral trait attached to it.
//         //  - its construction requireres a PeripheralClockConfig struct as well
//         //
//         //  if we pull out the operator0 field, we get the methods
//         //   - `set_timer`, to select the timer to serve as timing reference
//         //   - getters for the two pins
//         //   - `with_linked_pins` that takes the two pins, an `a` and `b` cgonfig, and a `dt`
//         //   config and gives back `LinkedPins`
//         //     - "useful for complementary/mirrored signals with/without configured deadtime"
//         //
//         // `PwmPin` to get period(
//         // let mut pin_a = slf.operator0.with_pin_a(...);
//         // pin_a.set_duty(duties.0);
//         // let mut pin_b = slf.operator0.with_pin_a(...);
//         // pin_b.set_duty(duties.1);
//     }
// }
