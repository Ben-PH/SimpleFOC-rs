use crate::base_traits::foc_motor::{FOCMotor, MotionCtrl, PhaseAngle};

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
}

impl BLDCMotor {
    fn align_position_sensor() {
        todo!()
    }
    fn align_current_sense() {
        todo!()
    }
    fn find_zero_reference() {
        todo!()
    }
}

impl FOCMotor for BLDCMotor {
    fn init() -> Result<Self, ()> {
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
