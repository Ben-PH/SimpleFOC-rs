#![no_std]
use sfoc_rs_core::rexports::discrete_count::{
    re_exports::{fixed::types::I20F12, typenum::*},
    CountReader, Counter,
};

struct AS5047Periph;

struct MilliTau(I20F12);
struct Nothing;

impl CountReader for Nothing {
    type ReadErr = ();

    type RawData = u16;

    fn read(&self) -> Result<Self::RawData, Self::ReadErr> {
        todo!()
    }
}
impl Counter for AS5047Periph {
    type Reader = Nothing;

    type Resolution = (U1, U4096);

    type Measure = MilliTau;

    fn update_count_state(
        &mut self,
        _count: sfoc_rs_core::rexports::discrete_count::CountRaw<Self>,
    ) -> Result<(), <Self::Reader as sfoc_rs_core::rexports::discrete_count::CountReader>::ReadErr>
    {
        todo!()
    }

    fn read_count_state(&self) -> &sfoc_rs_core::rexports::discrete_count::CountRaw<Self> {
        todo!()
    }

    fn try_update_count(
        &mut self,
    ) -> Result<(), <Self::Reader as sfoc_rs_core::rexports::discrete_count::CountReader>::ReadErr>
    {
        todo!()
    }

    fn try_read_measure(
        &self,
    ) -> Result<
        Self::Measure,
        <Self::Reader as sfoc_rs_core::rexports::discrete_count::CountReader>::ReadErr,
    > {
        todo!()
    }

    fn measure_count_state(&self) -> Self::Measure {
        todo!()
    }

    fn try_update_and_measure(
        &mut self,
        _count: &sfoc_rs_core::rexports::discrete_count::CountRaw<Self>,
    ) -> Result<
        Self::Measure,
        <Self::Reader as sfoc_rs_core::rexports::discrete_count::CountReader>::ReadErr,
    > {
        todo!()
    }

    fn measure_count(
        _count: &sfoc_rs_core::rexports::discrete_count::CountRaw<Self>,
    ) -> Self::Measure {
        todo!()
    }
}
