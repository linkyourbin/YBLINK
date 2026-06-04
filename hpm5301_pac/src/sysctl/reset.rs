#[repr(C)]
///no description available
#[doc(alias = "RESET")]
pub struct Reset {
    control: Control,
    config: Config,
    _reserved2: [u8; 0x04],
    counter: Counter,
}
impl Reset {
    ///0x00 - Reset Setting
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    ///0x04 - Reset Setting
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    ///0x0c - Reset Setting
    #[inline(always)]
    pub const fn counter(&self) -> &Counter {
        &self.counter
    }
}
/**control (rw) register accessor: Reset Setting

You can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@control`] module*/
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
///Reset Setting
pub mod control;
/**config (rw) register accessor: Reset Setting

You can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
#[doc(alias = "config")]
pub type Config = crate::Reg<config::ConfigSpec>;
///Reset Setting
pub mod config;
/**counter (rw) register accessor: Reset Setting

You can [`read`](crate::Reg::read) this register and get [`counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@counter`] module*/
#[doc(alias = "counter")]
pub type Counter = crate::Reg<counter::CounterSpec>;
///Reset Setting
pub mod counter;
