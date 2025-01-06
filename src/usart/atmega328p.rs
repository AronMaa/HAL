use core::ptr;

/// Define the USART register addresses for Atmega328p
const UBRR0H: *mut u8 = 0xc5 as *mut u8;  // High byte of UBRR0
const UBRR0L: *mut u8 = 0xc4 as *mut u8;  // Low byte of UBRR0
const UCSR0B: *mut u8 = 0xc1 as *mut u8;  // USART Control and Status Register B
const UCSR0C: *mut u8 = 0xc2 as *mut u8;  // USART Control and Status Register C
const UDR0: *mut u8 = 0xc6 as *mut u8;    // USART Data Register
const UCSRA: *const u8 = 0xc0 as *const u8; // USART Control and Status Register A

pub struct Atmega328pUsart;

impl Atmega328pUsart {
    // Baud rate calculation for Atmega328p
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
            ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
            ptr::write_volatile(UBRR0L, ubrr as u8);

            // Enable transmitter and receiver (TXEN0 and RXEN0 bits in UCSR0B)
            ptr::write_volatile(UCSR0B, 1 << 3 | 1 << 4);

            // Set frame format: 8 data bits, 1 stop bit (UCSR0C)
            ptr::write_volatile(UCSR0C, 1 << 1 | 1 << 2);
        }
    }

    fn transmit(data: u8) {
        unsafe {
            // Wait for empty transmit buffer (TXC0 in UCSRA)
            while ptr::read_volatile(UCSR0B) & (1 << 5) == 0 {}

            // Put data into buffer, sends the data
            ptr::write_volatile(UDR0, data);
        }
    }

    fn receive() -> u8 {
        unsafe {
            // Wait for data to be received (RXC0 in UCSRA)
            while ptr::read_volatile(UCSR0B) & (1 << 7) == 0 {}

            // Get and return received data from buffer
            ptr::read_volatile(UDR0)
        }
    }
}
