use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use mpu60x0::Mpu60x0;
use stm32f4xx_hal::{ i2c::I2c, pac::I2C1, gpio::{ Pin, Input, Floating } };

pub static MPU: Mutex<
    RefCell<
        Option<Mpu60x0<I2c<I2C1, (Pin<Input<Floating>, 'B', 6>, Pin<Input<Floating>, 'B', 7>)>>>
    >
> = Mutex::new(RefCell::new(None));
