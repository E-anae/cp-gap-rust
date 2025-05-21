#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    delay::{ self, Delay },
    i2c::{ I2c, Instance, Mode },
    pac::{ self, sai::ch::im, GPIOG },
    prelude::*,
};
use stm32f4xx_hal::serial::{ Serial, config::Config };
use core::panic::PanicInfo;
use rtt_target::{ rtt_init_print, rprintln };
use core::fmt::Write;

use mpu60x0::{ Mpu60x0, error::Mpu60x0Error };

mod mpu60x0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk((84).mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpiog = device.GPIOG.split();
    let gpiob = device.GPIOB.split();
    let gpiof = device.GPIOF.split();

    let mut led = gpiog.pg13.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);

    rprintln!("Hello, world!");

    let i2c = I2c::new(
        device.I2C1,
        (gpiob.pb6, gpiob.pb7),
        Mode::Fast {
            frequency: (400_000).hz(),
            duty_cycle: stm32f4xx_hal::i2c::DutyCycle::Ratio2to1,
        },
        clocks
    );

    let mut mpu = Mpu60x0::new(i2c, delay);

    match mpu.init() {
        Ok(_) => rprintln!("MPU60X0 initialized"),
        Err(e) => rprintln!("MPU60X0 initialization failed: {:?}", e),
    }

    let serial = Serial::uart7(
        device.UART7,
        (gpiof.pf7.into_alternate(), gpiof.pf6.into_alternate()),
        Config::default().baudrate((115_200).bps()),
        clocks
    ).unwrap();
    let (mut tx, _rx) = serial.split();

    loop {
        match mpu.read_fifo() {
            Ok(data) => {
                rprintln!(
                    "Gyro: x: {}, y: {}, z: {}",
                    data.gyro_data.x,
                    data.gyro_data.y,
                    data.gyro_data.z
                );
            }
            Err(e) => (),
        }
    }
}
