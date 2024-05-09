```rust
// Top level interface
pub trait FOController: Sized {
    fn init_fo_control() -> Result<Self, ()>;
    fn enable(&mut self);
    fn disable(&mut self);
    // fn link_sensor/current_sensor(....
    fn init_foc_algo(&mut self) -> u32; // why the u32?
    fn foc_loop(&mut self) -> !;
    fn move_command(motion: MotionCtrl);
    fn set_phase_voltage(u_q: f32, u_d: f32, phase_angle: PhaseAngle);
}
```

...stuff here

```rust
// bottom level, to-the-hardware interface
pub trait BLDCDriver: Sized {
    fn init_bldc_driver() -> Result<Self, ()>;
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_pwms(&mut self, dc_a: DutyCycle, dc_b: DutyCycle, dc_c: DutyCycle);
    fn set_phase_state(&mut self, ps_a: PhaseState, ps_b: PhaseState, ps_c: PhaseState);
}
```

...the encoder needs to have its a and b channels defined, as well as interupts if applicable.

the foc controller interface needs position, driver, and the option for monotoring over serial interface.

if doing velocity control, at the global scope:

 - BLDCController is constructed with pole-pairs 11
 - 3pwm driver is constructed with the **3 pin numbers,** then an enable pin chosen
 - An encoder is constructed, with **pins**, ppr and index 0xA0 (pin 36, i think?)
 - interupt handlers for the a, b and index pulse are defined and setup
 - A commander is constructed with a serial
 - **target velocity** global is defined. it's used by `doTarget` and `motor.move` in the main-loop. the controler needs access to this
 - doTarget is defined. taking a command, and running it with reference to &target_velocity


the setup() function:

 - **initialises the encoder**
 - enables interupts on the encoder
 - registers the software interupts as a fallback if harware ints aren't there
 - **links said encoder** into the FOCControllerBLDC global instance
 - sets a **driver voltage** power supply
 - initialises the driver
 - **links said BLDCdriver** to the global FOCControllerBLDC instance
 - **sets control type** to velocity
 - sets pid values of velocity
 - sets voltage limit to the FOCControllerBLDC
 - sets lp-filter time-constant for the FOCControllerBLDC velocity
 - begins the monitoring
 - initialises the FOCControllerBLDC
 - initialises the foc of the FOCControllerBLDC
 - adds a "target velocity" command
 - "motor ready"
 - "set the target velocity using serial"


`loop()` function:

- FOCControllerBLDC::loopFOC()
- FOCControllerBLDC::move(target_velocity_global)
- command.run() // user-communication


so it comes down to having something that is an implementation of foc control, either stepper or bldc.

...but the foccontroller needs a driver with a motor behind it and a position encoder with an encoder behind it.

...the shopping list of resources includes the pins


so brass-tax on setup:

- pins (encoder, motor)
- initial target velocity
- power supply voltage (driver assisciated)
- voltage limit (controller assosciated)
- initial pid values
- lp filter time constant

...but this doesn't account for the need for a time-source. arduino just uses some obfuscated global, but that just won't do...

