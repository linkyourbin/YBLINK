use core::task::Waker;

use embassy_time_driver::Driver;

use crate::time::{self, DEFAULT_TIMER_HZ};

struct Hpm5301TimeDriver;

impl Driver for Hpm5301TimeDriver {
    fn now(&self) -> u64 {
        hardware_ticks_to_embassy_ticks(time::now_ticks())
    }

    fn schedule_wake(&self, _at: u64, waker: &Waker) {
        // The current PAC has no generated interrupt vector table, so this
        // first Embassy bridge wakes the executor for cooperative polling.
        waker.wake_by_ref();
    }
}

static DRIVER: Hpm5301TimeDriver = Hpm5301TimeDriver;

pub(crate) fn init() {}

#[unsafe(no_mangle)]
fn _embassy_time_now() -> u64 {
    DRIVER.now()
}

#[unsafe(no_mangle)]
fn _embassy_time_schedule_wake(at: u64, waker: &Waker) {
    DRIVER.schedule_wake(at, waker);
}

fn hardware_ticks_to_embassy_ticks(ticks: u64) -> u64 {
    ticks.saturating_mul(embassy_time_driver::TICK_HZ) / DEFAULT_TIMER_HZ
}
