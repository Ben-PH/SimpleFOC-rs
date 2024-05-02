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
