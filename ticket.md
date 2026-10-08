# Ticket: Gyro I2C read runs inside a critical section and can drop UART7 host bytes

**Status:** open
**Area:** `src/utils.rs` (`gyro_process`), `src/gyro.rs`, `src/interrupts.rs`, `crates/mpu60x0`
**Severity:** medium (intermittent loss of host commands; hard to notice)

## Problem

`gyro_process()` in `src/utils.rs` wraps the whole gyro read in
`cortex_m::interrupt::free(...)`. `read_gyro()` -> `read_fifo()` performs
**8 blocking I2C transactions** (2 FIFO-count reads + 6 FIFO-data reads) at
400 kHz, with PRIMASK set the entire time, so every interrupt is masked,
including the UART7 RX interrupt.

UART7 runs at 9600 baud with 9-bit words and even parity, which is about one
byte every ~1.2 ms. The ISR in `src/interrupts.rs` reads one byte per
interrupt and has no overrun handling. If the I2C burst takes longer than one
byte time, the next byte overwrites the previous one in `DR` (ORE) and the
host frame is corrupted. The gapcom decoder then silently rejects the
message or desynchronises.

This got worse with the power-save change (commit `7eea2b2`): in power-save
mode the main loop is `sleep(); gyro_process();`, so **every** wake-up
(including each UART7 RX byte, not only the 20 Hz SysTick) triggers a full
gyro read. A host sending a multi-byte command wakes the core on each byte
and starts the long critical section in the middle of the frame.

Also note that an I2C failure or bus hang (blocking HAL call) inside the
critical section would block all interrupts for as long as the HAL waits.

I did not measure the I2C burst duration on hardware, so the size of the
window is an estimate. Please measure it (e.g. toggle a GPIO or use the DWT
cycle counter around `gyro_process`).

## Expected

- The host protocol never loses bytes while the gyro is being polled.
- In power-save mode the gyro is read about once per SysTick (20 Hz), not on
  every unrelated interrupt.

## Suggested fix (the agent may choose another approach)

1. Do not hold PRIMASK across I2C. `MPU` is `Mutex<RefCell<Option<_>>>`, and
   only the main loop touches it for reads, while the `set-gyroscope`
   callback (ISR) calls `enable()`/`disable()`. Options:
   - take the gyro out of the mutex (`Option::take`) inside a short critical
     section, do the I2C outside it, then put it back; or
   - keep the critical section but make the ISR robust instead (see 3).
2. In power-save mode, only call `gyro_process()` when the SysTick fired.
   For example, set an `AtomicBool` in the `SysTick` handler in
   `src/power.rs` and test-and-clear it in `main`.
3. In the UART7 ISR, handle `ORE`, so an overrun clears the flag rather than
   wedging reception.

Be careful with option 1: the callback runs in interrupt context, so a
`take()` must not make `enable()` or `disable()` silently do nothing while
the main loop holds the gyro. Define the behaviour explicitly.

## Acceptance criteria

- [ ] `gyro_process` does not run I2C with interrupts globally masked.
- [ ] In power-save mode the gyro is polled at the SysTick rate only.
- [ ] `set-gyroscope on/off` still takes effect while the main loop is polling.
- [ ] `cargo build --release` succeeds, and `cargo test` passes (add a test
      in `crates/mpu60x0/src/tests.rs` if driver code changes).
- [ ] Hardware check, if possible: send `ping` repeatedly from `gapcli`
      while the gyro is on, and confirm no lost responses.

## Minor cleanup (optional, same area)

- `src/utils.rs:90` and `:95` give unused-variable warnings in the `puts` and
  `printf` stubs. Rename the parameters to `_string` and `_format`. Do **not**
  remove the stubs, because the C library needs them to link.
- `CLAUDE.md` does not mention `src/power.rs` or the power-save command yet.

## Context

- Architecture: see `CLAUDE.md` (data flow and global-state sections).
- Relevant code: `src/main.rs` (main loop), `src/power.rs`, `src/utils.rs`
  (`gyro_process`), `src/interrupts.rs` (UART7 ISR), `src/gyro.rs`,
  `crates/mpu60x0/src/lib.rs` (`read_fifo`).
