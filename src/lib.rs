#![no_std]
#[cfg(feature = "esp32")]
pub use esp32_sfoc;
#[cfg(feature = "esp32s3")]
pub use esp32s3_sfoc;
pub use sfoc_rs_core::*;
#[cfg(feature = "AS5047P")]
#[allow(non_snake_case)]
pub use AS5047P;
