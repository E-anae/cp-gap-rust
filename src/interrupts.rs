use core::sync::atomic::{ AtomicPtr, Ordering };
use stm32f4xx_hal::{ pac::UART7, interrupt };

use crate::bindings::gapcom_handle_t;

static GAPCOM: AtomicPtr<gapcom_handle_t> = AtomicPtr::new(core::ptr::null_mut());

pub fn set_gapcom(gapcom: *mut gapcom_handle_t) {
    GAPCOM.store(gapcom, Ordering::SeqCst);
}

#[interrupt]
fn UART7() {
    let uart = unsafe { &*UART7::ptr() };

    if uart.sr.read().rxne().bit_is_set() {
        let received = uart.dr.read().bits() as u8;

        let gapcom = GAPCOM.load(Ordering::SeqCst);
        if !gapcom.is_null() {
            unsafe {
                crate::bindings::gapcom_accept(gapcom, &received, 1);
            }
        }
    }
}
