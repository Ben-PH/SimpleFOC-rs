#![no_std]
pub use sfoc_rs_core::*;
#[cfg(feature = "esp32")]
pub use esp32_sfoc;
#[cfg(feature = "esp32s3")]
pub use esp32s3_sfoc;
