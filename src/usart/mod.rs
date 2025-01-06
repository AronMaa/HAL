#[cfg(feature = "atmega328p")]
pub mod atmega328p;
#[cfg(feature = "cortex_m7")]
pub mod cortex_m7;

pub trait Usart {
    fn init(baud_rate: u32);
    fn transmit(data: u8);
    fn receive() -> u8;
}
