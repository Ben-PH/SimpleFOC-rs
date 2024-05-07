pub enum Command {
    CurrentDPID,
    CurrentQPID,
    VelocityPID,
    PositionPID,
    Status,
    Limits,
    MotionType,
    ForceType,
    SensorOffsets,
    Monitor,
    Resist,
    Inductants,
    PVS,
    PWMModulation,
}

pub enum PIDSubComm {
    P,
    I,
    D,
    Ramp,
    Limit,
    TimeConst,
}

pub enum LimitsSubComm {
    Current,
    Voltage,
    Velocity,
}

// TODO
// enum SensorSubComm {}

pub enum MonitoringSubComm {
    DownSample,
    Clear,
    Get,
    Set,
}

pub enum PWMModulationSubComm {
    Type,
    CenterFlag,
}
