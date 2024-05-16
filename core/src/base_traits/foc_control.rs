use embedded_time::duration::Microseconds;
use fixed::types::I16F16;
use foc::park_clarke::MovingReferenceFrame;

use crate::common::helpers::DutyCycle;

use super::bldc_driver::MotorHiPins;

/// The describes the position of an inductor in the pitch of the permenant magnetic field, in
/// units of tau.
/// A linear motor with a 20mm pitch, 10mm from reference zero, the value would be 0.5
/// A rotary motor with a pitch of 36 degrees, and 9 inductors would have 12 degrees in
/// the physical rotation of the motor for each phase-angle rotation.
pub struct PhaseAngle(pub I16F16);
pub struct Newtons(pub f32);

pub enum MotionCtrl {
    Force(Newtons),
    Velocity(PhaseAngle, Microseconds<u32>),
    Position(PhaseAngle),
    VelocityOpenLoop(PhaseAngle, Microseconds<u32>),
    PositionOpenLoop(PhaseAngle),
}
pub enum ForceControlType {
    Voltage,
    DCCurrent,
    FOCCurrent,
}
pub enum FOCMotorStatus {
    Uninit,
    Initting,
    Uncalibrated,
    Callibrating,
    Ready,
    RecoverableError,
    CalbrationFail,
    InitFail,
}
#[derive(Default)]
pub enum FOCModulationType {
    #[default]
    SinePWM,
    SpaceVectorPWM,
    Trapezoid120,
    Trapezoid150,
}

// temporarily hacked to be for a 3pwm bldc motor
pub trait FOController: Sized + MotorHiPins {
    // fn enable(&mut self);
    // fn disable(&mut self);
    // fn link_sensor/current_sensor(....
    // todo: fn init_foc_algo(&mut self) -> u32; // why the u32?
    // todo: fn foc_loop(&mut self) -> !;
    // todo: fn move_command(motion: MotionCtrl);
    fn set_phase_voltage(&mut self, voltages_q_d: MovingReferenceFrame, phase_angle: PhaseAngle) {
        let (sin_angle, cos_angle) = cordic::sin_cos(phase_angle.0);
        let orth_v = foc::park_clarke::inverse_park(cos_angle, sin_angle, voltages_q_d);
        let [phase_a, phase_b, phase_c] = foc::pwm::spwm(orth_v);
        self.set_pwms(DutyCycle(phase_a), DutyCycle(phase_b), DutyCycle(phase_c))
    }
}
