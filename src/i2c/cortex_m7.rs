use cortex_m::peripheral::I2C;
use cortex_m_rt::entry;
use cortex_m::asm::nop;

pub struct CortexM7I2C {
    pub baud_rate: u32,
}

impl CortexM7I2C {
    // Initialize the I2C interface
    pub fn init_i2c(&self) {
        let i2c = unsafe { &*cortex_m::peripheral::I2C::ptr() }; // Get I2C peripheral pointer
        i2c.cr1.modify(|_, w| w.pe().set_bit()); // Enable I2C peripheral (pe = peripheral enable)
    }

    // Send data over I2C to the specified address
    pub fn send_data(&self, address: u8, data: &[u8]) {
        let i2c = unsafe { &*cortex_m::peripheral::I2C::ptr() };

        // Generate START condition
        i2c.cr1.modify(|_, w| w.start().set_bit());

        // Wait for the start condition to be sent
        while !self.is_i2c_ready(i2c) {}

        // Send the address
        i2c.dr.write(|w| w.data().bits(address << 1));

        // Wait for acknowledgment from the slave
        while !self.is_i2c_ready(i2c) {}

        // Send data bytes
        for &byte in data {
            i2c.dr.write(|w| w.data().bits(byte));
            while !self.is_i2c_ready(i2c) {}
        }

        // Generate STOP condition
        i2c.cr1.modify(|_, w| w.stop().set_bit());
    }

    // Receive data over I2C from the specified address
    pub fn receive_data(&self, address: u8, num_bytes: usize) -> Vec<u8> {
        let mut received_data = Vec::new();
        let i2c = unsafe { &*cortex_m::peripheral::I2C::ptr() };

        // Generate START condition
        i2c.cr1.modify(|_, w| w.start().set_bit());

        // Wait for the start condition to be sent
        while !self.is_i2c_ready(i2c) {}

        // Send the address
        i2c.dr.write(|w| w.data().bits((address << 1) | 1));

        // Wait for acknowledgment from the slave
        while !self.is_i2c_ready(i2c) {}

        // Read the requested number of bytes
        for _ in 0..num_bytes {
            while !self.is_i2c_ready(i2c) {}
            received_data.push(i2c.dr.read().data().bits() as u8);
        }

        // Generate STOP condition
        i2c.cr1.modify(|_, w| w.stop().set_bit());

        received_data
    }

    // Check if the I2C peripheral is ready to perform the next action
    fn is_i2c_ready(&self, i2c: &I2C) -> bool {
        i2c.sr1.read().sb().bit_is_clear()
    }
}
