use core::{ fmt::Write, sync::atomic::{ AtomicPtr, Ordering } };
use stm32f4xx_hal::{ serial::Tx, pac::UART7 };

use crate::bindings::gapcom_sender_t;

static UART7_TX: AtomicPtr<Tx<UART7>> = AtomicPtr::new(core::ptr::null_mut());

pub fn set_uart7_tx(tx: *mut Tx<UART7>) {
    UART7_TX.store(tx, Ordering::SeqCst);
}

unsafe extern "C" fn sender_open(_self: *mut gapcom_sender_t) -> cty::c_int {
    if UART7_TX.load(Ordering::SeqCst).is_null() {
        return -1;
    }
    0
}

unsafe extern "C" fn sender_close(_self: *mut gapcom_sender_t) -> cty::c_int {
    0
}

unsafe extern "C" fn sender_send(_self: *mut gapcom_sender_t, buf: *const u8, len: usize) -> isize {
    let uart7_tx = UART7_TX.load(Ordering::SeqCst);
    if uart7_tx.is_null() {
        return -1;
    }

    unsafe {
        let tx = &mut *uart7_tx;
        let slice = core::slice::from_raw_parts(buf, len);
        match tx.write_str(str::from_utf8_unchecked(slice)) {
            Ok(_) => {}
            Err(_) => {
                return -1;
            }
        }
    }

    len as isize
}

pub static mut SENDER_IMPL: gapcom_sender_t = gapcom_sender_t {
    open: Some(sender_open),
    close: Some(sender_close),
    send: Some(sender_send),
};
