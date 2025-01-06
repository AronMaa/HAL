use core::ptr;

/// Define the USART register addresses for Cortex-M7 (STM32F4)
const USART1_CR1: *mut u32 = 0x40011000 as *mut u32;  // USART Control Register 1 for USART1
const USART1_BRR: *mut u32 = 0x40011008 as *mut u32;  // USART Baud Rate Register for USART1
const USART1_SR: *const u32 = 0x40011000 as *const u32; // USART Status Register for USART1
const USART1_DR: *mut u32 = 0x40011004 as *mut u32;  // USART Data Register for USART1

pub struct CortexM7Usart {
    base_address: u32,
}

impl CortexM7Usart {
    // Baud rate calculation (system clock of 168 MHz)
    const fn calculate_brr(baud: u32) -> u16 {
        ((168_000_000 / baud) as u16)
    }
}

impl super::Usart for CortexM7Usart {
    fn init(baud_rate: u32) {
        let usart_base = self.base_address;
        let brr = Self::calculate_brr(baud_rate);

        unsafe {
            // Step 1: Configure Baud rate
            ptr::write_volatile((usart_base + 0x08) as *mut u32, brr as u32);

            // Step 2: Enable USART, Enable Transmitter and Receiver
            let cr1 = 0x200C
            ptr::write_volatile((usart_base + 0x00) as *mut u32, cr1);
        }
    }

    fn transmit(data: u8) {
        let usart_base = self.base_address;
        unsafe {
            // Wait for the TXE flag in the SR register
            while ptr::read_volatile((usart_base + 0x00) as *const u32) & (1 << 7) == 0 {}

            // Write data to the DR register
            ptr::write_volatile((usart_base + 0x04) as *mut u32, data as u32);
        }
    }

    fn receive() -> u8 {
        let usart_base = self.base_address;
        unsafe {
            // Wait for the RXNE flag in the SR register
            while ptr::read_volatile((usart_base + 0x00) as *const u32) & (1 << 5) == 0 {}

            // Read the received data from the DR register
            ptr::read_volatile((usart_base + 0x04) as *const u32) as u8
        }
    }
}
