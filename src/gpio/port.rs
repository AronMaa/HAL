// Define the addresses for each port
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

const DDRC: *mut u8 = 0x27 as *mut u8;
const PORTC: *mut u8 = 0x28 as *mut u8;
const PINC: *mut u8 = 0x26 as *mut u8;

const DDRD: *mut u8 = 0x2A as *mut u8;
const PORTD: *mut u8 = 0x2B as *mut u8;
const PIND: *mut u8 = 0x29 as *mut u8;

#[derive(Clone, Copy)]
pub enum Port {
    B,
    C,
    D,
}

impl Port {
    // This function will now return references to the registers, not move the `port`.
    pub fn get_registers(&self) -> (*mut u8, *mut u8, *mut u8) {
        match *self {
            Port::B => (DDRB, PORTB, PINB),
            Port::C => (DDRC, PORTC, PINC),
            Port::D => (DDRD, PORTD, PIND),
        }
    }
}
