Simple FOC, but with Rust.

Still a heavy WIP. Getting close to a very basic const-voltage implementation for esp32.

One motivation is the prospect of, sometime down the line, there being a Rust option in which to explore the Simple FOC playground. To that end, some design principals that I try to think about:

- No suprises when moving between Arduino and Rust: Moving between the two should bring a sense of familiarity. It should feel like applying the same concepts, in familiar patterns, just using idioms that change with the language being used.
- Simplicity, Accesibility, and Educational Enjoyment: Simple FOC delliberately makes design and implementation decisions that prioritises ease of use and enjoyment, and I aim to follow their lead. This will often imply that in order to make _using_ this library simple, the underlying code must bear the burden of managing complexity.
- Advanced usage: Though not a primary aim, a bonus acheivement would include suitability for more advanced and nuanced use, such as 3d printer firmware. For emphasis: this goal is secondary to the first two.



### With respect to licence and assigning credit for work: 

This project started as a direct rewrite of the Simple FOC library. This has changed a touch, as Rust idioms and C++ Arduino idoms are at times in conflict. With that said, the project has gotten to its current state with a lot of direct input from the SFOC people, and this project continues to draw heavily from the original works of the SFOC Arduino library.

At time of writing, this is a re-write of Simple FOC library. "We stand on the shoulders of giants" couldn't apply more here. Please refer to the Citations file, and the OG library [here](https://github.com/simplefoc/Arduino-FOC)

#### Simple demo

`cargo run --expample spinny` each second, prints out the duty cycles to make a slowly rotating magnetic field

#### Supported platforms

None yet, but esp32 and esp32s3 are in the works

#### How to use this library.

Still working out that story, but here is where we are right now:

- `cargo make check` to check the code base
- `cargo build` in any of the projects in `platforms` to build for a platform


This is, essentially, the goal:

1. fork and clone this repo
2. `cd platforms <your-chip>`
3. trivially-easy task to assign gpio pins for motor control and motor feedback pins
4. `cargo run` to run a default, slow-spinning motor
5. plug in functions/modules/features/etc. to make your program less trivial
6. ???
7. profit
