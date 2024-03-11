//! A Rust driver for the PCF857x I/O expanders, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver is very simple. You can read/write all pins or individual pins.
//! Driver use a cache to avoid read each time you want know pins status or write.
//!
//! ## Devices
//!
//! Devices consist of 8 or 16 bidirectional ports, I²C-bus interface, with three
//! hardware address inputs and interrupt output.
//!
//! Datasheets:
//! - [PCF8574x](https://www.ti.com/lit/gpn/PCF8574)
//! - [PCF8575](https://www.ti.com/lit/gpn/PCF8575)
//!
//! ## How use-it (ESP32 example)?
//!
//! ### With the default address
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! use esp_idf_hal::peripherals::Peripherals;
//! use esp_idf_hal::i2c::I2cDriver;
//! use esp_idf_hal::i2c::config::Config
//! use pcf857x_simple::pcf8574::Pcf8574;
//! use pcf857x_simple::PCF857X_DEFAULT_ADDRESS;
//!
//! esp_idf_sys::link_patches();
//! esp_idf_svc::log::EspLogger::initialize_default();
//!
//! let peripherals = Peripherals::take().unwrap();
//!
//! let scl = peripherals.pins.gpio22;
//! let sda = peripherals.pins.gpio21;
//!
//! let i2c_config = Config::new()
//!   .baudrate(KiloHertz(100).into())
//!   .scl_enable_pullup(true)
//!   .sda_enable_pullup(true);
//!
//! let i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();
//!
//! let mut expander = Pcf8574::new(i2c_driver, PCF857X_DEFAULT_ADDRESS);
//! ```
//!
//! ### With helper address
//!
//! ```no_run
//! use esp_idf_hal::peripherals::Peripherals;
//! use esp_idf_hal::i2c::I2cDriver;
//! use esp_idf_hal::i2c::config::Config
//! use pcf857x_simple::pcf8574::Pcf8574;
//! use pcf857x_simple::pcf857x_address;
//!
//! esp_idf_sys::link_patches();
//! esp_idf_svc::log::EspLogger::initialize_default();
//!
//! let peripherals = Peripherals::take().unwrap();
//!
//! let scl = peripherals.pins.gpio22;
//! let sda = peripherals.pins.gpio21;
//!
//! let i2c_config = Config::new()
//!   .baudrate(KiloHertz(100).into())
//!   .scl_enable_pullup(true)
//!   .sda_enable_pullup(true);
//!
//! let i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();
//!
//! let mut expander = Pcf8574::new(i2c_driver, pcf857x_address(false, false, false));
//! ```
//!
//! ### With direct address
//!
//! ```no_run
//! use esp_idf_hal::peripherals::Peripherals;
//! use esp_idf_hal::i2c::I2cDriver;
//! use esp_idf_hal::i2c::config::Config
//! use pcf857x_simple::pcf8574::Pcf8574;
//! use pcf857x_simple::pcf857x_address;
//!
//! esp_idf_sys::link_patches();
//! esp_idf_svc::log::EspLogger::initialize_default();
//!
//! let peripherals = Peripherals::take().unwrap();
//!
//! let scl = peripherals.pins.gpio22;
//! let sda = peripherals.pins.gpio21;
//!
//! let i2c_config = Config::new()
//!   .baudrate(KiloHertz(100).into())
//!   .scl_enable_pullup(true)
//!   .sda_enable_pullup(true);
//!
//! let i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();
//!
//! let mut expander = Pcf8574::new(i2c_driver, 0x24);
//! ```
//!
//! ### Set P0 and P7 high
//!
//! ```no_run
//! use pcf857x_simple::Pin;
//! ...
//! let _ = expander.clear();
//! let _ = expander.up_pins(&[Pin::P0, Pin::P7]);
//! ```
//!
//! ### Read P0 and P7 high direct from device
//!
//! ```no_run
//! use pcf857x_simple::Pin;
//! ...
//! let _ = expander.clear();
//! let _ = expander.read_pin(Pin::P0);
//! let _ = expander.read_pin(Pin::P7);
//! ```
//!
//! ### Read P0 and P7 high direct from cache
//!
//! ```no_run
//! use pcf857x_simple::Pin;
//! ...
//! let _ = expander.clear();
//! let _ = expander.get_pin_from_cache(Pin::P0);
//! let _ = expander.get_pin_from_cache(Pin::P7);
//! ```
#![no_std]

pub mod pcf8574;
pub mod pcf8575;

/// Pin of Pcf857x
#[derive(Copy, Clone)]
pub enum Pin {
    /// Pin 0
    P0 = 1,
    /// Pin 1
    P1 = 2,
    /// Pin 2
    P2 = 4,
    /// Pin 3
    P3 = 8,
    /// Pin 4
    P4 = 16,
    /// Pin 5
    P5 = 32,
    /// Pin 6
    P6 = 64,
    /// Pin 7
    P7 = 128,
    /// Pin 10 (only PCF8575)
    P10 = 256,
    /// Pin 11 (only PCF8575)
    P11 = 512,
    /// Pin 12 (only PCF8575)
    P12 = 1024,
    /// Pin 13 (only PCF8575)
    P13 = 2048,
    /// Pin 14 (only PCF8575)
    P14 = 4096,
    /// Pin 15 (only PCF8575)
    P15 = 8192,
    /// Pin 16 (only PCF8575)
    P16 = 16384,
    /// Pin 17 (only PCF8575)
    P17 = 32768,
}

/// Defaut base address of device
pub const PCF857X_DEFAULT_ADDRESS: u8 = 0x20;

/// Helper to set address depending of address' pin's device
pub fn pcf857x_address(a0: bool, a1: bool, a2: bool) -> u8 {
    let mut addr: u8 = PCF857X_DEFAULT_ADDRESS;

    if a0 {
        addr += 1;
    }

    if a1 {
        addr += 2;
    }

    if a2 {
        addr += 4;
    }

    addr
}
