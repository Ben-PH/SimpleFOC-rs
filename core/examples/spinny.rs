/// The sfoc equivilent of a micro-controller "blinky"
///
/// Running something modeled on this example, except with actual hardware, a motor that is at a
/// fixed rpm, and outputting the pwm duty telemetry, you "should" see the same behavior.
///
/// Coming Soon™ will be actual code that you can run on $HARDWARE. It will use constant voltage,
/// chosen such that a matching back-EMF is generated at around 30RPM.
use std::time::SystemTime;

use discrete_count::re_exports::fixed::types::I16F16;
use foc::park_clarke;
use sfoc_rs::{
    self, bldc_driver::MotorPins, common::helpers::DutyCycle, foc_control::FOController,
    pos_sensor::PosSensor,
};

fn main() {
    let driver = SomePlatformSpecificImpl;
    foc_main(driver)
}

// This can be made portable. We'll be using the `FOController` and `PosSensor` traits. In the near
// future, there will also be examples on how to write a portable implementation.
fn foc_main(mut driver: SomePlatformSpecificImpl) -> ! {
    loop {
        // The "FO" part of the FOC. Here, we derive the position of the motor. We will use this to
        // determine which electro-magnetic-field angle we wish to set.
        //
        // In this example, we've hacked the sensor such that this method is hard-coded to rotate at 30rpm
        let angle = driver.get_position_um();
        let (sin, cos) = cordic::sin_cos(angle);

        // we are setting 10% in the quadrature (torque) and 0% direct-axis
        let qd = park_clarke::MovingReferenceFrame {
            d: I16F16::ZERO,
            q: I16F16::from_num(0.1),
        };

        // Orient our q/d values to the phase-angle we desire
        let inv_parke = foc::park_clarke::inverse_park(cos, sin, qd);
        // and use the ideal, though computationally heavy, scale-vector calculation for desired
        // pwm duty-cycles, i.e. the percentage-of-maximum that each motor-phase will be set to
        let [dc_a, dc_b, dc_c] = foc::pwm::svpwm(inv_parke);

        //... and set them.
        driver.set_pwms(DutyCycle(dc_a), DutyCycle(dc_b), DutyCycle(dc_c));

        // Let's artificially slow things down for the purpose of this example
        std::thread::sleep(std::time::Duration::from_millis(117));
    }
}

/// This would be replaced by a platform specific struct in the code that provides MCU-specific
/// support
struct SomePlatformSpecificImpl;

impl FOController for SomePlatformSpecificImpl {
    fn set_psu_millivolt(&self, mv: u16) {
        unimplemented!()
    }
}

impl MotorPins for SomePlatformSpecificImpl {
    fn set_pwms(
        &mut self,
        dc_a: sfoc_rs::common::helpers::DutyCycle,
        dc_b: sfoc_rs::common::helpers::DutyCycle,
        dc_c: sfoc_rs::common::helpers::DutyCycle,
    ) {
        println!(
            "DCs set: {:>+6.4}|{:>+6.4}|{:>+6.4}",
            dc_a.0, dc_b.0, dc_c.0
        );
    }
}

impl PosSensor for SomePlatformSpecificImpl {
    type Output = I16F16;

    /// For an actual motor, this would read a position encoder, and give you a position. This
    /// might be milliradians, for example.
    /// In this case, we use std-time to get millis that rolls back to zero every 2 seconds
    fn get_position_um(&self) -> Self::Output {
        let now = std::time::SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let millis = now % 2000;
        let frac = I16F16::from_num(millis) / 2000;
        I16F16::TAU * frac
    }
}
