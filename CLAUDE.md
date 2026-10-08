# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Bare-metal Rust firmware (`#![no_std]`, `#![no_main]`, nightly — uses `#![feature(c_variadic)]`) for an STM32F429ZI (`thumbv7em-none-eabihf`). It implements steps 0–3 of the "GAP" school project: a host CLI (`gapcli`) talks to the board over UART using the prebuilt C library `lib/libgapcom.a`, the board logs over a second UART, and it reads an MPU6050 gyroscope over I2C.

## Commands

- Build: `cargo build --release` (target is set in `.cargo/config.toml`, so no `--target` flag is needed). The binary is `target/thumbv7em-none-eabihf/release/cp-gap-rust`.
- Test: `cargo test` (runs on the host). The workspace has no default `build.target`; the firmware package sets `forced-target = "thumbv7em-none-eabihf"` (nightly `per-package-target`), so `cargo build` still produces the ARM binary. Hardware-independent code lives in the `crates/mpu60x0` workspace crate and is tested against a mock I2C bus in `crates/mpu60x0/src/tests.rs`. The `MPU` static lives in `src/gyro.rs`.

## Architecture

Data flow, which spans several files:

```
host ─UART7→ UART7 IRQ (interrupts.rs) → gapcom_accept (C lib) → callback (gapcom_callback.rs)
host ←UART7 TX← gapcom_sender.rs ← gapcom_respond_* (C lib)
main loop: utils::gyro_process() → mpu60x0 FIFO → logger → USART1 TX
```

- **C interop:** `build.rs` links `lib/libgapcom.a`. `src/bindings.rs` is the hand-maintained (originally bindgen) FFI declaration of that library. The headers in `Inc/` are reference copies only. Many are duplicated in nested folders, and none are compiled. The library needs `puts`/`printf` symbols, so `utils.rs` exports empty `#[no_mangle]` stubs. Removing them breaks linking. `tinyrlibc` supplies the other libc symbols.
- **Global state:** Everything shared between the main loop and the interrupt handler is a static. `GAPCOM` (an `AtomicPtr`) lives in `interrupts.rs`. `UART7_TX` (an `AtomicPtr`) lives in `gapcom_sender.rs`. `LOGGER` and `gyro::MPU` are `Mutex<RefCell<Option<_>>>` (cortex-m critical sections). `main.rs` initialises them in a fixed order: heap, RTT, peripherals, UART7 TX pointer, logger, gapcom handle and callbacks, then NVIC unmask of UART7, then the gyro.
- **Callbacks:** New host commands are added by writing an `unsafe extern "C"` callback in `gapcom_callback.rs` and registering it in `init_gapcom_callback` with its `GAPCOM_MSG_*_REQ` constant. Each callback must call the matching `gapcom_respond_*`. The callbacks run in interrupt context.
- **UARTs:** Both UART7 (host protocol) and USART1 (logs) are configured in `utils::init_peripherals` at 9600 baud, 9-bit words, even parity. Only the TX halves are kept, and UART7 RX is handled directly in the ISR through the PAC registers.
- **Gyro:** `crates/mpu60x0/` is a generic driver over `embedded_hal` blocking I2C. The gyro is disabled until the host sends `set-gyroscope on`, which calls `enable()`. `gyro_process()` returns silently while it is disabled.
