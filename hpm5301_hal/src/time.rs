use core::{
    future::Future,
    hint::spin_loop,
    pin::Pin,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    task::{Context, Poll},
};

use crate::pac;

pub const DEFAULT_TIMER_HZ: u64 = crate::sysctl::OSC24M_HZ as u64;
pub const DEFAULT_MTIMER_HZ: u64 = DEFAULT_TIMER_HZ;
pub const HALF_SECOND_TICKS: u64 = DEFAULT_TIMER_HZ / 2;

const TIMEBASE_CHANNEL: usize = 0;

static TIMEBASE_STARTED: AtomicBool = AtomicBool::new(false);
static LAST_COUNTER: AtomicU32 = AtomicU32::new(0);
static EPOCH: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hertz(pub u32);

pub(crate) fn init() {
    let gptmr1 = unsafe { pac::Gptmr1::steal() };
    let channel = gptmr1.channel(TIMEBASE_CHANNEL);

    unsafe {
        channel.cr().write(|w| w.dbgpause().set_bit());
        channel.cmpcmp0().write(|w| w.cmp().bits(u32::MAX));
        channel.cmpcmp1().write(|w| w.cmp().bits(u32::MAX));
        channel.rld().write(|w| w.rld().bits(u32::MAX));
        channel.cr().modify(|_, w| w.cntrst().set_bit());
        channel.cr().modify(|_, w| w.cntrst().clear_bit());
        gptmr1.sr().write(|w| w.bits(0x000f));
        gptmr1.irqen().write(|w| w.bits(0));
        channel.cr().modify(|_, w| w.cen().set_bit());
    }

    LAST_COUNTER.store(0, Ordering::Relaxed);
    EPOCH.store(0, Ordering::Relaxed);
    TIMEBASE_STARTED.store(true, Ordering::Release);
}

pub fn now_ticks() -> u64 {
    let counter = now_ticks32();
    extend_counter(counter)
}

pub fn now_ticks32() -> u32 {
    unsafe {
        pac::Gptmr1::steal()
            .channel(TIMEBASE_CHANNEL)
            .cnt()
            .read()
            .counter()
            .bits()
    }
}

pub const fn ticks_from_millis(ms: u64) -> u64 {
    ticks_from_nanos(ms * 1_000_000)
}

pub const fn ticks_from_micros(us: u64) -> u64 {
    ticks_from_nanos(us * 1_000)
}

pub const fn ticks_from_nanos(ns: u64) -> u64 {
    ((ns as u128 * DEFAULT_TIMER_HZ as u128).div_ceil(1_000_000_000)) as u64
}

pub fn delay_ticks(ticks: u64) {
    if ticks == 0 {
        return;
    }

    let start = now_ticks();
    while now_ticks().wrapping_sub(start) < ticks {
        spin_loop();
    }
}

pub fn wait_until(deadline: u64) {
    while (now_ticks().wrapping_sub(deadline) as i64) < 0 {
        spin_loop();
    }
}

pub fn delay_millis(ms: u64) {
    delay_ticks(ticks_from_millis(ms));
}

pub fn delay_micros(us: u64) {
    delay_ticks(ticks_from_micros(us));
}

pub fn delay_nanos(ns: u64) {
    delay_ticks(ticks_from_nanos(ns));
}

pub struct Ticker {
    next: u64,
    period_ticks: u64,
}

impl Ticker {
    pub fn every_ticks(period_ticks: u64) -> Self {
        Self {
            next: now_ticks(),
            period_ticks,
        }
    }

    pub fn every_millis(ms: u64) -> Self {
        Self::every_ticks(ticks_from_millis(ms))
    }

    pub fn every_micros(us: u64) -> Self {
        Self::every_ticks(ticks_from_micros(us))
    }

    pub fn wait_next(&mut self) {
        self.next = self.next.wrapping_add(self.period_ticks);
        wait_until(self.next);
    }
}

pub struct Delay;

impl Delay {
    pub const fn new() -> Self {
        Self
    }

    pub fn delay_ms(&mut self, ms: u64) {
        delay_millis(ms);
    }

    pub fn delay_us(&mut self, us: u64) {
        delay_micros(us);
    }

    pub fn delay_ns(&mut self, ns: u64) {
        delay_nanos(ns);
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new()
    }
}

impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        Delay::delay_ns(self, ns as u64);
    }
}

pub struct Timer {
    deadline: u64,
}

impl Timer {
    pub fn at_ticks(deadline: u64) -> Self {
        Self { deadline }
    }

    pub fn after_ticks(ticks: u64) -> Self {
        Self::at_ticks(now_ticks().wrapping_add(ticks))
    }

    pub fn after_millis(ms: u64) -> Self {
        Self::after_ticks(ticks_from_millis(ms))
    }

    pub fn after_micros(us: u64) -> Self {
        Self::after_ticks(ticks_from_micros(us))
    }

    pub fn after_nanos(ns: u64) -> Self {
        Self::after_ticks(ticks_from_nanos(ns))
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if (now_ticks().wrapping_sub(self.deadline) as i64) >= 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn extend_counter(counter: u32) -> u64 {
    debug_assert!(
        TIMEBASE_STARTED.load(Ordering::Acquire),
        "hpm5301_hal time used before hal::init"
    );

    let last = LAST_COUNTER.swap(counter, Ordering::Relaxed);
    if counter < last {
        EPOCH.fetch_add(1, Ordering::Relaxed);
    }

    ((EPOCH.load(Ordering::Relaxed) as u64) << 32) | counter as u64
}
