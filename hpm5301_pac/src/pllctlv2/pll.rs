#[repr(C)]
///no description available
#[doc(alias = "PLL")]
pub struct Pll {
    mfi: Mfi,
    mfn: Mfn,
    mfd: Mfd,
    ss_step: SsStep,
    ss_stop: SsStop,
    config: Config,
    locktime: Locktime,
    steptime: Steptime,
    advanced: Advanced,
    _reserved9: [u8; 0x1c],
    div: [Div; 3],
}
impl Pll {
    ///0x00 - PLL0 multiple register
    #[inline(always)]
    pub const fn mfi(&self) -> &Mfi {
        &self.mfi
    }
    ///0x04 - PLL0 fraction numerator register
    #[inline(always)]
    pub const fn mfn(&self) -> &Mfn {
        &self.mfn
    }
    ///0x08 - PLL0 fraction demoninator register
    #[inline(always)]
    pub const fn mfd(&self) -> &Mfd {
        &self.mfd
    }
    ///0x0c - PLL0 spread spectrum step register
    #[inline(always)]
    pub const fn ss_step(&self) -> &SsStep {
        &self.ss_step
    }
    ///0x10 - PLL0 spread spectrum stop register
    #[inline(always)]
    pub const fn ss_stop(&self) -> &SsStop {
        &self.ss_stop
    }
    ///0x14 - PLL0 confguration register
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    ///0x18 - PLL0 lock time register
    #[inline(always)]
    pub const fn locktime(&self) -> &Locktime {
        &self.locktime
    }
    ///0x1c - PLL0 step time register
    #[inline(always)]
    pub const fn steptime(&self) -> &Steptime {
        &self.steptime
    }
    ///0x20 - PLL0 advance configuration register
    #[inline(always)]
    pub const fn advanced(&self) -> &Advanced {
        &self.advanced
    }
    ///0x40..0x4c - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DIVDIV0` register.</div>
    #[inline(always)]
    pub const fn div(&self, n: usize) -> &Div {
        &self.div[n]
    }
    ///Iterator for array of:
    ///0x40..0x4c - no description available
    #[inline(always)]
    pub fn div_iter(&self) -> impl Iterator<Item = &Div> {
        self.div.iter()
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn divdiv0(&self) -> &Div {
        self.div(0)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn divdiv1(&self) -> &Div {
        self.div(1)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn divdiv2(&self) -> &Div {
        self.div(2)
    }
}
/**MFI (rw) register accessor: PLL0 multiple register

You can [`read`](crate::Reg::read) this register and get [`mfi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mfi`] module*/
#[doc(alias = "MFI")]
pub type Mfi = crate::Reg<mfi::MfiSpec>;
///PLL0 multiple register
pub mod mfi;
/**MFN (rw) register accessor: PLL0 fraction numerator register

You can [`read`](crate::Reg::read) this register and get [`mfn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mfn`] module*/
#[doc(alias = "MFN")]
pub type Mfn = crate::Reg<mfn::MfnSpec>;
///PLL0 fraction numerator register
pub mod mfn;
/**MFD (rw) register accessor: PLL0 fraction demoninator register

You can [`read`](crate::Reg::read) this register and get [`mfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mfd`] module*/
#[doc(alias = "MFD")]
pub type Mfd = crate::Reg<mfd::MfdSpec>;
///PLL0 fraction demoninator register
pub mod mfd;
/**SS_STEP (rw) register accessor: PLL0 spread spectrum step register

You can [`read`](crate::Reg::read) this register and get [`ss_step::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_step::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ss_step`] module*/
#[doc(alias = "SS_STEP")]
pub type SsStep = crate::Reg<ss_step::SsStepSpec>;
///PLL0 spread spectrum step register
pub mod ss_step;
/**SS_STOP (rw) register accessor: PLL0 spread spectrum stop register

You can [`read`](crate::Reg::read) this register and get [`ss_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ss_stop`] module*/
#[doc(alias = "SS_STOP")]
pub type SsStop = crate::Reg<ss_stop::SsStopSpec>;
///PLL0 spread spectrum stop register
pub mod ss_stop;
/**CONFIG (rw) register accessor: PLL0 confguration register

You can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
///PLL0 confguration register
pub mod config;
/**LOCKTIME (rw) register accessor: PLL0 lock time register

You can [`read`](crate::Reg::read) this register and get [`locktime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locktime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@locktime`] module*/
#[doc(alias = "LOCKTIME")]
pub type Locktime = crate::Reg<locktime::LocktimeSpec>;
///PLL0 lock time register
pub mod locktime;
/**STEPTIME (rw) register accessor: PLL0 step time register

You can [`read`](crate::Reg::read) this register and get [`steptime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`steptime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@steptime`] module*/
#[doc(alias = "STEPTIME")]
pub type Steptime = crate::Reg<steptime::SteptimeSpec>;
///PLL0 step time register
pub mod steptime;
/**ADVANCED (rw) register accessor: PLL0 advance configuration register

You can [`read`](crate::Reg::read) this register and get [`advanced::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`advanced::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@advanced`] module*/
#[doc(alias = "ADVANCED")]
pub type Advanced = crate::Reg<advanced::AdvancedSpec>;
///PLL0 advance configuration register
pub mod advanced;
/**DIV (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@div`] module*/
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
///no description available
pub mod div;
