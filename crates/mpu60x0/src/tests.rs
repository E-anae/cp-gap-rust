use std::collections::{ HashMap, VecDeque };

use embedded_hal::blocking::i2c::{ Write, WriteRead };

use super::error::ErrorKind;
use super::registers::*;
use super::Mpu60x0;

/// Fake I2C bus emulating an MPU6050: a register map plus a FIFO that is
/// drained one byte per `FIFO_DATA` read.
#[derive(Default)]
struct MockI2c {
    registers: HashMap<u8, u8>,
    fifo: VecDeque<u8>,
    /// Every `(register, value)` written, in order.
    writes: Vec<(u8, u8)>,
    fail: bool,
}

impl MockI2c {
    fn with_who_am_i(value: u8) -> Self {
        let mut mock = Self::default();
        mock.registers.insert(WHO_AM_I, value);
        mock
    }

    fn present() -> Self {
        Self::with_who_am_i(MPU60X0_ADDRESS)
    }

    fn push_sample(&mut self, x: i16, y: i16, z: i16) {
        for v in [x, y, z] {
            self.fifo.extend(v.to_be_bytes());
        }
    }
}

impl Write for MockI2c {
    type Error = ();

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), ()> {
        assert_eq!(address, 0x68);
        if self.fail {
            return Err(());
        }
        assert_eq!(bytes.len(), 2, "register writes are [reg, value]");
        self.writes.push((bytes[0], bytes[1]));
        self.registers.insert(bytes[0], bytes[1]);
        Ok(())
    }
}

impl WriteRead for MockI2c {
    type Error = ();

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), ()> {
        assert_eq!(address, 0x68);
        if self.fail {
            return Err(());
        }
        assert_eq!(bytes.len(), 1);
        assert_eq!(buffer.len(), 1);
        buffer[0] = match bytes[0] {
            FIFO_COUNT_H => ((self.fifo.len() >> 8) & 0xff) as u8,
            FIFO_COUNT_L => (self.fifo.len() & 0xff) as u8,
            FIFO_DATA => self.fifo.pop_front().unwrap_or(0),
            reg => *self.registers.get(&reg).unwrap_or(&0),
        };
        Ok(())
    }
}

fn enabled_driver() -> Mpu60x0<MockI2c> {
    let mut mpu = Mpu60x0::new(MockI2c::present());
    mpu.enable().unwrap();
    mpu
}

// ---- ping ----

#[test]
fn ping_succeeds_when_who_am_i_matches() {
    let mut mpu = Mpu60x0::new(MockI2c::present());
    assert!(mpu.ping().is_ok());
}

#[test]
fn ping_reports_device_not_found_on_wrong_id() {
    let mut mpu = Mpu60x0::new(MockI2c::with_who_am_i(0x00));
    assert_eq!(mpu.ping().unwrap_err().kind, ErrorKind::DeviceNotFound);
}

#[test]
fn ping_maps_bus_failure_to_i2c_error() {
    let mut i2c = MockI2c::present();
    i2c.fail = true;
    let mut mpu = Mpu60x0::new(i2c);
    assert_eq!(mpu.ping().unwrap_err().kind, ErrorKind::I2cError);
}

// ---- enable / disable ----

#[test]
fn enable_runs_init_sequence_in_order() {
    let mpu = enabled_driver();
    assert_eq!(
        mpu.i2c.writes,
        vec![
            (PWR_MGMT_1, 0x80),
            (PWR_MGMT_1, 0x01),
            (PWR_MGMT_2, 0x00),
            (USER_CTRL, 0x04),
            (USER_CTRL, 0x40),
            (I2C_MST_CTRL, 0x00),
            (FIFO_EN, 0x70),
            (SMPLRT_DIV, 0x31),
            (CONFIG, 0x04),
            (GYRO_CONFIG, 0x00)
        ]
    );
    assert!(mpu.up);
}

#[test]
fn enable_twice_fails_without_touching_the_bus() {
    let mut mpu = enabled_driver();
    let writes_before = mpu.i2c.writes.len();
    assert_eq!(mpu.enable().unwrap_err().kind, ErrorKind::CustomError);
    assert_eq!(mpu.i2c.writes.len(), writes_before);
}

