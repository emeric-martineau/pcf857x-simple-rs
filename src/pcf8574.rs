//! Mod to use PCF8574
use crate::{Pin, PinState};
use embedded_hal::i2c::I2c;

// Yes, I have duplicate the code, cause I find that macro is really hard to read.

/// Struct to manage Pcf8574
pub struct Pcf8574<I>
where
    I: I2c,
{
    /// I2C driver
    i2c: I,
    /// Adress of hardware
    address: u8,
    /// State pint
    pins_state: u8,
}

impl<I> Pcf8574<I>
where
    I: I2c,
{
    /// Create a new struct to manage Pcf8575
    pub fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            pins_state: 0,
        }
    }

    /// Turn off all pins
    pub fn clear(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0])?;
        self.pins_state = 0;
        Ok(())
    }

    /// Turn on all pins
    pub fn all_on(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0xff])?;
        self.pins_state = 0xff;
        Ok(())
    }

    /// Send two bytes
    pub fn write(&mut self, value: u8) -> Result<(), I::Error> {
        let r = self.i2c.write(self.address, &[value]);

        if r.is_ok() {
            self.pins_state = value;
        }

        r
    }

    /// Set up pins, don't change other
    pub fn up_pins(&mut self, value: &[Pin]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            new_value |= *pin as u8
        }

        self.write(new_value)
    }

    /// Set up pins, don't change other
    pub fn down_pins(&mut self, value: &[Pin]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            new_value &= !(*pin as u8)
        }

        self.write(new_value)
    }

    /// Invert pins
    pub fn toogle_pins(&mut self, value: &[Pin]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for pin in value {
            if self.pins_state & (*pin as u8) > 0 {
                // Pin Up
                new_value &= !(*pin as u8)
            } else {
                // Pin Down
                new_value |= *pin as u8
            }
        }

        self.write(new_value)
    }

    /// Set up don't change other
    pub fn set_pins(&mut self, value: &[PinState]) -> Result<(), I::Error> {
        let mut new_value = self.pins_state;

        for state in value {
            match state {
                PinState::Up(pin) => new_value |= *pin as u8,
                PinState::Down(pin) => new_value &= !(*pin as u8),
            }
        }

        self.write(new_value)
    }

    /// Reset internal pin cache
    pub fn reset_pins_cache(&mut self) {
        self.pins_state = 0;
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
    pub fn read_pins(&mut self) -> Result<u8, I::Error> {
        let mut buffer: [u8; 1] = [0];

        self.i2c.read(self.address, &mut buffer)?;

        self.pins_state = buffer[0];

        Ok(self.pins_state)
    }

    /// Read all pins status. Don't update internal state of this struct.
    /// /// Return true if pin is up.
    pub fn read_pin(&mut self, pin: Pin) -> Result<bool, I::Error> {
        let r = self.read_pins();

        if let Ok(data) = r {
            return Ok((data & (pin as u8)) > 0);
        }

        Err(r.unwrap_err())
    }
}
