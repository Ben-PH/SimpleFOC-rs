pub const SQRT3: f32 = 1.73205080757;
pub const SQRT2: f32 = 1.41421356237;
pub const PI: f32 = 3.14159265359;
pub const PI_2: f32 = 1.57079632679;
pub const PI_3: f32 = 1.0471975512;
pub const _2PI: f32 = 6.28318530718;
pub const _3PI_2: f32 = 4.71238898038;
pub const PI_6: f32 = 0.52359877559;
pub const RPM_TO_RADS: f32 = 0.10471975512;
pub const _2_SQRT3: f32 = 1.15470053838;
pub const _1_SQRT3: f32 = 0.57735026919;
pub const SQRT3_2: f32 = 0.86602540378;
pub const _120_D2R: f32 = 2.09439510239;

pub struct DQCurrent {
    pub d: f32,
    pub q: f32,
}
pub struct PhaseCurrent {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}
// dq voltage structs
pub struct DQVoltage {
    pub d: f32,
    pub q: f32,
}
// alpha beta current structure
pub struct ABCurrent {
    pub alpha: f32,
    pub beta: f32,
}
