use pid::Pid;

use crate::common::helpers::{DQVoltage, DQCurrent};


pub struct RawRewriteStateVars {
    pub set_pont: f32,
    pub feed_forward_velocity: f32,
    pub shaft_angle: f32,
    pub electrical_angle:f32,
    pub shaft_velocity: f32,
    pub target_shaft_angle: f32,
    pub latest_dq_voltage: DQVoltage,
    pub latest_dq_current: DQCurrent,
    pub estimated_back_emf: f32, 
    pub park_clarke_dq: (f32, f32),
}


pub struct RawRewriteConfigParams {
    pub voltage_sensor_align: f32,
    pub velocity_index_search: f32,
}

pub struct RawRewritePhysParams {
    pub phase_resistance: f32,
    pub pole_pars: u8,
    pub kv_rating: f32,
    pub phase_inductance: f32,
}
pub struct RawRewriteLimits {
    pub millivolt_limit: u16,
    pub milliamp_limit: u16,
    pub velocity_limit: u16,
}

pub struct LowPassFilter {
    pub constant: f32,
    pub previous: (),
}
pub struct Controlers {
    pub q_current: Pid<f32>,
    pub lpf_q_current: LowPassFilter,
    pub d_current: Pid<f32>,
    pub lpf_d_current: LowPassFilter,

    pub velocity: Pid<f32>,
    pub lpf_q_velocity: LowPassFilter,
    pub angle: Pid<f32>,
    pub lpf_q_angle: LowPassFilter,

    pub motion_downsample: u32,
    pub motion_count: u32,
}

pub struct RawRewriteSensorVars {
    pub zero_offset: f32,
    pub abs_zero_angle: Option<f32>,
    pub is_forward: Option<bool>,
    pub pp_check_res: bool,
}

