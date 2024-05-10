/// If you're reading this, I've probably asked for your thoughts on this as a "design reference".
/// The goal is that the examples can be 100% self documenting. The comments throughout this file
/// are to communicate with you the thinking/philosophy/etc.
///
/// This is intended as a simple-foc-rs version of the velocity_control example in the arduino
/// library.

mod pin_numbers {
    use typenum::*;
    pub type PhaseA = U3;
    pub type PhaseB = U4;
    pub type PhaseC = U5;

    pub type EncA = U1;
    pub type EncB = U2;
}

mod initial_config {

    use sfoc_rs::{pid_reexported::Pid, common::types::VelocityPID};

    pub fn initial_velocity_pid() -> VelocityPID {
        let mut v_pid = VelocityPID(Pid::new(0.0, 6.0));
        v_pid.0.kp = 0.2;
        v_pid.0.ki = 2.0;
        v_pid.0.kd = 0.0;
        v_pid
    }
    // 12 volts
    pub type DriverVoltagePwrSup = typenum::U12;
}


use pin_numbers::*;
use initial_config::*;
use sfoc_rs::base_traits::foc_control::FOController;
fn entry() -> ! {
    // Here we need to specify to the type-system which implementation is being used. For each
    // supported platform, a `struct` must implement these three traits to allow for the `<$OBJECt as $TRAIT>::$TRAIT_FN` pattern
    // I'm not 100% convinced this is the best way, and I'm pretty sure type-inference means
    // `$TRAIT::$TRAIT_FN(...)` will be sufficient, so long as `init_fo_control` (or elsewhere)
    // lets the compiler infer "okay, I understand which implementation of these traits i need to
    // use" 
    let motor_pins = <Platform as MotorPinClaimer>::motor_pins::<PhaseA, PhaseB, PhaseC>();
    let encoder_pins = <Platform as EncoderPinClaimer>::encoder_pins::<EncA, EncB>();
    let time_getter = <Platform as TimeSourceBuilder>::get_source_clock_implementor();
    

    // Ideally, this `Placeholder` type would encapsulate specific implementations, such as BLDC(4/6)/Stepper(2/4) and platform. That's a fair way away, at least for now, and not so sure what that will look like. Key points of this comment:
    //  - Platform selection (i.e. which MCU to use) is aliased/abstracted/etc out of user-code almost entirely. 
    //  - Use-case specifics such as which motor is used, voltages, pins, etc is encapsulated at
    //  the type level.
    //
    //  The FOController trait, and the init function, must have its constraints setup so that
    //  incompatable pins (i.e. not all pins can be encoder input pins. motor pins should belong to
    //  the same timer) is a compiletime error. This places a burden on making sure the
    //  platform-specific pin-getters are type-constrained properly.
    let foced_up_motor: PlaceHolderFOCInstance = FOController::init_fo_control(
        encoder_pins,
        motor_pins,
        initial_velocity_pid(),
        time_getter
    );

    // I'm not exactly sure what is going on with the `command` global, and in the velocity control
    // example, what is going on exactly there. This is an approximation: you can optionally have a
    // tx, rx channel that acts as a command buffer that gets filled... somehow?
    let command_channel = CommandChannel::init();
    foced_up_motor.attach_command_recv(command_channel.rx);

    foced_up_motor.begin_monitoring(Serial::new(115200));

    command_channel.tx.push(('T', (), "target velocity"));
    println!("Motor ready");
    println!("Using the serial terminal to set a target velocity:");

    main_loop(foced_up_motor, command_channel.rx)
}

fn main_loop(controller: (), command_channel: ()) -> ! {
    loop {
        controller.loopFOC();
        controller.move_motor();

        todo!("workout the sfoc command.run() idiom and downstream it to here");
    }
}

fn main() {
    entry()
}

