#[cfg(feature = "atmega328p")]
pub mod atmega328p;
#[cfg(feature = "cortex_m7")]
pub mod cortex_m7;

pub trait I2C {
    fn init_i2c(&self);
    fn send_data(&self, address: u8, data: &[u8]);
    fn receive_data(&self, address: u8, num_bytes: usize) -> Vec<u8>;
}
