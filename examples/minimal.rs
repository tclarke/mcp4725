//! This example uses the second I2C interface on a stm32f103c8 (blue pill board) to control
//! the MCP4725. The I2C address is not the default but has A0 pull high.
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use mcp4725::{Dac, MCP4725};
use stm32f1xx_hal::{prelude::*, pac, i2c::{BlockingI2c, DutyCycle, Mode}};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let mut flash = device.FLASH.constrain();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&clocks);

    let mut gpiob = device.GPIOB.split();

    let scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);
    let i2c = BlockingI2c::i2c2(
        device.I2C2,
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        1000,
        10,
        1000,
        1000
    );
    let mut dac = MCP4725::new_with_address(i2c, 0x63);

    let mut lvl: u16 = 1;
    loop {
        dac.set_value(lvl).unwrap();
        lvl = (lvl * 2) & 0x0fff;
        delay.delay_ms(100u16);
    }
}
