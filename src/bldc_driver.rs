pub enum PhaseState {
    Off,
    On,
    Hi,
    Lo,
}
pub trait BLDCDriver: Sized {
    fn init() -> Result<Self, ()>;
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_pwm(&mut self, v_a: f32,v_b: f32,v_c: f32);
    fn set_phasestate(&mut self, state: PhaseState);
}
