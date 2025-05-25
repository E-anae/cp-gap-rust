use core::{ result::Result::{ self, Ok } };
use embedded_hal::blocking::i2c::{ Write, WriteRead };
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32f4xx_hal::{ i2c::I2c, pac::I2C1, gpio::{ Pin, Input, Floating } };

use error::Mpu60x0Error;
use data::{ FifoData, GyroData };
use registers::{
    CONFIG,
    FIFO_COUNT_H,
    FIFO_COUNT_L,
    FIFO_DATA,
    FIFO_EN,
    GYRO_CONFIG,
    I2C_MST_CTRL,
    MPU60X0_ADDRESS,
    PWR_MGMT_1,
    PWR_MGMT_2,
    SMPLRT_DIV,
    USER_CTRL,
    WHO_AM_I,
};

mod registers;
mod data;
pub mod error;

pub static MPU: Mutex<
    RefCell<
        Option<Mpu60x0<I2c<I2C1, (Pin<Input<Floating>, 'B', 6>, Pin<Input<Floating>, 'B', 7>)>>>
    >
> = Mutex::new(RefCell::new(None));

pub struct Mpu60x0<I2C> {
    i2c: I2C,
    up: bool,
}

impl<I2C: Write + WriteRead> Mpu60x0<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Mpu60x0 { i2c, up: false }
    }

    fn write_at_address(&mut self, address: u8, value: u8) -> Result<(), Mpu60x0Error> {
        self.i2c.write(0x68, &[address, value]).map_err(|_| { Mpu60x0Error::i2c_error() })
    }

    fn read_address(&mut self, address: u8) -> Result<u8, Mpu60x0Error> {
        let mut buffer = [0; 1];
        self.i2c
            .write_read(0x68, &[address], &mut buffer)
            .map_err(|_| { Mpu60x0Error::i2c_error() })?;
        Ok(buffer[0])
    }

    pub fn delay_ms(&mut self, ms: u32) {
        cortex_m::asm::delay(840 * ms);
    }

    pub fn ping(&mut self) -> Result<(), Mpu60x0Error> {
        if self.read_address(WHO_AM_I)? != MPU60X0_ADDRESS {
            return Err(Mpu60x0Error::device_not_found());
        }
        Ok(())
    }

    pub fn enable(&mut self) -> Result<(), Mpu60x0Error> {
        if self.up {
            return Err(Mpu60x0Error::new("Device already initialized"));
        }

        self.ping()?;

        // 1. Reset device
        self.write_at_address(PWR_MGMT_1, 0x80)?;
        self.delay_ms(100);

        // 2. Wake up and set clock source
        self.write_at_address(PWR_MGMT_1, 0x01)?;
        self.delay_ms(10);

        // 3. Disable standby
        self.write_at_address(PWR_MGMT_2, 0x00)?;
        self.delay_ms(10);

        // 4. Reset FIFO
        self.write_at_address(USER_CTRL, 0x04)?;
        self.delay_ms(10);

        // 5. Enable FIFO logic
        self.write_at_address(USER_CTRL, 0x40)?;
        self.delay_ms(10);

        // 6. Disable I2C master mode
        self.write_at_address(I2C_MST_CTRL, 0x00)?;
        self.delay_ms(10);

        // 7. Enable gyro to FIFO
        self.write_at_address(FIFO_EN, 0x70)?;
        self.delay_ms(10);

        // 8. Configure sample rate, DLPF, gyro config
        self.write_at_address(SMPLRT_DIV, 0x31)?;
        self.write_at_address(CONFIG, 0x04)?;
        self.write_at_address(GYRO_CONFIG, 0x00)?;

        self.up = true;

        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), Mpu60x0Error> {
        if !self.up {
            return Err(Mpu60x0Error::device_not_initialized());
        }

        self.write_at_address(PWR_MGMT_1, 0x80)?;
        self.up = false;

        Ok(())
    }

    pub fn read_fifo(&mut self) -> Result<FifoData, Mpu60x0Error> {
        let mut buffer = [0; 6];

        let fifo_h = self.read_address(FIFO_COUNT_H)?;
        let fifo_l = self.read_address(FIFO_COUNT_L)?;
        let count = u16::from_be_bytes([fifo_h, fifo_l]);

        if count < 6 {
            return Err(Mpu60x0Error::not_enough_data());
        }

        for i in 0..6 {
            buffer[i] = self.read_address(FIFO_DATA)?;
        }

        Ok(FifoData::from_buffer(buffer))
    }

    pub fn read_gyro(&mut self) -> Result<GyroData, Mpu60x0Error> {
        if !self.up {
            return Err(Mpu60x0Error::device_not_initialized());
        }

        let fifo_data = self.read_fifo()?;

        Ok(fifo_data.gyro_data)
    }
}
