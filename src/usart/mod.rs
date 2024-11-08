pub mod atmega328p;
pub mod cortex_m7;

pub trait Usart {
    fn init(baud_rate: u32);
    fn transmit(data: u8);
    fn receive() -> u8;
}