#[test]
fn enable_fails_and_stays_down_when_device_missing() {
    let mut mpu = Mpu60x0::new(MockI2c::with_who_am_i(0x42));
    assert_eq!(mpu.enable().unwrap_err().kind, ErrorKind::DeviceNotFound);
    assert!(!mpu.up);
    assert!(mpu.i2c.writes.is_empty());
}

#[test]
fn disable_resets_device_and_allows_re_enable() {
    let mut mpu = enabled_driver();
    mpu.i2c.writes.clear();
    mpu.disable().unwrap();
    assert_eq!(mpu.i2c.writes, vec![(PWR_MGMT_1, 0x80)]);
    assert!(!mpu.up);
    assert!(mpu.enable().is_ok());
}

#[test]
fn disable_before_enable_fails() {
    let mut mpu = Mpu60x0::new(MockI2c::present());
    assert_eq!(mpu.disable().unwrap_err().kind, ErrorKind::DeviceNotInitialized);
}

// ---- reading ----

#[test]
fn read_gyro_requires_enable() {
    let mut i2c = MockI2c::present();
    i2c.push_sample(1, 2, 3);
    let mut mpu = Mpu60x0::new(i2c);
    assert_eq!(mpu.read_gyro().unwrap_err().kind, ErrorKind::DeviceNotInitialized);
}

#[test]
fn read_gyro_decodes_big_endian_signed_samples() {
    let mut mpu = enabled_driver();
    mpu.i2c.push_sample(0x0102, -2, i16::MIN);
    let g = mpu.read_gyro().unwrap();
    assert_eq!((g.x, g.y, g.z), (0x0102, -2, i16::MIN));
}

#[test]
fn read_gyro_consumes_one_sample_at_a_time() {
    let mut mpu = enabled_driver();
    mpu.i2c.push_sample(1, 2, 3);
    mpu.i2c.push_sample(4, 5, 6);

    let first = mpu.read_gyro().unwrap();
    assert_eq!((first.x, first.y, first.z), (1, 2, 3));
    assert_eq!(mpu.i2c.fifo.len(), 6);

    let second = mpu.read_gyro().unwrap();
    assert_eq!((second.x, second.y, second.z), (4, 5, 6));
    assert!(mpu.i2c.fifo.is_empty());
}

#[test]
fn read_fifo_reports_not_enough_data_for_partial_sample() {
    let mut mpu = enabled_driver();
    mpu.i2c.fifo.extend([1, 2, 3, 4, 5]);
    assert_eq!(mpu.read_fifo().unwrap_err().kind, ErrorKind::NotEnoughData);
    assert_eq!(mpu.i2c.fifo.len(), 5, "partial data must not be consumed");
}

#[test]
fn read_fifo_handles_counts_above_255() {
    let mut mpu = enabled_driver();
    for _ in 0..50 {
        mpu.i2c.push_sample(7, 8, 9); // 300 bytes: high count byte is non-zero
    }
    let g = mpu.read_fifo().unwrap().gyro_data;
    assert_eq!((g.x, g.y, g.z), (7, 8, 9));
}

#[test]
fn read_gyro_propagates_bus_failure() {
    let mut mpu = enabled_driver();
    mpu.i2c.fail = true;
    assert_eq!(mpu.read_gyro().unwrap_err().kind, ErrorKind::I2cError);
}

// ---- data / error types ----

#[test]
fn fifo_data_from_buffer_parses_axes() {
    let d = super::data::FifoData::from_buffer([0x00, 0x01, 0xff, 0xff, 0x80, 0x00]);
    assert_eq!((d.gyro_data.x, d.gyro_data.y, d.gyro_data.z), (1, -1, i16::MIN));
}

#[test]
fn error_display_includes_kind_and_message() {
    let err = super::error::Mpu60x0Error::device_not_found();
    assert_eq!(format!("{err}"), "[DeviceNotFound] Device not found");
}
