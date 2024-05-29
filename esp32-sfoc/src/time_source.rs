
struct Timer0<TG: TimerGroupInstance> {
    timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>,
}

impl<TG: TimerGroupInstance> Timer0<TG> {
    fn init(timer: esp_hal::timer::Timer<esp_hal::timer::Timer0<TG>, Blocking>) -> Self {
        timer.enable_peripheral();
        Self { timer }
    }
}
impl<TG: TimerGroupInstance> counters::TimeCount for Timer0<TG> {
    type RawData = u64;
    type TickMeasure = fugit::Instant<u64, 1, 80_000_000>;
    type Error = ();

    fn try_now_raw(&self) -> Result<Self::RawData, Self::Error> {
        Ok(self.timer.now())
    }

    fn raw_to_measure(from: Self::RawData) -> Self::TickMeasure {
        Self::TickMeasure::from_ticks(from)
    }
}

