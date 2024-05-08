use core::{marker::PhantomData, num::NonZeroU64};

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

pub struct Velocity<D, T: From<NonZeroU64>> {
    pub displacement: D,
    pub time: T,
}

impl<D, T> Default for Velocity<D, T>
where
    D: Default,
    T: From<NonZeroU64> + Default,
{
    fn default() -> Self {
        Self {
            displacement: Default::default(),
            time: NonZeroU64::new(1).unwrap().into(),
        }
    }
}
