use discrete_count::CountReader;
use esp_hal::{
    timer::timg::{Timer, Timer0},
    Blocking,
};
use fixed::types::I16F16;
use fugit::Duration;
use typenum::{U1, U10000};

pub struct TimerHolder<T> {
    timer: Timer<Timer0<T>, Blocking>,
}

impl<T> TimerHolder<T> {
    pub fn init(timer: Timer<Timer0<T>, Blocking>) -> Self {
        // timer.
        Self { timer }
    }
}

impl<T> discrete_count::CountReader for TimerHolder<T> {
    type RawData = u64;
    type ReadErr = ();
    fn read(&self) -> Result<Self::RawData, Self::ReadErr> {
        todo!()
        // Ok(self.timer.now())
    }
}
impl<T> discrete_count::Counter for TimerHolder<T> {
    type Reader = Self;

    type Resolution = I16F16;

    type Measure = Duration<<Self::Reader as CountReader>::RawData, U1, U10000>;

    fn update_count_state(
        &mut self,
        count: discrete_count::CountRaw<Self>,
    ) -> Result<(), <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn read_count_state(&self) -> &discrete_count::CountRaw<Self> {
        todo!()
    }

    fn try_update_count(
        &mut self,
    ) -> Result<(), <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn try_read_measure(
        &self,
    ) -> Result<Self::Measure, <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn measure_count_state(&self) -> Self::Measure {
        todo!()
    }

    fn try_update_and_measure(
        &mut self,
        count: &discrete_count::CountRaw<Self>,
    ) -> Result<Self::Measure, <Self::Reader as discrete_count::CountReader>::ReadErr> {
        todo!()
    }

    fn measure_count(count: &discrete_count::CountRaw<Self>) -> Self::Measure {
        todo!()
    }
}
