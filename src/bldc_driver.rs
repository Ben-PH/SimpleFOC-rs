use crate::foc_utils::PhaseVoltages;

/// Describes what a given phase/coil/inductor is doing
#[derive(Default, Copy, Clone)]
pub enum PhaseState {
    #[default]
    Off,
    On,
    Hi,
    Lo,
}

pub trait BLDCDriver: Sized {
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_pwm(&mut self, voltages: PhaseVoltages);
    fn set_phasestate(&mut self, state: [PhaseState; 3]);
}
