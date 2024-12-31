#![no_std]
#![no_main]

#[cfg(feature = "atmega328p")]
use spi::atmega328p::{init_spi, send_data, receive_data};

#[cfg(feature = "cortex_m7")]
use spi::cortex_m7::{init_spi, send_data, receive_data};

mod panic;
mod gpio;
mod usart;

use gpio::{DigitalPin, Port, PinMode};
use usart::atmega328p::Atmega328pUsart;
use usart::cortex_m7::CortexM7Usart;
use usart::Usart;

#[no_mangle]
pub extern "C" fn main() {
    let pin2 = DigitalPin::new(Port::D, 2);
    pin2.configure(PinMode::InputPullUp);
    let state = pin2.read();

    let pin3 = DigitalPin::new(Port::D, 3);
    pin3.configure(PinMode::Output);
    pin3.write(true);

    let led = DigitalPin::new(Port::B, 5);
    led.configure(PinMode::Output);
    led.write(true);

    // Initialize USART with a baud rate of 9600 for Atmega328p
    Atmega328pUsart::init(9600);

    // Transmit a character
    Atmega328pUsart::transmit(b'H');
    Atmega328pUsart::transmit(b'e');
    Atmega328pUsart::transmit(b'l');
    Atmega328pUsart::transmit(b'l');
    Atmega328pUsart::transmit(b'o');

    // Receive data (for demonstration, in a real application this might loop or process continuously)
    let received = Atmega328pUsart::receive();

    println!("Démarrage du programme HAL...");
    /*
    [CORRECTION SPI] (Don't hesitate to remove this part)
    Since you are working without the standard crate, you cannot use println! macro.
    Eventually, you could use hprintln! macro for certain target (CORTEX M7 for example), but not for the ATMEGA328P.
    This imply you do conditionnal compilation, to use hprintln! macro only if you compile your project for a CORTEX M7 target for example.

    But it seemed that you decide to compile your project differently, by using multiple main() function? Please give instruction in your README if the user has to modify the entry point of your project according to the target (it seemsed to by the case here).
    (if you do so, it won't solve the println! error, and you still cannot compile hprintln! macro for your ATMEGA328P target. Don't hesitate to send me an email if you need help)
    */
    init_spi();

    send_data(0x55); // Envoi de la donnée 0x55
    let received = receive_data(); // Réception de données
    println!("Données reçues : {}", received);
}

// Main function for MCU 1 (Atmega328p) - Transmitter
#[no_mangle]
pub extern "C" fn main_mcu1() {
    // Initialize USART for Atmega328p with a baud rate of 9600
    Atmega328pUsart::init(9600);

    // Message to send
    let message = b"Hello, MCU 2!";

    // Transmit each byte of the message
    for &byte in message.iter() {
        Atmega328pUsart::transmit(byte);
    }

    // Optionally, wait for a response
    let response = Atmega328pUsart::receive();
    // Handle the response (e.g., blink an LED or store in a variable)
}

// Main function for MCU 2 (Cortex-M7) - Receiver
#[no_mangle]
pub extern "C" fn main_mcu2() {
    // Initialize USART for Cortex-M7 with a baud rate of 9600
    CortexM7Usart::init(9600);

    // Buffer to store received message
    let mut buffer = [0u8; 12]; // Assuming a 12-byte message max
    let mut index = 0;

    // Receive each byte of the message
    while index < buffer.len() {
        buffer[index] = CortexM7Usart::receive();
        index += 1;
    }

    // Optional: Send an acknowledgment back
    let ack = b"Received!";
    for &byte in ack.iter() {
        CortexM7Usart::transmit(byte);
    }
}
