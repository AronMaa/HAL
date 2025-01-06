#![no_std]
#![no_main]

#[cfg(feature = "atmega328p")]
use spi::atmega328p::{init_spi, send_data, receive_data};
use usart::atmega328p::Atmega328pUsart;

#[cfg(feature = "cortex_m7")]
use spi::cortex_m7::{init_spi, send_data, receive_data};
use usart::cortex_m7::CortexM7Usart;

mod panic;
mod gpio;
mod usart;
mod spi;

use gpio::{DigitalPin, Port, PinMode};
use usart::Usart;
use spi::Spi;

#[no_mangle]
pub extern "C" fn main() {
    // Initialize platform-specific USART
    #[cfg(feature = "atmega328p")]
    {
        Atmega328pUsart::init(9600);
    }

    #[cfg(feature = "cortex_m7")]
    {
        CortexM7Usart::init(9600);
    }

    // Configure GPIO pins
    let pin2 = DigitalPin::new(Port::D, 2);
    pin2.configure(PinMode::InputPullUp);
    let state = pin2.read();

    let pin3 = DigitalPin::new(Port::D, 3);
    pin3.configure(PinMode::Output);
    pin3.write(true);

    let led = DigitalPin::new(Port::B, 5);
    led.configure(PinMode::Output);
    led.write(state);

    // Transmit a message
    let message = b"Hello!";
    transmit_message(message);

    // Receive a byte and use it
    match receive_byte() {
        byte if byte != 0 => handle_received_byte(byte),
        _ => debug_log("No valid data received."),
    };

    // SPI communication
    init_spi(true, 115200);
    send_data(0x55);
    match receive_data() {
        data => debug_log(&format!("SPI data received: {}", data)),
    }
}

fn transmit_message(message: &[u8]) {
    #[cfg(feature = "atmega328p")]
    {
        for &byte in message {
            Atmega328pUsart::transmit(byte);
        }
    }

    #[cfg(feature = "cortex_m7")]
    {
        for &byte in message {
            CortexM7Usart::transmit(byte);
        }
    }
}

fn receive_byte() -> u8 {
    #[cfg(feature = "atmega328p")]
    {
        Atmega328pUsart::receive()
    }

    #[cfg(feature = "cortex_m7")]
    {
        CortexM7Usart::receive()
    }
}

fn handle_received_byte(byte: u8) {
    // Example: Blink the LED or log
    #[cfg(feature = "atmega328p")]
    {
        let led = DigitalPin::new(Port::B, 5);
        led.write(byte != 0); // Use the received byte to control the LED
    }

    #[cfg(feature = "cortex_m7")]
    {
        debug_log(&format!("Received byte: {}", byte));
    }
}

fn debug_log(message: &str) {
    #[cfg(feature = "atmega328p")]
    {
        transmit_message(message.as_bytes());
    }

    #[cfg(feature = "cortex_m7")]
    {
        use cortex_m_semihosting::hprintln;
        hprintln!("{}", message).unwrap();
    }
}
