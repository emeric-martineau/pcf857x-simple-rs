//! Mod to use PCF8575
use crate::{Pin, PinState};
use embedded_hal::i2c::I2c;

/// Struct to manage Pcf8575
pub struct Pcf8575<'a, I>
where
    I: I2c,
{
    /// I2C driver
    i2c: &'a mut I,
    /// Adress of hardware
    address: u8,
    /// State pint
    pins_state: u16,
}

impl<'a, I> Pcf8575<'a, I>
where
    I: I2c,
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
    pub fn clear(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0, 0])?;
        self.pins_state = 0;
        Ok(())
    }

    /// Turn on all pins
    pub fn all_on(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0xff, 0xff])?;
        self.pins_state = 0xffff;
        Ok(())
    }

    /// Send two bytes
    pub fn write(&mut self, value: u16) -> Result<(), I::Error> {
        let r = self.i2c.write(
            self.address,
            &[((value & 0xff00) >> 8) as u8, (value & 0xff) as u8],
        );

        if r.is_ok() {
            self.pins_state = value;
        }

        r
    }

    /// Set up pins, don't change other
    pub fn up_pins(&mut self, value: &[Pin]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            new_value |= *pin as u16
        }

        self.write(new_value)
    }

    /// Set up pins, don't change other
    pub fn down_pins(&mut self, value: &[Pin]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            new_value &= !(*pin as u16)
        }

        self.write(new_value)
    }

    /// Invert pins
    pub fn toogle_pins(&mut self, value: &[Pin]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            if self.pins_state & (*pin as u16) > 0 {
                // Pin Up
                new_value &= !(*pin as u16)
            } else {
                // Pin Down
                new_value |= *pin as u16
            }
        }

        self.write(new_value)
    }

    /// Set up don't change other
    pub fn set_pins(&mut self, value: &[PinState]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for state in value {
            match state {
                PinState::Up(pin) => new_value |= *pin as u16,
                PinState::Down(pin) => new_value &= !(*pin as u16),
            }
        }

        self.write(new_value)
    }

    /// Reset internal pin cache
    pub fn reset_pins_cache(&mut self) {
        self.pins_state = 0;
    }

    /// Get state of pins from this struct
    pub fn get_pins_cache(&self) -> u16 {
        self.pins_state
    }

    /// Read pins state from cache. Return true if pin is up
    pub fn get_pin_from_cache(&self, pin: Pin) -> bool {
        self.pins_state & (pin as u16) > 0
    }

    /// Read all pins status. Don't update internal state of this struct
    pub fn read_pins(&mut self) -> Result<u16, I::Error> {
        let mut buffer: [u8; 2] = [0, 0];

        self.i2c.read(self.address, &mut buffer)?;

        self.pins_state = ((buffer[1] as u16) << 8) | (buffer[0] as u16);

        Ok(self.pins_state)
    }

    /// Read all pins status. Return true if pin is up
    pub fn read_pin(&mut self, pin: Pin) -> Result<bool, I::Error> {
        let r = self.read_pins();

        if let Ok(data) = r {
            return Ok(data & (pin as u16) > 0);
        }

        Err(r.unwrap_err())
    }
}
