use avr_device::atmega328p::{TWI, TWI_CONTROL, TWI_STATUS, TWI_DATA, TWI_BAUD};

pub struct Atmega328pI2C {
    pub baud_rate: u32,
}

impl Atmega328pI2C {
    pub fn init_i2c(&self) {
        unsafe {
            let prescaler = 1;
            let twbr = 72; // Set the baud rate for 100kHz
            TWI_BAUD.write(twbr);

            // Enable the TWI (I2C) peripheral
            TWI_CONTROL.write(0b10000000);
        }
    }

    pub fn send_data(&self, address: u8, data: &[u8]) {
        unsafe {
            // Start condition
            TWI_CONTROL.write(0b10000001);

            // Wait for the TWI to be ready
            while !self.is_i2c_ready() {}

            // Send the address
            TWI_DATA.write(address << 1);

            // Wait for ACK from slave
            while !self.is_i2c_ready() {}

            // Send data
            for &byte in data {
                TWI_DATA.write(byte);
                while !self.is_i2c_ready() {}
            }

            // Send stop condition
            TWI_CONTROL.write(0b10000000);
        }
    }

    pub fn receive_data(&self, address: u8, num_bytes: usize) -> Vec<u8> {
        let mut received_data = Vec::new();
        unsafe {
            // Start condition
            TWI_CONTROL.write(0b10000001);

            // Wait for TWI to be ready
            while !self.is_i2c_ready() {}

            // Send address
            TWI_DATA.write((address << 1) | 1);

            // Wait for ACK from slave
            while !self.is_i2c_ready() {}

            // Read data
            for _ in 0..num_bytes {
                while !self.is_i2c_ready() {}
                received_data.push(TWI_DATA.read());
            }

            // Send stop condition
            TWI_CONTROL.write(0b10000000);
        }

        received_data
    }

    fn is_i2c_ready(&self) -> bool {
        unsafe {
            TWI_STATUS.read().tw_sr() == 0 // Check TWI status for readiness
        }
    }
}
