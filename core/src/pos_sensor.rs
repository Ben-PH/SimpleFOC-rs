pub trait PosSensor {
    type Output;
    // simplification: interprate as micrometers/microradians for now
    fn get_position_um(&self) -> Self::Output;
}

pub trait ABEncoder {
    /// e.g. for esp32, this will be an i32
    type RawOutput;
    fn read(&self) -> Self::RawOutput;
}
