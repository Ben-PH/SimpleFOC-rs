use core::marker::PhantomData;

pub trait MovementOrientation {
    fn dirstr(dir: &Option<bool>) -> &'static str;
}

struct LinearMovement;
impl MovementOrientation for LinearMovement {
    fn dirstr(dir: &Option<bool>) -> &'static str {
        match dir {
            Some(true) => "Forwards",
            Some(false) => "Backwards",
            None => "Unknown",
        }
    }
}
struct RotaryMovement;
impl MovementOrientation for RotaryMovement {
    fn dirstr(dir: &Option<bool>) -> &'static str {
        match dir {
            Some(true) => "Clockwise",
            Some(false) => "CounterClockwise",
            None => "Unknown",
        }
    }
}

struct Direction<T: MovementOrientation> {
    dir: Option<bool>,
    orientation: PhantomData<T>,
}

impl<T: MovementOrientation> core::fmt::Debug for Direction<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Direction")
            .field(&T::dirstr(&self.dir))
            .finish()
    }
}

// So velocity must have a non-zero denominator. Because this is a run-time thing, the use of
// typnum for the denominator would explode the binary size
//
// The default is looking like this Because
//  a) at time of writing, this was a "get it done" solution
//  b) can't think of a nicer way to ensure that the denominotor is constrained to have a suitable
//  deafult impl
// impl<D, T, E> Default for Velocity<D, T>
// where
//     D: Default,
//     E: core::fmt::Debug,
//     T: TryFrom<NonZeroU64, Error = E>,
// {
//     fn default() -> Self {
//         Self {
//             displacement: Default::default(),
//             // this should be trivial for the compiler to optimise into a single op
//             time: T::try_from(NonZeroU64::new(1).unwrap()).unwrap(),
//         }
//     }
// }
