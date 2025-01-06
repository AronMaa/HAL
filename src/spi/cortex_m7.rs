use cortex_m::peripheral::SPI1;
use cortex_m::register::mmio::ReadWrite;

pub struct CortexM7Spi {
    pub is_master: bool,
    pub baud_rate: u32,
}

impl CortexM7Spi {
    pub fn init_spi(is_master: bool, baud_rate: u32) {
        // Initialize SPI for Cortex-M7
        const SPI1_BASE: u32 = 0x40013000;
        let spi = unsafe { &mut *(SPI1_BASE as *mut SPI1) };
      
        unsafe {
            if is_master {
                spi.cr1.modify(|r, w| w.bits(r.bits() | (1 << 6))); // Master mode
            } else {
                spi.cr1.modify(|r, w| w.bits(r.bits() & !(1 << 6))); // Slave mode
            }

            let baud_rate = 168_000_000 / (2 * baud_rate);
            spi.cr1.modify(|r, w| w.bits(r.bits() & !(0x7 << 3) | (baud_rate << 3)));
        }
    }

    pub fn send_data(data: u8) {
        const SPI1_BASE: u32 = 0x40013000;
        let spi = unsafe { &mut *(SPI1_BASE as *mut SPI1) };

        unsafe {
            while spi.sr.read().bits() & (1 << 1) == 0 {}  // Wait for TXE flag
            spi.dr.write(|w| w.bits(data as u16));  // Send data
        }
    }

    pub fn receive_data() -> u8 {
        const SPI1_BASE: u32 = 0x40013000;
        let spi = unsafe { &mut *(SPI1_BASE as *mut SPI1) };

        let data: u8;
        unsafe {
            while spi.sr.read().bits() & (1 << 0) == 0 {}  // Wait for RXNE flag
            data = spi.dr.read().bits() as u8;  // Read received data
        }
        data
    }
}
