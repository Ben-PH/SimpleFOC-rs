use crate::common::types::{VelocityPID, QCurrentPID, DCurrentPID};



#[allow(dead_code)]
// D: implements the bldc driver
pub struct BLDCMotor<D> {
    pole_pairs: u8,
    phase_resistance: f32,
    /// phases/second/volt
    ///
    /// A more generalised form of the kV metric. A 1kV motor with 12 permenant magnets means 6
    /// pairs, which means 6 pitches per revolution. 1kV implies that 6000 pitches traversed in 1
    /// minuite induces a 1V back-emf.
    ///
    /// This metric allows one to denote rotary kV (take the pair-count of a rotary motor, and infer the
    /// rpm needed to traverse `psv` pitches in a minute), as well as linear motor characteristics
    psv: f32,
    phase_inductance: f32,
    velocity_pid: VelocityPID,
    current_pid_q: QCurrentPID,
    current_pid_d: DCurrentPID,
    driver: D,
}

impl<D> BLDCMotor<D> {
    pub fn align_position_sensor() {
        todo!()
    }
    pub fn align_current_sense() {
        todo!()
    }
    pub fn find_zero_reference() {
        todo!()
    }
}

