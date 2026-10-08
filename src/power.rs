use core::sync::atomic::{ AtomicBool, Ordering };
use cortex_m::peripheral::{ syst::SystClkSource, SYST };
use cortex_m_rt::exception;

pub const SYSCLK_HZ: u32 = 84_000_000;
// Matches the gyroscope sample rate (20 Hz).
const TICK_HZ: u32 = 20;

// Power save is on by default.
static POWER_SAVE: AtomicBool = AtomicBool::new(true);

// Set by the SysTick handler, consumed by the main loop.
static TICK: AtomicBool = AtomicBool::new(false);

/// Returns true once per SysTick, clearing the flag.
pub fn take_tick() -> bool {
    TICK.swap(false, Ordering::SeqCst)
}

pub fn set_requested(enabled: bool) {
    POWER_SAVE.store(enabled, Ordering::SeqCst);
}

pub fn requested() -> bool {
    POWER_SAVE.load(Ordering::SeqCst)
}

/// Starts or stops the SysTick that wakes the core from `wfi`.
pub fn apply(syst: &mut SYST, enabled: bool) {
    if enabled {
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(SYSCLK_HZ / TICK_HZ - 1);
        syst.clear_current();
        syst.enable_interrupt();
        syst.enable_counter();
    } else {
        syst.disable_counter();
        syst.disable_interrupt();
    }
}

/// Sleeps until the next interrupt, unless power save was turned off meanwhile.
pub fn sleep() {
    cortex_m::interrupt::free(|_| {
        if requested() {
            // A pending interrupt still wakes `wfi` while PRIMASK is set.
            cortex_m::asm::wfi();
        }
    });
}

// Wakes the core from `wfi` and flags that a gyro sample is due.
#[exception]
fn SysTick() {
    TICK.store(true, Ordering::SeqCst);
}
