#[cfg(feature = "atmega328p")]
pub mod atmega328p;
#[cfg(feature = "cortex_m7")]
pub mod cortex_m7;

pub trait Spi {
    fn init_spi(is_master: bool, baud_rate: u32);
    fn send_data(data: u8);
    fn receive_data() -> u8;
}
