pub const SQRT3: f32 = 1.732_050_807_57;
pub const SQRT2: f32 = 1.414_213_562_37;
pub const PI: f32 = 3.141_592_653_59;
pub const PI_2: f32 = 1.570_796_326_79;
pub const PI_3: f32 = 1.047_197_551_2;
pub const _2PI: f32 = 6.283_185_307_18;
pub const _3PI_2: f32 = 4.712_388_980_38;
pub const PI_6: f32 = 0.523_598_775_59;
pub const RPM_TO_RADS: f32 = 0.104_719_755_12;
pub const _2_SQRT3: f32 = 1.154_700_538_38;
pub const _1_SQRT3: f32 = 0.577_350_269_19;
pub const SQRT3_2: f32 = 0.866_025_403_78;
pub const _120_D2R: f32 = 2.094_395_102_39;

struct Current(f32);
struct Voltage(f32);

pub struct DQCurrent {
    pub d: Current,
    pub q: Current,
}
pub struct PhaseVoltages {
    pub a: Voltage,
    pub b: Voltage,
    pub c: Voltage,
}
pub struct PhaseCurrent {
    pub a: Current,
    pub b: Current,
    pub c: Current,
}
// dq voltage structs
pub struct DQVoltage {
    pub d: Voltage,
    pub q: Voltage,
}
// alpha beta current structure
pub struct ABCurrent {
    pub alpha: Current,
    pub beta: Current,
}
