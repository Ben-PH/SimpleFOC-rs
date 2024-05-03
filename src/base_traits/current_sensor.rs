use crate::common::helpers::{ABCurrent, DQCurrent, PhaseCurrent};

pub trait CurrentSensor: Sized {
    fn init() -> Result<Self, ()>;
    // fn link_driver?(&mut self, driver: BLDCDriver);
    // fn linked_driver?(&self) -> Option<&BLDCDriver>;
    fn driver_allign(&self, allign_voltage: f32) -> Result<DriverStatus, ()>;
    fn getPhaseCurrents(&self) -> PhaseCurrent;
    fn get_dc_current(&self, phase_theta: Option<f32>) -> f32;
    fn get_foc_currents(phase_theta: f32) -> DQCurrent;
    fn get_AB_currents(current: PhaseCurrent) -> ABCurrent;
    fn get_DQ_currents(current: ABCurrent, phase_theta: f32) -> DQCurrent;
    fn enable(&mut self);
    fn disable(&mut self);
}
