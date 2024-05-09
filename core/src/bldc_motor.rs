use crate::base_traits::foc_motor::{FOCMotor, MotionCtrl, PhaseAngle, VelocityPID, DCurrentPID, QCurrentPID};

#[allow(dead_code)]
pub struct BLDCMotor {
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
}

impl BLDCMotor {
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

impl FOCMotor for BLDCMotor {
    fn init_foc_motor() -> Result<Self, ()> {
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

    fn move_command(_motion: MotionCtrl) {
        todo!()
    }

    fn set_phase_voltage(_u_q: f32, _u_d: f32, _phase_angle: PhaseAngle) {
        todo!()
    }
}
