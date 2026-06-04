use core::{hint::spin_loop, marker::PhantomData};

use crate::{
    Peri, PeripheralType, pac,
    peripherals::{GPTMR0, GPTMR1},
    sysctl::{self, Resource},
    time::Hertz,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Channel {
    Ch0 = 0,
    Ch1 = 1,
    Ch2 = 2,
    Ch3 = 3,
}

impl Channel {
    pub const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub input_frequency: Hertz,
    pub debug_pause: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_frequency: Hertz(sysctl::OSC24M_HZ),
            debug_pause: true,
        }
    }
}

pub trait Instance: PeripheralType + 'static {
    const RESOURCE: Resource;

    fn regs() -> &'static pac::gptmr0::RegisterBlock;
    fn configure_clock();
}

impl Instance for GPTMR0 {
    const RESOURCE: Resource = Resource::Tmr0;

    fn regs() -> &'static pac::gptmr0::RegisterBlock {
        unsafe { &*pac::Gptmr0::ptr() }
    }

    fn configure_clock() {
        sysctl::configure_gptmr0_clock();
    }
}

impl Instance for GPTMR1 {
    const RESOURCE: Resource = Resource::Tmr1;

    fn regs() -> &'static pac::gptmr0::RegisterBlock {
        unsafe { &*pac::Gptmr1::ptr() }
    }

    fn configure_clock() {
        sysctl::configure_gptmr1_clock_for_driver();
    }
}

pub struct LowLevelTimer<'d, T: Instance> {
    _peri: Peri<'d, T>,
}

impl<'d, T: Instance> LowLevelTimer<'d, T> {
    pub fn new(peri: Peri<'d, T>) -> Self {
        sysctl::enable_resource(T::RESOURCE);
        T::configure_clock();
        Self { _peri: peri }
    }

    pub fn start(&self, channel: Channel) {
        T::regs()
            .channel(channel.index())
            .cr()
            .modify(|_, w| w.cen().set_bit());
    }

    pub fn stop(&self, channel: Channel) {
        T::regs()
            .channel(channel.index())
            .cr()
            .modify(|_, w| w.cen().clear_bit());
    }

    pub fn reset_counter(&self, channel: Channel) {
        let cr = T::regs().channel(channel.index()).cr();
        cr.modify(|_, w| w.cntrst().set_bit());
        cr.modify(|_, w| w.cntrst().clear_bit());
    }

    pub fn counter(&self, channel: Channel) -> u32 {
        T::regs()
            .channel(channel.index())
            .cnt()
            .read()
            .counter()
            .bits()
    }

    pub fn set_counter(&self, channel: Channel, value: u32) {
        let ch = T::regs().channel(channel.index());
        unsafe {
            ch.cntuptval()
                .write(|w| w.cntuptval().bits(hw_count_value(value)));
        }
        ch.cr().modify(|_, w| w.cntupt().set_bit());
    }

    pub fn set_reload(&self, channel: Channel, value: u32) {
        unsafe {
            T::regs()
                .channel(channel.index())
                .rld()
                .write(|w| w.rld().bits(hw_count_value(value)));
        }
    }

    pub fn reload(&self, channel: Channel) -> u32 {
        T::regs()
            .channel(channel.index())
            .rld()
            .read()
            .rld()
            .bits()
            .wrapping_add(1)
    }

    pub fn set_compare(&self, channel: Channel, compare: usize, value: u32) {
        assert!(compare < 2);
        unsafe {
            T::regs()
                .channel(channel.index())
                .cmp(compare)
                .write(|w| w.cmp().bits(hw_count_value(value)));
        }
    }

    pub fn compare(&self, channel: Channel, compare: usize) -> u32 {
        assert!(compare < 2);
        T::regs()
            .channel(channel.index())
            .cmp(compare)
            .read()
            .cmp()
            .bits()
            .wrapping_add(1)
    }

    pub fn enable_compare_output(&self, channel: Channel, enable: bool) {
        T::regs()
            .channel(channel.index())
            .cr()
            .modify(|_, w| w.cmpen().bit(enable));
    }

    pub fn set_compare_initial_high(&self, channel: Channel, high: bool) {
        T::regs()
            .channel(channel.index())
            .cr()
            .modify(|_, w| w.cmpinit().bit(high));
    }
}

pub struct Timer<'d, T: Instance> {
    low: LowLevelTimer<'d, T>,
    channel: Channel,
    input_frequency: Hertz,
    _instance: PhantomData<T>,
}

impl<'d, T: Instance> Timer<'d, T> {
    pub fn new(peri: Peri<'d, T>, channel: Channel, config: Config) -> Self {
        let low = LowLevelTimer::new(peri);
        low.stop(channel);
        low.set_reload(channel, u32::MAX);
        low.set_compare(channel, 0, u32::MAX);
        low.set_compare(channel, 1, u32::MAX);
        T::regs()
            .channel(channel.index())
            .cr()
            .modify(|_, w| w.dbgpause().bit(config.debug_pause));
        low.reset_counter(channel);
        low.start(channel);

        Self {
            low,
            channel,
            input_frequency: config.input_frequency,
            _instance: PhantomData,
        }
    }

    pub fn input_frequency(&self) -> Hertz {
        self.input_frequency
    }

    pub fn now(&self) -> u32 {
        self.low.counter(self.channel)
    }

    pub fn delay_ticks(&mut self, ticks: u32) {
        if ticks == 0 {
            return;
        }

        let start = self.now();
        while self.now().wrapping_sub(start) < ticks {
            spin_loop();
        }
    }

    pub fn delay_ns(&mut self, ns: u64) {
        self.delay_ticks(ticks_from_nanos(ns, self.input_frequency));
    }

    pub fn delay_us(&mut self, us: u64) {
        self.delay_ns(us.saturating_mul(1_000));
    }

    pub fn delay_ms(&mut self, ms: u64) {
        self.delay_ns(ms.saturating_mul(1_000_000));
    }
}

impl<T: Instance> embedded_hal::delay::DelayNs for Timer<'_, T> {
    fn delay_ns(&mut self, ns: u32) {
        Timer::delay_ns(self, ns as u64);
    }
}

fn ticks_from_nanos(ns: u64, frequency: Hertz) -> u32 {
    let ticks = (ns as u128 * frequency.0 as u128).div_ceil(1_000_000_000);
    ticks.min(u32::MAX as u128) as u32
}

const fn hw_count_value(value: u32) -> u32 {
    if value > 0 && value != u32::MAX {
        value - 1
    } else {
        value
    }
}
