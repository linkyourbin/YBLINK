#[repr(C)]
///no description available
#[doc(alias = "MONITOR")]
pub struct Monitor {
    control: Control,
    current: Current,
    low_limit: LowLimit,
    high_limit: HighLimit,
}
impl Monitor {
    ///0x00 - Clock measure and monitor control
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    ///0x04 - Clock measure result
    #[inline(always)]
    pub const fn current(&self) -> &Current {
        &self.current
    }
    ///0x08 - Clock lower limit
    #[inline(always)]
    pub const fn low_limit(&self) -> &LowLimit {
        &self.low_limit
    }
    ///0x0c - Clock upper limit
    #[inline(always)]
    pub const fn high_limit(&self) -> &HighLimit {
        &self.high_limit
    }
}
/**control (rw) register accessor: Clock measure and monitor control

You can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@control`] module*/
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
///Clock measure and monitor control
pub mod control;
/**current (rw) register accessor: Clock measure result

You can [`read`](crate::Reg::read) this register and get [`current::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@current`] module*/
#[doc(alias = "current")]
pub type Current = crate::Reg<current::CurrentSpec>;
///Clock measure result
pub mod current;
/**low_limit (rw) register accessor: Clock lower limit

You can [`read`](crate::Reg::read) this register and get [`low_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@low_limit`] module*/
#[doc(alias = "low_limit")]
pub type LowLimit = crate::Reg<low_limit::LowLimitSpec>;
///Clock lower limit
pub mod low_limit;
/**high_limit (rw) register accessor: Clock upper limit

You can [`read`](crate::Reg::read) this register and get [`high_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@high_limit`] module*/
#[doc(alias = "high_limit")]
pub type HighLimit = crate::Reg<high_limit::HighLimitSpec>;
///Clock upper limit
pub mod high_limit;
