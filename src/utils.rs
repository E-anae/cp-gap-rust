use alloc::format;
use stm32f4xx_hal::{
    i2c::{ I2c, Mode },
    pac::{ self, I2C1, UART7, USART1 },
    gpio::{ Pin, Floating, Input },
    prelude::*,
    serial::{ config::Config, Serial, Tx, Event },
};

use crate::{ mpu60x0, logger_instance, Mpu60x0Error, ErrorKind };

pub struct Peripherals {
    pub i2c: I2c<I2C1, (Pin<Input<Floating>, 'B', 6>, Pin<Input<Floating>, 'B', 7>)>,
    pub uart7_tx: Tx<UART7>,
    pub usart1_tx: Tx<USART1>,
}

pub fn init_peripherals() -> Peripherals {
    let device = pac::Peripherals::take().unwrap();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk((84).mhz()).freeze();

    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpiof = device.GPIOF.split();

    let i2c = I2c::new(
        device.I2C1,
        (gpiob.pb6, gpiob.pb7),
        Mode::Fast {
            frequency: (400_000).hz(),
            duty_cycle: stm32f4xx_hal::i2c::DutyCycle::Ratio2to1,
        },
        clocks
    );

    let mut uart7 = Serial::new(
        device.UART7,
        (gpiof.pf7.into_alternate(), gpiof.pf6.into_alternate()),
        Config::default().baudrate((9600).bps()).wordlength_9().parity_even(),
        clocks
    ).expect("UART7 initialization failed");

    uart7.listen(Event::Rxne);

    let (uart7_tx, _) = uart7.split();

    let usart1 = Serial::new(
        device.USART1,
        (gpioa.pa9.into_alternate(), gpioa.pa10.into_alternate()),
        Config::default().baudrate((9600).bps()).wordlength_9().parity_even(),
        clocks
    ).expect("USART1 initialization failed");

    let (usart1_tx, _) = usart1.split();

    Peripherals {
        i2c,
        uart7_tx,
        usart1_tx,
    }
}

pub fn gyro_process() {
    let gyro_result = cortex_m::interrupt::free(|cs| {
        if let Ok(mut gyro) = mpu60x0::MPU.borrow(cs).try_borrow_mut() {
            gyro.as_mut().map(|g| g.read_gyro())
        } else {
            None
        }
    });

    if let Some(result) = gyro_result {
        match result {
            Ok(gyro_data) => {
                logger_instance().info(
                    &format!("X{:08} Y{:08} Z{:08}", gyro_data.x, gyro_data.y, gyro_data.z)
                );
            }
            Err(Mpu60x0Error { kind: ErrorKind::I2cError, .. }) => {
                logger_instance().error("I2C error while reading gyro data");
            }
            Err(_) => {}
        }
    }
}
