//! MCP4725 driver library
//! 
#![no_std]

pub mod dac;

pub use dac::Dac;

use num::cast::AsPrimitive;
use embedded_hal::blocking::i2c::{Read, Write};

#[derive(Debug, Default)]
pub struct MCP4725<I2C> {
    i2c: I2C,
    address: u8,
}

const MCP4725_I2CADDR_DEFAULT: u8 = 0x62;       // Default i2c addr
const MCP4725_CMD_WRITEDAC: u8 = 0x40;          // Write data to the DAC
const MCP4725_CMD_WRITEDACEEPROM: u8 = 0x60;    // Write data to the DAC and EEPROM

impl<I2C, E> MCP4725<I2C>
where
    I2C: Write<Error = E> + Read<Error = E>
{
    pub fn set_and_store_output(&mut self, output: u16) -> Result<(), E> {
        let mut payload = [MCP4725_CMD_WRITEDACEEPROM; 3];

        payload[1] = (output >> 4) as u8;
        payload[2] = ((output & 0x0f) << 4) as u8;
        self.i2c.write(self.address, &payload)
    }

    pub fn get_output(&mut self) -> Result<(bool, bool, u8, u16), E> {
        let mut buffer = [0u8; 4];
        self.i2c.read(self.address, &mut buffer)?;
        let eeprom_status = (buffer[1] & 0x80) != 0;
        let por = (buffer[1] & 0x40) != 0;
        let pds = (buffer[1] >> 1) & 0x03;
        let data: u16 = ((buffer[2] as u16) << 4) | (buffer[3] >> 4) as u16;
        Ok((eeprom_status, por, pds, data))
    }

    pub fn new(i2c: I2C) -> Self {
        Self::new_with_address(i2c, MCP4725_I2CADDR_DEFAULT)
    }

    pub fn new_with_address(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
        }
    }
}

impl<D, I2C, E> Dac<D, E> for MCP4725<I2C>
where
    D: AsPrimitive<u16>,
    I2C: Write<Error = E>,
{
    type CalData = ();

    fn set_value(&mut self, value: D) -> Result<(), E> {
        let mut payload = [MCP4725_CMD_WRITEDAC; 3];

        payload[1] = ((value.as_() >> 4) & 0xff) as u8;
        payload[2] = ((value.as_() << 4) & 0xf0) as u8;
        self.i2c.write(self.address, &payload)
    }
}
