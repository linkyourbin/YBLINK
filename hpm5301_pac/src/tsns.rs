#[repr(C)]
///Register block
pub struct RegisterBlock {
    t: T,
    tmax: Tmax,
    tmin: Tmin,
    age: Age,
    status: Status,
    config: Config,
    validity: Validity,
    flag: Flag,
    upper_lim_irq: UpperLimIrq,
    lower_lim_irq: LowerLimIrq,
    upper_lim_rst: UpperLimRst,
    lower_lim_rst: LowerLimRst,
    async_: Async,
    _reserved13: [u8; 0x04],
    advan: Advan,
}
impl RegisterBlock {
    ///0x00 - Temperature
    #[inline(always)]
    pub const fn t(&self) -> &T {
        &self.t
    }
    ///0x04 - Maximum Temperature
    #[inline(always)]
    pub const fn tmax(&self) -> &Tmax {
        &self.tmax
    }
    ///0x08 - Minimum Temperature
    #[inline(always)]
    pub const fn tmin(&self) -> &Tmin {
        &self.tmin
    }
    ///0x0c - Sample age
    #[inline(always)]
    pub const fn age(&self) -> &Age {
        &self.age
    }
    ///0x10 - Status
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x14 - Configuration
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    ///0x18 - Sample validity
    #[inline(always)]
    pub const fn validity(&self) -> &Validity {
        &self.validity
    }
    ///0x1c - Temperature flag
    #[inline(always)]
    pub const fn flag(&self) -> &Flag {
        &self.flag
    }
    ///0x20 - Maximum temperature to interrupt
    #[inline(always)]
    pub const fn upper_lim_irq(&self) -> &UpperLimIrq {
        &self.upper_lim_irq
    }
    ///0x24 - Minimum temperature to interrupt
    #[inline(always)]
    pub const fn lower_lim_irq(&self) -> &LowerLimIrq {
        &self.lower_lim_irq
    }
    ///0x28 - Maximum temperature to reset
    #[inline(always)]
    pub const fn upper_lim_rst(&self) -> &UpperLimRst {
        &self.upper_lim_rst
    }
    ///0x2c - Minimum temperature to reset
    #[inline(always)]
    pub const fn lower_lim_rst(&self) -> &LowerLimRst {
        &self.lower_lim_rst
    }
    ///0x30 - Configuration in asynchronous mode
    #[inline(always)]
    pub const fn async_(&self) -> &Async {
        &self.async_
    }
    ///0x38 - Advance configuration
    #[inline(always)]
    pub const fn advan(&self) -> &Advan {
        &self.advan
    }
}
/**T (rw) register accessor: Temperature

You can [`read`](crate::Reg::read) this register and get [`t::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@t`] module*/
pub type T = crate::Reg<t::TSpec>;
///Temperature
pub mod t;
/**TMAX (rw) register accessor: Maximum Temperature

You can [`read`](crate::Reg::read) this register and get [`tmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmax`] module*/
#[doc(alias = "TMAX")]
pub type Tmax = crate::Reg<tmax::TmaxSpec>;
///Maximum Temperature
pub mod tmax;
/**TMIN (rw) register accessor: Minimum Temperature

You can [`read`](crate::Reg::read) this register and get [`tmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmin`] module*/
#[doc(alias = "TMIN")]
pub type Tmin = crate::Reg<tmin::TminSpec>;
///Minimum Temperature
pub mod tmin;
/**AGE (rw) register accessor: Sample age

You can [`read`](crate::Reg::read) this register and get [`age::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`age::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@age`] module*/
#[doc(alias = "AGE")]
pub type Age = crate::Reg<age::AgeSpec>;
///Sample age
pub mod age;
/**STATUS (rw) register accessor: Status

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
///Status
pub mod status;
/**CONFIG (rw) register accessor: Configuration

You can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
///Configuration
pub mod config;
/**VALIDITY (rw) register accessor: Sample validity

You can [`read`](crate::Reg::read) this register and get [`validity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`validity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@validity`] module*/
#[doc(alias = "VALIDITY")]
pub type Validity = crate::Reg<validity::ValiditySpec>;
///Sample validity
pub mod validity;
/**FLAG (rw) register accessor: Temperature flag

You can [`read`](crate::Reg::read) this register and get [`flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flag`] module*/
#[doc(alias = "FLAG")]
pub type Flag = crate::Reg<flag::FlagSpec>;
///Temperature flag
pub mod flag;
/**UPPER_LIM_IRQ (rw) register accessor: Maximum temperature to interrupt

You can [`read`](crate::Reg::read) this register and get [`upper_lim_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upper_lim_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@upper_lim_irq`] module*/
#[doc(alias = "UPPER_LIM_IRQ")]
pub type UpperLimIrq = crate::Reg<upper_lim_irq::UpperLimIrqSpec>;
///Maximum temperature to interrupt
pub mod upper_lim_irq;
/**LOWER_LIM_IRQ (rw) register accessor: Minimum temperature to interrupt

You can [`read`](crate::Reg::read) this register and get [`lower_lim_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lower_lim_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lower_lim_irq`] module*/
#[doc(alias = "LOWER_LIM_IRQ")]
pub type LowerLimIrq = crate::Reg<lower_lim_irq::LowerLimIrqSpec>;
///Minimum temperature to interrupt
pub mod lower_lim_irq;
/**UPPER_LIM_RST (rw) register accessor: Maximum temperature to reset

You can [`read`](crate::Reg::read) this register and get [`upper_lim_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upper_lim_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@upper_lim_rst`] module*/
#[doc(alias = "UPPER_LIM_RST")]
pub type UpperLimRst = crate::Reg<upper_lim_rst::UpperLimRstSpec>;
///Maximum temperature to reset
pub mod upper_lim_rst;
/**LOWER_LIM_RST (rw) register accessor: Minimum temperature to reset

You can [`read`](crate::Reg::read) this register and get [`lower_lim_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lower_lim_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lower_lim_rst`] module*/
#[doc(alias = "LOWER_LIM_RST")]
pub type LowerLimRst = crate::Reg<lower_lim_rst::LowerLimRstSpec>;
///Minimum temperature to reset
pub mod lower_lim_rst;
/**ASYNC (rw) register accessor: Configuration in asynchronous mode

You can [`read`](crate::Reg::read) this register and get [`async_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@async_`] module*/
#[doc(alias = "ASYNC")]
pub type Async = crate::Reg<async_::AsyncSpec>;
///Configuration in asynchronous mode
pub mod async_;
/**ADVAN (rw) register accessor: Advance configuration

You can [`read`](crate::Reg::read) this register and get [`advan::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`advan::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@advan`] module*/
#[doc(alias = "ADVAN")]
pub type Advan = crate::Reg<advan::AdvanSpec>;
///Advance configuration
pub mod advan;
