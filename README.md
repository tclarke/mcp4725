# MCP4725 &emsp; 
[![crates.io](https://img.shields.io/crates/v/mcp4725)](https://crates.io/crates/mcp4725)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/tclarke/mcp4725)
[![Documentation](https://docs.rs/mcp4725/badge.svg)](https://docs.rs/mcp4725)

## A Rust crate for MCP4725 12-bit DAC with EEPROM Memory over I<sup>2</sup>C 

<https://github.com/tclarke/mcp4725.git>

## [Documentation](https://docs.rs/mcp4725/latest/mcp4725/)


[mcp4725]: https://www.microchip.com/en-us/product/mcp4725

The MCP4725 DAC is a low-power, high accuray, single channel, 12-bit buffered voltage output DAC with non-volatile memory (EEPROM).
It's on-board preciision output amplifier allows it to achieve rail-to-rail analog output swing.
It uses an I2C control interdace and has single-supply operation from 2.7V to 5.5V.


### Features

- uses embedded-hal version 0.2.x
- designed for embedded use (STM32, ESP32-C3, -C6 and -S3, etc.)
- supports any I2C address allowed by the hardware (default is 0x62)
- no_std embedded compatible

  

### License
----

You are free to copy, modify, and distribute this application with attribution under the terms of either

 * Apache License, Version 2.0
   ([LICENSE-Apache-2.0](./LICENSE-Apache-2.0) or <https://opensource.org/licenses/Apache-2.0>)
 * MIT license
   ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
