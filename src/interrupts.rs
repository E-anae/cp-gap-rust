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

    let sr = uart.sr.read();

    if sr.rxne().bit_is_set() || sr.ore().bit_is_set() {
        // Reading DR (after SR) also clears ORE, so it is always read.
        let received = uart.dr.read().bits() as u8;

        // Drop bytes received with an overrun, parity, framing or noise error.
        if sr.ore().bit_is_set() || sr.pe().bit_is_set() || sr.fe().bit_is_set() || sr.nf().bit_is_set() {
            return;
        }

        let gapcom = GAPCOM.load(Ordering::SeqCst);
        if !gapcom.is_null() {
            unsafe {
                crate::bindings::gapcom_accept(gapcom, &received, 1);
            }
        }
    }
}
