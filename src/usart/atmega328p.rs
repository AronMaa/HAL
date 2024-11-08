use core::ptr;

pub struct Atmega328pUsart;

impl Atmega328pUsart {
    // Baud rate calculation
    const fn calculate_ubrr(baud: u32) -> u16 {
        // Assuming an F_CPU of 16 MHz
        ((16_000_000 / (16 * baud)) - 1) as u16
    }
}

impl super::Usart for Atmega328pUsart {
    fn init(baud_rate: u32) {
        let ubrr = Self::calculate_ubrr(baud_rate);
        unsafe {
            // Set baud rate (UBRR0H and UBRR0L registers)
            ptr::write_volatile(0xc5 as *mut u8, (ubrr >> 8) as u8);
            ptr::write_volatile(0xc4 as *mut u8, ubrr as u8);

            // Enable transmitter and receiver (TXEN0 and RXEN0 bits in UCSR0B)
            ptr::write_volatile(0xc1 as *mut u8, 1 << 3 | 1 << 4);

            // Set frame format: 8 data bits, 1 stop bit (UCSR0C)
            ptr::write_volatile(0xc2 as *mut u8, 1 << 1 | 1 << 2);
        }
    }

    fn transmit(data: u8) {
        unsafe {
            // Wait for empty transmit buffer
            while ptr::read_volatile(0xc0 as *const u8) & (1 << 5) == 0 {}
            // Put data into buffer, sends the data
            ptr::write_volatile(0xc6 as *mut u8, data);
        }
    }

    fn receive() -> u8 {
        unsafe {
            // Wait for data to be received
            while ptr::read_volatile(0xc0 as *const u8) & (1 << 7) == 0 {}
            // Get and return received data from buffer
            ptr::read_volatile(0xc6 as *const u8)
        }
    }
}
