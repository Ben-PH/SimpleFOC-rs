This is where platform specific setups go to in order to become generic.

The idea being, is that you marshal pins, and set things up as far as the hardware is concerned in the other spinnies, and then you just call `generic_spinny::main_loop(...)`


This will only rotate a magnetic field at 10rpm at a 10% power setting.

The roadmap calls for the next to be a PID spinny, which uses position sensing to set a 10rpm rotation.

Also plan on writing:

- constant speed spinny
- pot-controlled position hold
- pot-controlled speed hold
