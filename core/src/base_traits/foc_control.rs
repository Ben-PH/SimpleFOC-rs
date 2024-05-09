use embedded_hal::{digital::InputPin, pwm::SetDutyCycle};
use embedded_time::duration::Microseconds;

use crate::common::types::VelocityPID;

/// The describes the position of an inductor in the pitch of the permenant magnetic field, in
/// units of tau.
/// A linear motor with a 20mm pitch, 10mm from reference zero, the value would be 0.5
/// A rotary motor with a pitch of 36 degrees, and 9 inductors would have 12 degrees in
/// the physical rotation of the motor for each phase-angle rotation.
pub struct PhaseAngle(pub f32);
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
pub trait FOController<EncA, EncB, PhA, PhB, PhC>: Sized
where
    EncA: InputPin,
    EncB: InputPin,
    PhA: SetDutyCycle,
    PhB: SetDutyCycle,
    PhC: SetDutyCycle,
{
    fn init_fo_control(
        enc_pins: (EncA, EncB),
        bldc3_pins: (PhA, PhB, PhC),
        velocity_pid: VelocityPID,
    ) -> Result<Self, ()>;
    fn enable(&mut self);
    fn disable(&mut self);
    // fn link_sensor/current_sensor(....
    fn init_foc_algo(&mut self) -> u32; // why the u32?
    fn foc_loop(&mut self) -> !;
    fn move_command(motion: MotionCtrl);
    fn set_phase_voltage(u_q: f32, u_d: f32, phase_angle: PhaseAngle);
}

pub struct UnimpFOController;

#[allow(unused_variables)]
impl<EncA, EncB, PhA, PhB, PhC> FOController<EncA, EncB, PhA, PhB, PhC> for UnimpFOController
where
    EncA: InputPin,
    EncB: InputPin,
    PhA: SetDutyCycle,
    PhB: SetDutyCycle,
    PhC: SetDutyCycle,
{
    fn init_fo_control(
        enc_pins: (EncA, EncB),
        bldc3_pins: (PhA, PhB, PhC),
        velocity_pid: VelocityPID,
    ) -> Result<Self, ()> {
        // init the encoder
        // enable interupts
        // init the driver (including voltage settings)
        // letup lp-filter
        // init me
        // init my foc algo
        // ready
        todo!()
    }
    fn enable(&mut self) {
        todo!()
    }
    fn disable(&mut self) {
        todo!()
    }
    fn init_foc_algo(&mut self) -> u32 {
        todo!()
    }
    fn foc_loop(&mut self) -> ! {
        todo!()
    }
    fn move_command(motion: MotionCtrl) {
        todo!()
    }
    fn set_phase_voltage(u_q: f32, u_d: f32, phase_angle: PhaseAngle) {
        todo!()
    }
}
