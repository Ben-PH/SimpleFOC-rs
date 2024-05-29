use fixed::{types::I16F16, consts::FRAC_PI_3};
use foc::park_clarke;
/// The sfoc equivilent of "blinky"
use sfoc_rs::{self, base_traits::{foc_control::FOController, bldc_driver::MotorPins, pos_sensor::PosSensor}, common::helpers::DutyCycle};

fn main() {
    let driver = SomePlatformSpecificImpl;
    foc_main(driver)
}

// This can be made portable. We'll be using the `FOController` and `PosSensor` traits. In the near
// future, there will also be examples on how to write a portable implementation.
fn foc_main(mut driver: SomePlatformSpecificImpl) -> ! 
{

    loop {
        let angle = driver.get_position_um();
        let (sin, cos) = cordic::sin_cos(angle);
        let qd = park_clarke::MovingReferenceFrame{
            d: I16F16::ZERO,
            q: I16F16::from_num(0.1)
        };
        let inv_parke = foc::park_clarke::inverse_park(cos, sin, qd); 
        let [dc_a, dc_b, dc_c] = foc::pwm::svpwm(inv_parke);
        driver.set_pwms(DutyCycle(dc_a), DutyCycle(dc_b), DutyCycle(dc_c));
    }
}

/// This would be replaced by a platform specific struct in the code that provides MCU-specific
/// support
struct SomePlatformSpecificImpl;

impl FOController for SomePlatformSpecificImpl { }

impl MotorPins for SomePlatformSpecificImpl{
    fn set_pwms(&mut self, dc_a: sfoc_rs::common::helpers::DutyCycle, dc_b: sfoc_rs::common::helpers::DutyCycle, dc_c: sfoc_rs::common::helpers::DutyCycle) {
        println!("DCs set: {}-{}-{}", dc_a.0, dc_b.0, dc_c.0);
    }
}

impl PosSensor for SomePlatformSpecificImpl {
    type Output = I16F16;

    /// For an actual motor, this would read a position encoder, and give you a position. This
    /// might be milliradians, for example.
    /// In this case, we are just doing pi/3, i.e. 60 degrees
    fn get_position_um(&self) -> Self::Output {
        I16F16::from_num(FRAC_PI_3)
    }
}

