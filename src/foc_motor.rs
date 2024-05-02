use crate::pos_sensor::PosSensor;

pub enum MotionCtrlType {
    Force,
    Velocity,
    Position,
    VelocityOpenLoop,
    PositionOpenLoop,
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
pub struct FOCMotor<PS: PosSensor, CS> {
    pos_sensor: PS,
    current_sensor: CS,
    velocity_limit: u32,
    voltage_limit: u32,
    current_limit: u32,
    // foc_modulation: FOCModulationType,
    // target
}

impl<PS: PosSensor, CS> FOCMotor<PS, CS> {
    pub fn pos(&self) -> u32 {
        todo!()
    }
    pub fn velocity(&self) -> i32 {
        todo!()
    }
}

