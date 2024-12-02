// use esp_hal::{
//
//     time::{Enable, Instance, TimerGroupInstance},
//     Blocking,
// };
// use fixed::types::I16F16;
//
// struct Timer0<TG> {
//     timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>,
// }
//
// impl<TG: TimerGroupInstance> Timer0<TG> {
//     pub fn init(timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>) -> Self {
//         timer.enable_peripheral();
//         Self { timer }
//     }
// }
//
// impl<TG: TimerGroupInstance> discrete_count::CountReader for Timer0<TG> {
//     type RawData = u64;
//     type ReadErr = ();
//     fn read() -> Result<Self::RawData, Self::ReadErr> {
//         todo!()
//         // Ok(self.timer.now())
//     }
// }
// impl<TG: TimerGroupInstance> discrete_count::Counter for Timer0<TG> {
//     type Reader = Self;
//
//     type Resolution = I16F16;
//
//     type Measure = u64;
//
//     fn update_count_state(
//         &mut self,
//         count: discrete_count::CountRaw<Self>,
//     ) -> Result<(), <Self::Reader as discrete_count::CountReader>::ReadErr> {
//         todo!()
//     }
//
//     fn read_count_state(&self) -> &discrete_count::CountRaw<Self> {
//         todo!()
//     }
//
//     fn try_update_count(
//         &mut self,
//     ) -> Result<(), <Self::Reader as discrete_count::CountReader>::ReadErr> {
//         todo!()
//     }
//
//     fn try_read_measure(
//         &self,
//     ) -> Result<Self::Measure, <Self::Reader as discrete_count::CountReader>::ReadErr> {
//         todo!()
//     }
//
//     fn measure_count_state(&self) -> Self::Measure {
//         todo!()
//     }
//
//     fn try_update_and_measure(
//         &mut self,
//         count: &discrete_count::CountRaw<Self>,
//     ) -> Result<Self::Measure, <Self::Reader as discrete_count::CountReader>::ReadErr> {
//         todo!()
//     }
//
//     fn measure_count(count: &discrete_count::CountRaw<Self>) -> Self::Measure {
//         todo!()
//     }
// }
