use discrete_count::CountReader;
use esp_hal::pcnt::unit;

/// micrometers for each pulse
pub struct EncoderPosn {
    // the underlying esp32 pulse count reader
    unit: unit::Unit,
}

impl EncoderPosn {
    pub fn new(unit: unit::Unit) -> Self {
        Self { unit }
    }
}

// TODO
pub struct ABReader;

impl CountReader for ABReader {
    type ReadErr = ();

    type RawData = ();

    fn read() -> Result<Self::RawData, Self::ReadErr> {
        todo!()
    }
}

impl discrete_count::Counter for EncoderPosn {
    type Reader = ABReader;

    type Resolution = u64;

    type Measure = u64;

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
    // type RawData = i16;
    // type CountMeasure = i16;
    // type Error = ();
    // fn try_read_raw(&self) -> Result<Self::RawData, Self::Error> {
    //     Ok(self.unit.get_value())
    // }
    //
    // fn raw_to_measure(_from: Self::RawData) -> Self::CountMeasure {
    //     todo!("Each pulse should be scaled by a distance here")
    // }
}
