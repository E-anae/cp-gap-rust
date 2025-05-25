#![no_main]
#![no_std]

extern crate alloc;

use panic_halt as _;
use tinyrlibc as _;
use embedded_alloc::LlffHeap as Heap;

use rtt_target::{ rtt_init_print };
use core::mem::MaybeUninit;
use cortex_m_rt::entry;

use mpu60x0::{ Mpu60x0, error::* };
use gapcom_callback::init_gapcom_callback;
use utils::{ init_peripherals, gyro_process };
use bindings::{ gapcom_create, gapcom_set_sender_impl };
use logger::{ init_logger, logger_instance };
use gapcom_sender::SENDER_IMPL;

mod mpu60x0;
mod bindings;
mod gapcom_callback;
mod utils;
mod interrupts;
mod gapcom_sender;
mod logger;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32768;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe {
        HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    init_heap();
    rtt_init_print!();

    let mut peri = init_peripherals();

    gapcom_sender::set_uart7_tx(&mut peri.uart7_tx as *mut _);

    init_logger(peri.usart1_tx);

    logger_instance().info("System booted");

    unsafe {
        let gapcom = gapcom_create();
        gapcom_set_sender_impl(gapcom, &raw mut SENDER_IMPL as *mut _);

        init_gapcom_callback(gapcom);

        interrupts::set_gapcom(gapcom);

        cortex_m::peripheral::NVIC::unmask(stm32f4xx_hal::pac::Interrupt::UART7);
    }

    let gyro = Mpu60x0::new(peri.i2c);
    cortex_m::interrupt::free(|cs| {
        mpu60x0::MPU.borrow(cs).replace(Some(gyro));
    });

    loop {
        gyro_process();
        logger_instance().debug("Looping...");
    }
}
