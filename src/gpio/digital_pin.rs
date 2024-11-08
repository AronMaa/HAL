use crate::gpio::{Port, PinMode};  // Importing necessary types

pub struct DigitalPin {
    port: Port,
    pin: u8,
}

impl DigitalPin {
    pub fn new(port: Port, pin: u8) -> Self {
        DigitalPin { port, pin }
    }

    pub fn configure(&self, mode: PinMode) {
        unsafe {
            // Borrow the port value and pass it to `get_registers`
            let (ddr, port, _) = self.port.get_registers(); // Borrow `self.port`
            Self::set_pin_mode(ddr, port, self.pin, mode);
        }
    }

    pub fn write(&self, high: bool) {
        unsafe {
            // Borrow the port value and pass it to `get_registers`
            let (_, port, _) = self.port.get_registers(); // Borrow `self.port`
            Self::write_pin(port, self.pin, high);
        }
    }

    pub fn read(&self) -> bool {
        unsafe {
            // Borrow the port value and pass it to `get_registers`
            let (_, _, pin_reg) = self.port.get_registers(); // Borrow `self.port`
            Self::read_pin(pin_reg, self.pin)
        }
    }

    pub fn toggle(&self) {
        unsafe {
            // Borrow the port value and pass it to `get_registers`
            let (_, port, _) = self.port.get_registers(); // Borrow `self.port`
            Self::toggle_pin(port, self.pin);
        }
    }

    // Helper functions remain the same (no changes here)
    unsafe fn set_pin_mode(ddr: *mut u8, port: *mut u8, pin: u8, mode: PinMode) {
        match mode {
            PinMode::Output => {
                *ddr |= 1 << pin;
                *port &= !(1 << pin);
            }
            PinMode::Input => {
                *ddr &= !(1 << pin);
                *port &= !(1 << pin);
            }
            PinMode::InputPullUp => {
                *ddr &= !(1 << pin);
                *port |= 1 << pin;
            }
        }
    }

    unsafe fn write_pin(port: *mut u8, pin: u8, high: bool) {
        if high {
            *port |= 1 << pin;
        } else {
            *port &= !(1 << pin);
        }
    }

    unsafe fn read_pin(pin_reg: *mut u8, pin: u8) -> bool {
        (*pin_reg & (1 << pin)) != 0
    }

    unsafe fn toggle_pin(port: *mut u8, pin: u8) {
        *port ^= 1 << pin;
    }
}
