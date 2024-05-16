simple FOC, rewritten in rust.

### State of project

Without any actual testing done, the first proofe of concept seems to be there. For a platform to be supported:

1. Define the assembly of peripherals needed (pins, pulse-counters, etc) for said platform, using that platforms idioms.
2. Implement the relevant traits on said assembly.

For example, the esp32 with the esp-hal crate allows you to pass in pins, mcpwm peripheral, and timer group peripheral in their own way.

you then initialise said peripherals (e.g. connecting pins to the mcpwm operators), and assemble them into a struct.

Now: think about the process of setting a phase voltage when at a given angle. It comes in two parts:

1.0: Determine the voltages that each phase needs to be seeing
1.1: Map these voltages to the pwm duty cycle for each pin
2:   Setup the pins to run with said duty cycles

The requirements for a platform to be supported comes down to these key notions:

- Implement the desired traits for a struct
- Ensure that the struct contains data thet allows the struct to meet the trait constraints

E.g. to set the duty cycles of the phases, you must have access to three items that implement the `embedded_hal::pwm::SetDutyCycle` trait. This implies that the constructor must include three arguments, constrained in such a way that they must be able to be transformed into something that implements this trait, and this post-transformation item becomes part of the final struct.



#### with respect to licence and assigning credit for work: 

At time of writing, this is a re-write of Simple FOC library. "We stand on the shoulders of giants" couldn't apply more here. Please refer to the Citations file, and the OG library [here](https://github.com/simplefoc/Arduino-FOC)
