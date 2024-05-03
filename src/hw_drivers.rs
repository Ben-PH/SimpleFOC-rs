use frunk::HList;
use typenum::{IsLess, IsNotEqual, U7, U5, U0};

pub mod esp32_mcu;

pub trait HWApi<N> 
// 1 to 6, but not 5
where 
      N: IsLess<U7> + IsNotEqual<U5> + IsNotEqual<U0>,
{
    fn config(freq: embedded_time::rate::Hertz<u32>, pins: HList);
    fn write_duty_cycle();
}
