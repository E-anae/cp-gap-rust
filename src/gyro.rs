use core::sync::atomic::{ AtomicBool, AtomicU8, Ordering };
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use mpu60x0::Mpu60x0;
use stm32f4xx_hal::{ i2c::I2c, pac::I2C1, gpio::{ Pin, Input, Floating } };

pub static MPU: Mutex<
    RefCell<
        Option<Mpu60x0<I2c<I2C1, (Pin<Input<Floating>, 'B', 6>, Pin<Input<Floating>, 'B', 7>)>>>
    >
> = Mutex::new(RefCell::new(None));

// Requested gyro state, written by the set-gyroscope callback (ISR) and applied by the main loop.
// The latest request wins; it is never dropped while the main loop holds the gyro.
pub const REQ_NONE: u8 = 0;
pub const REQ_ENABLE: u8 = 1;
pub const REQ_DISABLE: u8 = 2;

static REQUEST: AtomicU8 = AtomicU8::new(REQ_NONE);
static INSTALLED: AtomicBool = AtomicBool::new(false);

pub fn set_installed() {
    INSTALLED.store(true, Ordering::SeqCst);
}

pub fn is_installed() -> bool {
    INSTALLED.load(Ordering::SeqCst)
}

pub fn request(enable: bool) {
    REQUEST.store(if enable { REQ_ENABLE } else { REQ_DISABLE }, Ordering::SeqCst);
}

pub fn take_request() -> u8 {
    REQUEST.swap(REQ_NONE, Ordering::SeqCst)
}
