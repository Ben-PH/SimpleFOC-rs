use micromath::F32;

use crate::common::helpers::{ABCurrent, Current, DQCurrent, PhaseCurrent};

pub trait CurrentSensor: Sized {
    fn init_current_sensor() -> Result<Self, ()>;
    // fn link_driver?(&mut self, driver: BLDCDriver);
    // fn linked_driver?(&self) -> Option<&BLDCDriver>;
    fn driver_allign(&self, allign_voltage: f32) -> Result<(), ()>;
    fn get_phase_currents(&self) -> PhaseCurrent;
    fn get_dc_current(&self, phase_theta: Option<f32>) -> f32 {
        let phase_current = self.get_phase_currents();
        let ab_current = Self::get_ab_currents(phase_current);
        let sign: f32 = if let Some(theta) = phase_theta {
            let theta = F32(theta);
            let (F32(st), F32(ct)) = theta.sin_cos();
            let sign = ab_current.beta.0 * ct - ab_current.alpha.0 * st;
            if sign > 0.0 {
                1.0
            } else {
                -1.0
            }
        } else {
            1.0
        };
        sign * F32(ab_current.alpha.0 * ab_current.alpha.0 + ab_current.beta.0 * ab_current.beta.0)
            .sqrt()
            .0
    }
    fn get_foc_currents(&self, angle_el: f32) -> DQCurrent {
        let current = self.get_phase_currents();
        let ab_current = Self::get_ab_currents(current);
        Self::get_dq_currents(ab_current, angle_el)
    }
    fn get_ab_currents(current: PhaseCurrent) -> ABCurrent;
    fn get_dq_currents(current: ABCurrent, phase_theta: f32) -> DQCurrent {
        let angle_el = F32(phase_theta);
        let (F32(st), F32(ct)) = angle_el.sin_cos();
        let d = current.alpha.0 * ct + current.beta.0 * st;
        let q = current.beta.0 * ct + current.alpha.0 * st;
        DQCurrent {
            d: Current(d),
            q: Current(q),
        }
    }
    /// usually this is just implemented as a no-op
    fn enable(&mut self);
    /// usually this is just implemented as a no-op
    fn disable(&mut self);
}
