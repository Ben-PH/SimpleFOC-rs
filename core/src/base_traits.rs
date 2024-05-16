use typenum::{IsGreater, True, Unsigned, U0};

pub mod bldc_driver;
pub mod current_sensor;
pub mod foc_control;
pub mod pos_sensor;

// Allows you to attach an upper-bound of the voltage coming from your power supply
pub trait PowerSupplyVoltage {
    type DeciVolts: Unsigned + IsGreater<U0, Output = True>;
}
