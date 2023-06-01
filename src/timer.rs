//! Global timer for keeping track of the time
//!
//! This global timer is based on the [`TG0_T0`](hal::timer)
//! (timer group 0, timer 0) and works by incresing the
//! [`TIME_COUNTER`] on each interrupt.
//! 
//! The timer counter is reset to a specific value on each
//! interrupt indicated by [`PRECISION`].
//!
//! Before using the timer the [`init()`] function *should* be
//! called.
//! 
//! The [`elapsed_us()`] and [`reset()`] functions can be used
//! to get and modify the value of the [`TIME_COUNTER`].

use core::cell::RefCell;
use core::sync::atomic::{Ordering::Relaxed, AtomicU32};

use critical_section::Mutex;

use hal::{timer::{Timer0, Timer}, peripherals::{TIMG0, Interrupt}, Priority, prelude::*};

/// Global timer object, used to be able to access the timer
/// inside the [`TG0_T0_LEVEL()`] interrupt handler function
/// 
/// The [`init()`] function initializes the value of this global
static TIMER: Mutex<RefCell<Option<Timer<Timer0<TIMG0>>>>> = Mutex::new(RefCell::new(None));

/// Counter to keep track of the time
static TIME_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Increment [`TIME_COUNTER`] every 100µs
const PRECISION: u32 = 100;

/// Timer group 0, timer 0 interrupt handler
#[interrupt]
fn TG0_T0_LEVEL() {
    critical_section::with(|cs| {
        let mut timer = TIMER.borrow_ref_mut(cs);
        let timer = timer.as_mut().unwrap();

        TIME_COUNTER.fetch_add(1, Relaxed);

        if timer.is_interrupt_set() {
            timer.clear_interrupt();
            timer.start(PRECISION.micros());
        }
    });
}

/// Initialize the global timer
/// 
/// The global timer is based on the `TG0_T0` (timer group 0, timer 0)
/// and the initializetion consists in configuring it and enabling the interrupt
pub fn init(mut timer: Timer<Timer0<TIMG0>>) {
    hal::interrupt::enable(Interrupt::TG0_T0_LEVEL, Priority::Priority2).unwrap();
    timer.start(PRECISION.micros());
    timer.listen();
    critical_section::with(|cs| {
        TIMER.borrow_ref_mut(cs).replace(timer);
    });
}

/// Restarts the timer from 0
pub fn reset() {
    TIME_COUNTER.store(0, Relaxed);
}

/// Get the amount of micro-seconds elapsed since the start
/// of the timer.
pub fn elapsed_us() -> u32 {
    TIME_COUNTER.load(Relaxed) * PRECISION
}
