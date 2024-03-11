//! Mod to use PCF8574
use crate::Pin;
use embedded_hal::blocking::i2c::{self, Read};

// Yes, I have duplicate the code, cause I find that macro is really hard to read.

/// Struct to manage Pcf8574
pub struct Pcf8574<'a, I>
where
    I: i2c::Write + i2c::Write,
{
    /// I2C driver
    i2c: &'a mut I,
    /// Adress of hardware
    address: u8,
    /// State pint
    pins_state: u8,
}

impl<'a, I> Pcf8574<'a, I>
where
    I: i2c::Write + i2c::Read,
{
    /// Create a new struct to manage Pcf8575
    pub fn new(i2c: &'a mut I, address: u8) -> Self {
        Self {
            i2c,
            address,
            pins_state: 0,
        }
    }

    /// Turn off all pins
    pub fn clear(&mut self) -> Result<(), <I as i2c::Write>::Error> {
        self.i2c.write(self.address, &[0])
    }

    /// Turn on all pins
    pub fn all_on(&mut self) -> Result<(), <I as i2c::Write>::Error> {
        self.i2c.write(self.address, &[0xff])
    }

    /// Send two bytes
    pub fn write(&mut self, value: u8) -> Result<(), <I as i2c::Write>::Error> {
        let r = self.i2c.write(self.address, &[value]);

        if r.is_ok() {
            self.pins_state = value;
        }

        r
    }

    /// Set up pins, don't change other
    pub fn up_pins(&mut self, value: &[Pin]) -> Result<(), <I as i2c::Write>::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            new_value |= *pin as u8
        }

        self.write(new_value)
    }

    /// Set up pins, don't change other
    pub fn down_pins(&mut self, value: &[Pin]) -> Result<(), <I as i2c::Write>::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            new_value &= !(*pin as u8)
        }

        self.write(new_value)
    }

    /// Get state of pins from this struct
    pub fn get_pins_cache(&self) -> u8 {
        self.pins_state
    }

    /// Read pins state from cache. Return true if pin is up
    pub fn get_pin_from_cache(&self, pin: Pin) -> bool {
        self.pins_state & (pin as u8) > 0
    }

    /// Read all pins status. Don't update internal state of this struct
    pub fn read_pins(&mut self) -> Result<u8, <I as Read<u8>>::Error> {
        let mut buffer: [u8; 1] = [0];

        self.i2c.read(self.address, &mut buffer)?;

        self.pins_state = buffer[0];

        Ok(self.pins_state)
    }

    /// Read all pins status. Don't update internal state of this struct.
    /// /// Return true if pin is up.
    pub fn read_pin(&mut self, pin: Pin) -> Result<bool, <I as Read<u8>>::Error> {
        let r = self.read_pins();

        if let Ok(data) = r {
            return Ok((data & (pin as u8)) > 0);
        }

        Err(r.unwrap_err())
    }
}
