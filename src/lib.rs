#![cfg_attr(not(test), no_std)]

use core::cell::RefCell;

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

#[derive(Debug)]
pub enum Error<I> {
    /// I2C bus error
    I2C(I),
    /// Connection error (device not found)
    Conn,
    /// Address error (invalid or out of bounds)
    Address,
    /// Port error (invalid or out of bounds)
    Port,
}

pub enum DeviceAddress {
    Crow = 0x01,
    Ww = 0x10,
    Ansible = 0x20,
    Mp = 0x30,
    Er301 = 0x31,
    Faderbank = 0x34,
    Matrixarchate = 0x38,
    Tetrapad = 0x3b,
    Orca = 0x40,
    DistingEx = 0x41,
    Es = 0x50,
    TelexO = 0x60,
    TelexI = 0x68,
}

/// Generic device with a shared I2C bus
pub struct Device<'a, I2C> {
    address: u8,
    i2c: &'a RefCell<I2C>,
}

impl<'a, I2C, S> Device<'a, I2C>
where
    I2C: Write<u8, Error = S> + Read<u8, Error = S> + WriteRead<u8, Error = S>,
{
    pub fn new(i2c: &'a RefCell<I2C>, address: u8) -> Self {
        Self { address, i2c }
    }
    pub fn write(&mut self, data: &[u8]) -> Result<(), Error<S>> {
        self.i2c
            .borrow_mut()
            .write(self.address, data)
            .map_err(Error::I2C)?;
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), Error<S>> {
        self.i2c
            .borrow_mut()
            .read(self.address, buf)
            .map_err(Error::I2C)?;
        Ok(())
    }

    pub fn write_read(&mut self, data: &[u8], buf: &mut [u8]) -> Result<(), Error<S>> {
        self.i2c
            .borrow_mut()
            .write_read(self.address, data, buf)
            .map_err(Error::I2C)?;
        Ok(())
    }
}

pub struct Mii<I2C> {
    i2c: RefCell<I2C>,
}

impl<'a, I2C, S> Mii<I2C>
where
    I2C: Write<u8, Error = S> + Read<u8, Error = S> + WriteRead<u8, Error = S>,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c: RefCell::new(i2c),
        }
    }

    pub fn create_device(&'a self, address: u8) -> Device<'a, I2C> {
        Device {
            address,
            i2c: &self.i2c,
        }
    }
}

/// The 16n faderbank
pub struct Faderbank<'a, I2C> {
    device: Device<'a, I2C>,
    fader_buf: [[u8; 2]; 16],
}

impl<'a, I2C, S> Faderbank<'a, I2C>
where
    I2C: Write<u8, Error = S> + Read<u8, Error = S> + WriteRead<u8, Error = S>,
{
    pub fn new(mii: &'a Mii<I2C>) -> Self {
        Self {
            device: mii.create_device(DeviceAddress::Faderbank as u8),
            fader_buf: [[0; 2]; 16],
        }
    }

    pub fn read_fader(&mut self, no: usize) -> Result<u16, Error<S>> {
        self.device
            .write_read(&[no as u8], &mut self.fader_buf[no])?;
        Ok((self.fader_buf[no][0] as u16) << 8 | (self.fader_buf[no][1] as u16) & 0xff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[test]
    fn init() {
        let mut i2c = I2cMock::new([]);
        i2c.expect(&[
            // connection check
            I2cTransaction::write_read(
                DeviceAddress::Faderbank as u8,
                vec![0],
                vec![0, 0],
            ),
        ]);
        let mii = Mii::new(i2c);
        let mut faderbank = Faderbank::new(&mii);
        faderbank.read_fader(0).ok();
    }
}
