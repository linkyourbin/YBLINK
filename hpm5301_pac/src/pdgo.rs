#[repr(C)]
///Register block
pub struct RegisterBlock {
    dgo_turnoff: DgoTurnoff,
    dgo_rc32k_cfg: DgoRc32kCfg,
    _reserved2: [u8; 0x05f8],
    dgo_gpr00: DgoGpr00,
    dgo_gpr01: DgoGpr01,
    dgo_gpr02: DgoGpr02,
    dgo_gpr03: DgoGpr03,
    _reserved6: [u8; 0xf0],
    dgo_ctr0: DgoCtr0,
    dgo_ctr1: DgoCtr1,
    dgo_ctr2: DgoCtr2,
    dgo_ctr3: DgoCtr3,
    dgo_ctr4: DgoCtr4,
}
impl RegisterBlock {
    ///0x00 - trunoff control
    #[inline(always)]
    pub const fn dgo_turnoff(&self) -> &DgoTurnoff {
        &self.dgo_turnoff
    }
    ///0x04 - RC32K CLOCK
    #[inline(always)]
    pub const fn dgo_rc32k_cfg(&self) -> &DgoRc32kCfg {
        &self.dgo_rc32k_cfg
    }
    ///0x600 - Generic control 0
    #[inline(always)]
    pub const fn dgo_gpr00(&self) -> &DgoGpr00 {
        &self.dgo_gpr00
    }
    ///0x604 - Generic control 1
    #[inline(always)]
    pub const fn dgo_gpr01(&self) -> &DgoGpr01 {
        &self.dgo_gpr01
    }
    ///0x608 - Generic control 2
    #[inline(always)]
    pub const fn dgo_gpr02(&self) -> &DgoGpr02 {
        &self.dgo_gpr02
    }
    ///0x60c - Generic control 3
    #[inline(always)]
    pub const fn dgo_gpr03(&self) -> &DgoGpr03 {
        &self.dgo_gpr03
    }
    ///0x700 - control register 0
    #[inline(always)]
    pub const fn dgo_ctr0(&self) -> &DgoCtr0 {
        &self.dgo_ctr0
    }
    ///0x704 - control register 1
    #[inline(always)]
    pub const fn dgo_ctr1(&self) -> &DgoCtr1 {
        &self.dgo_ctr1
    }
    ///0x708 - control register 2
    #[inline(always)]
    pub const fn dgo_ctr2(&self) -> &DgoCtr2 {
        &self.dgo_ctr2
    }
    ///0x70c - control register 3
    #[inline(always)]
    pub const fn dgo_ctr3(&self) -> &DgoCtr3 {
        &self.dgo_ctr3
    }
    ///0x710 - control register 4
    #[inline(always)]
    pub const fn dgo_ctr4(&self) -> &DgoCtr4 {
        &self.dgo_ctr4
    }
}
/**DGO_TURNOFF (rw) register accessor: trunoff control

You can [`read`](crate::Reg::read) this register and get [`dgo_turnoff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_turnoff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_turnoff`] module*/
#[doc(alias = "DGO_TURNOFF")]
pub type DgoTurnoff = crate::Reg<dgo_turnoff::DgoTurnoffSpec>;
///trunoff control
pub mod dgo_turnoff;
/**DGO_RC32K_CFG (rw) register accessor: RC32K CLOCK

You can [`read`](crate::Reg::read) this register and get [`dgo_rc32k_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_rc32k_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_rc32k_cfg`] module*/
#[doc(alias = "DGO_RC32K_CFG")]
pub type DgoRc32kCfg = crate::Reg<dgo_rc32k_cfg::DgoRc32kCfgSpec>;
///RC32K CLOCK
pub mod dgo_rc32k_cfg;
/**DGO_GPR00 (rw) register accessor: Generic control 0

You can [`read`](crate::Reg::read) this register and get [`dgo_gpr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_gpr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_gpr00`] module*/
#[doc(alias = "DGO_GPR00")]
pub type DgoGpr00 = crate::Reg<dgo_gpr00::DgoGpr00Spec>;
///Generic control 0
pub mod dgo_gpr00;
/**DGO_GPR01 (rw) register accessor: Generic control 1

You can [`read`](crate::Reg::read) this register and get [`dgo_gpr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_gpr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_gpr01`] module*/
#[doc(alias = "DGO_GPR01")]
pub type DgoGpr01 = crate::Reg<dgo_gpr01::DgoGpr01Spec>;
///Generic control 1
pub mod dgo_gpr01;
/**DGO_GPR02 (rw) register accessor: Generic control 2

You can [`read`](crate::Reg::read) this register and get [`dgo_gpr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_gpr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_gpr02`] module*/
#[doc(alias = "DGO_GPR02")]
pub type DgoGpr02 = crate::Reg<dgo_gpr02::DgoGpr02Spec>;
///Generic control 2
pub mod dgo_gpr02;
/**DGO_GPR03 (rw) register accessor: Generic control 3

You can [`read`](crate::Reg::read) this register and get [`dgo_gpr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_gpr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_gpr03`] module*/
#[doc(alias = "DGO_GPR03")]
pub type DgoGpr03 = crate::Reg<dgo_gpr03::DgoGpr03Spec>;
///Generic control 3
pub mod dgo_gpr03;
/**DGO_CTR0 (rw) register accessor: control register 0

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_ctr0`] module*/
#[doc(alias = "DGO_CTR0")]
pub type DgoCtr0 = crate::Reg<dgo_ctr0::DgoCtr0Spec>;
///control register 0
pub mod dgo_ctr0;
/**DGO_CTR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_ctr1`] module*/
#[doc(alias = "DGO_CTR1")]
pub type DgoCtr1 = crate::Reg<dgo_ctr1::DgoCtr1Spec>;
///control register 1
pub mod dgo_ctr1;
/**DGO_CTR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_ctr2`] module*/
#[doc(alias = "DGO_CTR2")]
pub type DgoCtr2 = crate::Reg<dgo_ctr2::DgoCtr2Spec>;
///control register 2
pub mod dgo_ctr2;
/**DGO_CTR3 (rw) register accessor: control register 3

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_ctr3`] module*/
#[doc(alias = "DGO_CTR3")]
pub type DgoCtr3 = crate::Reg<dgo_ctr3::DgoCtr3Spec>;
///control register 3
pub mod dgo_ctr3;
/**DGO_CTR4 (rw) register accessor: control register 4

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dgo_ctr4`] module*/
#[doc(alias = "DGO_CTR4")]
pub type DgoCtr4 = crate::Reg<dgo_ctr4::DgoCtr4Spec>;
///control register 4
pub mod dgo_ctr4;
