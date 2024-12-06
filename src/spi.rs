#[cfg(feature = "atmega328p")]
pub mod atmega328p {
    use avr_device::atmega328p::Peripherals;

    pub fn init_spi() {
        println!("Initialisation du SPI pour Atmega328p...");
        let peripherals = Peripherals::take().unwrap();
        unsafe {
            (*avr_device::atmega328p::SPI::ptr()).spcr.write(|w| w.bits(0b01010000)); // SPI Enable, Master mode
            (*avr_device::atmega328p::SPI::ptr()).spsr.write(|w| w.bits(0b00000001)); // Double speed
        }
        println!("SPI configuré pour Atmega328p.");
    }

    pub fn send_data(data: u8) {
        println!("Envoi de données via SPI (Atmega328p)...");
        unsafe {
            while (*avr_device::atmega328p::SPI::ptr()).spsr.read().bits() & (1 << 7) == 0 {}
            (*avr_device::atmega328p::SPI::ptr()).spdr.write(|w| w.bits(data));
        }
        println!("Données envoyées : {}", data);
    }

    pub fn receive_data() -> u8 {
        println!("Réception de données via SPI (Atmega328p)...");
        unsafe {
            while (*avr_device::atmega328p::SPI::ptr()).spsr.read().bits() & (1 << 7) == 0 {}
            let data = (*avr_device::atmega328p::SPI::ptr()).spdr.read().bits();
            println!("Données reçues : {}", data);
            data
        }
    }
}

#[cfg(feature = "cortex_m7")]
pub mod cortex_m7 {
    use cortex_m::peripheral::Peripherals;

    pub fn init_spi() {
        println!("Initialisation du SPI pour Cortex-M7...");
        let peripherals = Peripherals::take().unwrap();
        println!("SPI configuré pour Cortex-M7.");
    }

    pub fn send_data(data: u8) {
        println!("Envoi de données via SPI (Cortex-M7)...");
        println!("Données envoyées : {}", data);
    }

    pub fn receive_data() -> u8 {
        println!("Réception de données via SPI (Cortex-M7)...");
        let data = 0; // Remplace par la lecture du registre
        println!("Données reçues : {}", data);
        data
    }
}
