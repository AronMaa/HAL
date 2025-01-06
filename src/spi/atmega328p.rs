use avr_device::atmega328p::{SPI, SPCR, SPSR};
use avr_device::atmega328p::USART0;

pub struct Atmega328pSpi {
    pub is_master: bool,
    pub baud_rate: u32,
}

impl Atmega328pSpi {
    pub fn init_spi(is_master: bool, baud_rate: u32) {
        // Initialize SPI for Atmega328p
        unsafe {
            if is_master {
                SPI.spcr.write(|w| w.bits(0b01010000)); // Enable SPI, Master mode
                SPI.spsr.write(|w| w.bits(0b00000001)); // Double speed mode
            } else {
                SPI.spcr.write(|w| w.bits(0b00010000)); // Enable SPI, Slave mode
            }

            let baud_rate = 16_000_000 / (2 * baud_rate);
            SPI.spcr.write(|w| w.bits(baud_rate));
        }
    }

    pub fn send_data(data: u8) {
        unsafe {
            while SPI.spsr.read().bits() & (1 << 7) == 0 {}  // Wait for SPI to be ready
            SPI.spdr.write(|w| w.bits(data));  // Send data via SPI
        }
    }

    pub fn receive_data() -> u8 {
        let data: u8;
        unsafe {
            while SPI.spsr.read().bits() & (1 << 7) == 0 {}  // Wait for SPI to receive data
            data = SPI.spdr.read().bits();  // Read received data
        }
    }
}
