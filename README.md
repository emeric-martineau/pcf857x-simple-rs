# Manager of PFC8574 and PFC8575.

This crate is very simple manager of hardware pcf8574 and pcf8575. 

Crate was tested on ESP32 WROOM-32.

See rust documentation.

## Compatibility

| pcf857x-simple | embedded-hal |
|----------------|--------------|
| 0.1.x          | v0.2.x       |
| 0.2.x          | v0.2.x       |

## Example of use

```rust
use std::thread;
use std::time::Duration;

use esp_idf_hal::units::KiloHertz;
use esp_idf_sys as _;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::i2c::config::Config;
use pcf857x_simple::pcf8574::Pcf8574;
use pcf857x_simple::{Pin, PinState, PCF857X_DEFAULT_ADDRESS};

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let scl = peripherals.pins.gpio22;
    let sda = peripherals.pins.gpio21;

    let i2c_config = Config::new()
      .baudrate(KiloHertz(100).into())
      .scl_enable_pullup(true)
      .sda_enable_pullup(true);

    let mut i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();

    let mut expander = Pcf8574::new(&mut i2c_driver, PCF857X_DEFAULT_ADDRESS);

    loop {
      let _ = expander.clear();

      println!("All off");
      thread::sleep(Duration::from_millis(1000));

      let _ = expander.up_pins(&[Pin::P0]).unwrap();
      println!("P0 on");

      thread::sleep(Duration::from_millis(1000));

      let _ = expander.up_pins(&[Pin::P1]).unwrap();
      println!("P0, P1 on");

      thread::sleep(Duration::from_millis(1000));

      let _ = expander.up_pins(&[Pin::P2]).unwrap();
      println!("P0, P1, P2 on");

      thread::sleep(Duration::from_millis(1000));

      let _ = expander.set_pins(&[PinState::Up(Pin::P0), PinState::Down(Pin::P1), PinState::Down(Pin::P2)]);
      println!("P0 = on, P1 = off, P2 = off");

      thread::sleep(Duration::from_millis(1000));
    }
}
```
