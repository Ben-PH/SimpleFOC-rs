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

    use sfoc_rs::{common::types::VelocityPID, pid_reexported::Pid};

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

use embedded_hal::{digital::InputPin, pwm::SetDutyCycle};
use embedded_time::Clock;
use initial_config::*;
use sfoc_rs::{base_traits::foc_control::FOController, common::helpers::Triplet};

// as it currently stands, the user will need to use platform specific code to get the right pins.
// This can very in complexity, but the goal is to make it
// e.g. for esp32:
//
// ```
//     // expose the resources
//     let peripherals = Peripherals::take();
//     let system = peripherals.SYSTEM.split();
//
//     // marshal the pins for encoder and motor mins
//     let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
//     let pins = io.pins;
//
//     let encoder_pins = (
//         pins.gpio1.into_pull_up_input(),
//         pins.gpio2.into_pull_up_input(),
//     );
//
//     let motor_pins = Triplet {
//         phase_a: pins.gpio3,
//         phase_a: pins.gpio4,
//         phase_a: pins.gpio5,
//      };
//
//     // create a time-source. This would feasibly be baked into the platform library, but easily
//     enough implemented as part of sfoc platform support.
//     let clock_ctrl = ClockControl::boot_defaults(system.clock_control);
//     let clocks: Clocks = clock_ctrl.freeze();
//     let group = TimerGroup::new(peripherals.TIMG0, &clocks, None);
//     let time_src = Timer0::init(group.timer0);
//
//     entry(motor_pins, encoder_pins, time_src)
// ```
fn entry<
    PA: SetDutyCycle,
    PB: SetDutyCycle,
    PC: SetDutyCycle,
    MPinSource: Into<Triplet<PA, PB, PC>>,
    EncA: InputPin,
    EncB: InputPin,
>(
    motor_pins: MPinSource,
    encoder_pins: (EncA, EncB),
    time_getter: impl Clock,
) -> ! {
    // ...my thinking being is that the hardware support library defines how the pins are made
    // available/initialised. e.g.:
    // ```
    // impl From<$PINS> for `Triplet<...> {
    //     fn into(...) -> ... {
    //         $hw-specific-conversions-here
    //     }
    // }
    let motor_pins: Triplet<PA, PB, PC> = motor_pins.into();

    // Ideally, this `Placeholder` type would encapsulate specific implementations, such as BLDC(4/6)/Stepper(2/4) and platform. That's a fair way away, at least for now, and not so sure what that will look like. Key points of this comment:
    //  - Platform selection (i.e. which MCU to use) is aliased/abstracted/etc out of user-code almost entirely.
    //  - Use-case specifics such as which motor is used, voltages, pins, etc is encapsulated at
    //  the type level.
    //
    //  The FOController trait, and the init function, must have its constraints setup so that
    //  incompatable pins (i.e. not all pins can be encoder input pins. motor pins should belong to
    //  the same timer) is a compiletime error. This places a burden on making sure the
    //  platform-specific pin-getters are type-constrained properly.
    //
    let foced_up_motor: PlaceHolderFOCInstance = FOController::init_fo_control(
        encoder_pins,
        motor_pins,
        initial_velocity_pid(),
        time_getter,
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
    todo!()
}
