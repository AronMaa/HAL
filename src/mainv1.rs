#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// Define addresses for registers controlling Port B, Port C, and Port D
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

const DDRC: *mut u8 = 0x27 as *mut u8;
const PORTC: *mut u8 = 0x28 as *mut u8;
const PINC: *mut u8 = 0x26 as *mut u8;

const DDRD: *mut u8 = 0x2A as *mut u8;
const PORTD: *mut u8 = 0x2B as *mut u8;
const PIND: *mut u8 = 0x29 as *mut u8;

// Enum for the three ports available on the Atmega328p
pub enum Port {
    B,
    C,
    D,
}

// Enum for pin mode configuration
pub enum PinMode {
    Input,
    Output,
    InputPullUp,
}

// Struct for handling any digital pin on any port
pub struct DigitalPin {
    port: Port,
    pin: u8,
}

impl DigitalPin {
    // Initialize a digital pin with a specific port and pin number
    pub fn new(port: Port, pin: u8) -> Self {
        DigitalPin { port, pin }
    }

    // Configure the pin mode (input, output, or input with pull-up resistor)
    pub fn configure(&self, mode: PinMode) {
        unsafe {
            match self.port {
                Port::B => Self::set_pin_mode(DDRB, PORTB, self.pin, mode),
                Port::C => Self::set_pin_mode(DDRC, PORTC, self.pin, mode),
                Port::D => Self::set_pin_mode(DDRD, PORTD, self.pin, mode),
            }
        }
    }

    // Write a value to the pin (HIGH or LOW)
    pub fn write(&self, high: bool) {
        unsafe {
            match self.port {
                Port::B => Self::write_pin(PORTB, self.pin, high),
                Port::C => Self::write_pin(PORTC, self.pin, high),
                Port::D => Self::write_pin(PORTD, self.pin, high),
            }
        }
    }

    // Read the value of the pin (HIGH or LOW)
    pub fn read(&self) -> bool {
        unsafe {
            match self.port {
                Port::B => Self::read_pin(PINB, self.pin),
                Port::C => Self::read_pin(PINC, self.pin),
                Port::D => Self::read_pin(PIND, self.pin),
            }
        }
    }

    // Helper function to set the pin mode
    unsafe fn set_pin_mode(ddr: *mut u8, port: *mut u8, pin: u8, mode: PinMode) {
        match mode {
            PinMode::Output => {
                *ddr |= 1 << pin; // Set pin as output
                *port &= !(1 << pin); // Clear PORTxn for output low
            }
            PinMode::Input => {
                *ddr &= !(1 << pin); // Set pin as input
                *port &= !(1 << pin); // Disable pull-up
            }
            PinMode::InputPullUp => {
                *ddr &= !(1 << pin); // Set pin as input
                *port |= 1 << pin;  // Enable pull-up
            }
        }
    }

    // Helper function to write to a pin
    unsafe fn write_pin(port: *mut u8, pin: u8, high: bool) {
        if high {
            *port |= 1 << pin; // Set pin HIGH
        } else {
            *port &= !(1 << pin); // Set pin LOW
        }
    }

    // Helper function to read from a pin
    unsafe fn read_pin(pin_reg: *mut u8, pin: u8) -> bool {
        (*pin_reg & (1 << pin)) != 0 // Return true if HIGH, false if LOW
    }
}

// Example main function demonstrating the use of all digital pins on all ports
#[no_mangle]
pub extern "C" fn main() {
    // Example: Configure and use pin 2 on Port D (Arduino digital pin 2)
    let pin2 = DigitalPin::new(Port::D, 2);
    pin2.configure(PinMode::InputPullUp);
    let state = pin2.read();

    // Example: Configure and use pin 3 on Port D (Arduino digital pin 3)
    let pin3 = DigitalPin::new(Port::D, 3);
    pin3.configure(PinMode::Output);
    pin3.write(true); // Set pin HIGH

    // Example: Configure and use pin 13 on Port B (Arduino built-in LED)
    let led = DigitalPin::new(Port::B, 5); // Pin 13 is PB5
    led.configure(PinMode::Output);
    led.write(true); // Set pin HIGH (turn on LED)
}
