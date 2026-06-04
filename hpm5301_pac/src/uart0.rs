#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    idle_cfg: IdleCfg,
    addr_cfg: AddrCfg,
    iir2: Iir2,
    cfg: Cfg,
    oscr: Oscr,
    fcrr: Fcrr,
    moto_cfg: MotoCfg,
    _reserved_7_union_20: [u8; 0x04],
    _reserved_8_union_24: [u8; 0x04],
    _reserved_9_union_28: [u8; 0x04],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    gpr: Gpr,
}
impl RegisterBlock {
    ///0x04 - Idle Configuration Register
    #[inline(always)]
    pub const fn idle_cfg(&self) -> &IdleCfg {
        &self.idle_cfg
    }
    ///0x08 - address match config register
    #[inline(always)]
    pub const fn addr_cfg(&self) -> &AddrCfg {
        &self.addr_cfg
    }
    ///0x0c - Interrupt Identification Register2
    #[inline(always)]
    pub const fn iir2(&self) -> &Iir2 {
        &self.iir2
    }
    ///0x10 - Configuration Register
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    ///0x14 - Over Sample Control Register
    #[inline(always)]
    pub const fn oscr(&self) -> &Oscr {
        &self.oscr
    }
    ///0x18 - FIFO Control Register config
    #[inline(always)]
    pub const fn fcrr(&self) -> &Fcrr {
        &self.fcrr
    }
    ///0x1c - moto system control register
    #[inline(always)]
    pub const fn moto_cfg(&self) -> &MotoCfg {
        &self.moto_cfg
    }
    ///0x20 - Divisor Latch LSB (when DLAB = 1)
    #[inline(always)]
    pub const fn union_20_dll(&self) -> &Union20Dll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Transmitter Holding Register (when DLAB = 0)
    #[inline(always)]
    pub const fn union_20_thr(&self) -> &Union20Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Receiver Buffer Register (when DLAB = 0)
    #[inline(always)]
    pub const fn union_20_rbr(&self) -> &Union20Rbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - Divisor Latch MSB (when DLAB = 1)
    #[inline(always)]
    pub const fn union_24_dlm(&self) -> &Union24Dlm {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - Interrupt Enable Register (when DLAB = 0)
    #[inline(always)]
    pub const fn union_24_ier(&self) -> &Union24Ier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 - FIFO Control Register
    #[inline(always)]
    pub const fn union_28_fcr(&self) -> &Union28Fcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - Interrupt Identification Register
    #[inline(always)]
    pub const fn union_28_iir(&self) -> &Union28Iir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - Line Control Register
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    ///0x30 - Modem Control Register (
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    ///0x34 - Line Status Register
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    ///0x38 - Modem Status Register
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    ///0x3c - GPR Register
    #[inline(always)]
    pub const fn gpr(&self) -> &Gpr {
        &self.gpr
    }
}
/**IDLE_CFG (rw) register accessor: Idle Configuration Register

You can [`read`](crate::Reg::read) this register and get [`idle_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@idle_cfg`] module*/
#[doc(alias = "IDLE_CFG")]
pub type IdleCfg = crate::Reg<idle_cfg::IdleCfgSpec>;
///Idle Configuration Register
pub mod idle_cfg;
/**ADDR_CFG (rw) register accessor: address match config register

You can [`read`](crate::Reg::read) this register and get [`addr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr_cfg`] module*/
#[doc(alias = "ADDR_CFG")]
pub type AddrCfg = crate::Reg<addr_cfg::AddrCfgSpec>;
///address match config register
pub mod addr_cfg;
/**IIR2 (rw) register accessor: Interrupt Identification Register2

You can [`read`](crate::Reg::read) this register and get [`iir2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iir2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iir2`] module*/
#[doc(alias = "IIR2")]
pub type Iir2 = crate::Reg<iir2::Iir2Spec>;
///Interrupt Identification Register2
pub mod iir2;
/**Cfg (rw) register accessor: Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfg`] module*/
pub type Cfg = crate::Reg<cfg::CfgSpec>;
///Configuration Register
pub mod cfg;
/**OSCR (rw) register accessor: Over Sample Control Register

You can [`read`](crate::Reg::read) this register and get [`oscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oscr`] module*/
#[doc(alias = "OSCR")]
pub type Oscr = crate::Reg<oscr::OscrSpec>;
///Over Sample Control Register
pub mod oscr;
/**FCRR (rw) register accessor: FIFO Control Register config

You can [`read`](crate::Reg::read) this register and get [`fcrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcrr`] module*/
#[doc(alias = "FCRR")]
pub type Fcrr = crate::Reg<fcrr::FcrrSpec>;
///FIFO Control Register config
pub mod fcrr;
/**MOTO_CFG (rw) register accessor: moto system control register

You can [`read`](crate::Reg::read) this register and get [`moto_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moto_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@moto_cfg`] module*/
#[doc(alias = "MOTO_CFG")]
pub type MotoCfg = crate::Reg<moto_cfg::MotoCfgSpec>;
///moto system control register
pub mod moto_cfg;
/**UNION_20_RBR (rw) register accessor: Receiver Buffer Register (when DLAB = 0)

You can [`read`](crate::Reg::read) this register and get [`union_20_rbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_20_rbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_20_rbr`] module*/
#[doc(alias = "UNION_20_RBR")]
pub type Union20Rbr = crate::Reg<union_20_rbr::Union20RbrSpec>;
///Receiver Buffer Register (when DLAB = 0)
pub mod union_20_rbr;
/**UNION_20_THR (rw) register accessor: Transmitter Holding Register (when DLAB = 0)

You can [`read`](crate::Reg::read) this register and get [`union_20_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_20_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_20_thr`] module*/
#[doc(alias = "UNION_20_THR")]
pub type Union20Thr = crate::Reg<union_20_thr::Union20ThrSpec>;
///Transmitter Holding Register (when DLAB = 0)
pub mod union_20_thr;
/**UNION_20_DLL (rw) register accessor: Divisor Latch LSB (when DLAB = 1)

You can [`read`](crate::Reg::read) this register and get [`union_20_dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_20_dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_20_dll`] module*/
#[doc(alias = "UNION_20_DLL")]
pub type Union20Dll = crate::Reg<union_20_dll::Union20DllSpec>;
///Divisor Latch LSB (when DLAB = 1)
pub mod union_20_dll;
/**UNION_24_IER (rw) register accessor: Interrupt Enable Register (when DLAB = 0)

You can [`read`](crate::Reg::read) this register and get [`union_24_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_24_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_24_ier`] module*/
#[doc(alias = "UNION_24_IER")]
pub type Union24Ier = crate::Reg<union_24_ier::Union24IerSpec>;
///Interrupt Enable Register (when DLAB = 0)
pub mod union_24_ier;
/**UNION_24_DLM (rw) register accessor: Divisor Latch MSB (when DLAB = 1)

You can [`read`](crate::Reg::read) this register and get [`union_24_dlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_24_dlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_24_dlm`] module*/
#[doc(alias = "UNION_24_DLM")]
pub type Union24Dlm = crate::Reg<union_24_dlm::Union24DlmSpec>;
///Divisor Latch MSB (when DLAB = 1)
pub mod union_24_dlm;
/**UNION_28_IIR (rw) register accessor: Interrupt Identification Register

You can [`read`](crate::Reg::read) this register and get [`union_28_iir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_28_iir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_28_iir`] module*/
#[doc(alias = "UNION_28_IIR")]
pub type Union28Iir = crate::Reg<union_28_iir::Union28IirSpec>;
///Interrupt Identification Register
pub mod union_28_iir;
/**UNION_28_FCR (rw) register accessor: FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`union_28_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_28_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_28_fcr`] module*/
#[doc(alias = "UNION_28_FCR")]
pub type Union28Fcr = crate::Reg<union_28_fcr::Union28FcrSpec>;
///FIFO Control Register
pub mod union_28_fcr;
/**LCR (rw) register accessor: Line Control Register

You can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcr`] module*/
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
///Line Control Register
pub mod lcr;
/**MCR (rw) register accessor: Modem Control Register (

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mcr`] module*/
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
///Modem Control Register (
pub mod mcr;
/**LSR (rw) register accessor: Line Status Register

You can [`read`](crate::Reg::read) this register and get [`lsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lsr`] module*/
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
///Line Status Register
pub mod lsr;
/**MSR (rw) register accessor: Modem Status Register

You can [`read`](crate::Reg::read) this register and get [`msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@msr`] module*/
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
///Modem Status Register
pub mod msr;
/**GPR (rw) register accessor: GPR Register

You can [`read`](crate::Reg::read) this register and get [`gpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpr`] module*/
#[doc(alias = "GPR")]
pub type Gpr = crate::Reg<gpr::GprSpec>;
///GPR Register
pub mod gpr;
