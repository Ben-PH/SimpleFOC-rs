use esp_hal::pcnt::unit;


/// micrometers for each pulse
pub struct EncoderPosn {
    // the underlying esp32 pulse count reader
    unit: unit::Unit,
}

impl EncoderPosn {
    pub fn new(unit: unit::Unit) -> Self { Self { unit } }
}

impl counters::Counter for EncoderPosn {
    type RawData = i16;
    type CountMeasure = i16;
    type Error = ();
    fn try_read_raw(&self) -> Result<Self::RawData, Self::Error> {
        Ok(self.unit.get_value())
    }

    fn raw_to_measure(_from: Self::RawData) -> Self::CountMeasure {
        todo!("Each pulse should be scaled by a distance here")
    }
}

