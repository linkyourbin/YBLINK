#[repr(C)]
///Register block
pub struct RegisterBlock {
    pmic_gpr00: PmicGpr00,
    pmic_gpr01: PmicGpr01,
    pmic_gpr02: PmicGpr02,
    pmic_gpr03: PmicGpr03,
    pmic_gpr04: PmicGpr04,
    pmic_gpr05: PmicGpr05,
    pmic_gpr06: PmicGpr06,
    pmic_gpr07: PmicGpr07,
    pmic_gpr08: PmicGpr08,
    pmic_gpr09: PmicGpr09,
    pmic_gpr10: PmicGpr10,
    pmic_gpr11: PmicGpr11,
    pmic_gpr12: PmicGpr12,
    pmic_gpr13: PmicGpr13,
    pmic_gpr14: PmicGpr14,
    pmic_gpr15: PmicGpr15,
}
impl RegisterBlock {
    ///0x00 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr00(&self) -> &PmicGpr00 {
        &self.pmic_gpr00
    }
    ///0x04 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr01(&self) -> &PmicGpr01 {
        &self.pmic_gpr01
    }
    ///0x08 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr02(&self) -> &PmicGpr02 {
        &self.pmic_gpr02
    }
    ///0x0c - Generic control
    #[inline(always)]
    pub const fn pmic_gpr03(&self) -> &PmicGpr03 {
        &self.pmic_gpr03
    }
    ///0x10 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr04(&self) -> &PmicGpr04 {
        &self.pmic_gpr04
    }
    ///0x14 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr05(&self) -> &PmicGpr05 {
        &self.pmic_gpr05
    }
    ///0x18 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr06(&self) -> &PmicGpr06 {
        &self.pmic_gpr06
    }
    ///0x1c - Generic control
    #[inline(always)]
    pub const fn pmic_gpr07(&self) -> &PmicGpr07 {
        &self.pmic_gpr07
    }
    ///0x20 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr08(&self) -> &PmicGpr08 {
        &self.pmic_gpr08
    }
    ///0x24 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr09(&self) -> &PmicGpr09 {
        &self.pmic_gpr09
    }
    ///0x28 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr10(&self) -> &PmicGpr10 {
        &self.pmic_gpr10
    }
    ///0x2c - Generic control
    #[inline(always)]
    pub const fn pmic_gpr11(&self) -> &PmicGpr11 {
        &self.pmic_gpr11
    }
    ///0x30 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr12(&self) -> &PmicGpr12 {
        &self.pmic_gpr12
    }
    ///0x34 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr13(&self) -> &PmicGpr13 {
        &self.pmic_gpr13
    }
    ///0x38 - Generic control
    #[inline(always)]
    pub const fn pmic_gpr14(&self) -> &PmicGpr14 {
        &self.pmic_gpr14
    }
    ///0x3c - Generic control
    #[inline(always)]
    pub const fn pmic_gpr15(&self) -> &PmicGpr15 {
        &self.pmic_gpr15
    }
}
/**PMIC_GPR00 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr00`] module*/
#[doc(alias = "PMIC_GPR00")]
pub type PmicGpr00 = crate::Reg<pmic_gpr00::PmicGpr00Spec>;
///Generic control
pub mod pmic_gpr00;
/**PMIC_GPR01 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr01`] module*/
#[doc(alias = "PMIC_GPR01")]
pub type PmicGpr01 = crate::Reg<pmic_gpr01::PmicGpr01Spec>;
///Generic control
pub mod pmic_gpr01;
/**PMIC_GPR02 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr02`] module*/
#[doc(alias = "PMIC_GPR02")]
pub type PmicGpr02 = crate::Reg<pmic_gpr02::PmicGpr02Spec>;
///Generic control
pub mod pmic_gpr02;
/**PMIC_GPR03 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr03`] module*/
#[doc(alias = "PMIC_GPR03")]
pub type PmicGpr03 = crate::Reg<pmic_gpr03::PmicGpr03Spec>;
///Generic control
pub mod pmic_gpr03;
/**PMIC_GPR04 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr04`] module*/
#[doc(alias = "PMIC_GPR04")]
pub type PmicGpr04 = crate::Reg<pmic_gpr04::PmicGpr04Spec>;
///Generic control
pub mod pmic_gpr04;
/**PMIC_GPR05 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr05`] module*/
#[doc(alias = "PMIC_GPR05")]
pub type PmicGpr05 = crate::Reg<pmic_gpr05::PmicGpr05Spec>;
///Generic control
pub mod pmic_gpr05;
/**PMIC_GPR06 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr06`] module*/
#[doc(alias = "PMIC_GPR06")]
pub type PmicGpr06 = crate::Reg<pmic_gpr06::PmicGpr06Spec>;
///Generic control
pub mod pmic_gpr06;
/**PMIC_GPR07 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr07`] module*/
#[doc(alias = "PMIC_GPR07")]
pub type PmicGpr07 = crate::Reg<pmic_gpr07::PmicGpr07Spec>;
///Generic control
pub mod pmic_gpr07;
/**PMIC_GPR08 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr08`] module*/
#[doc(alias = "PMIC_GPR08")]
pub type PmicGpr08 = crate::Reg<pmic_gpr08::PmicGpr08Spec>;
///Generic control
pub mod pmic_gpr08;
/**PMIC_GPR09 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr09::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr09::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr09`] module*/
#[doc(alias = "PMIC_GPR09")]
pub type PmicGpr09 = crate::Reg<pmic_gpr09::PmicGpr09Spec>;
///Generic control
pub mod pmic_gpr09;
/**PMIC_GPR10 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr10`] module*/
#[doc(alias = "PMIC_GPR10")]
pub type PmicGpr10 = crate::Reg<pmic_gpr10::PmicGpr10Spec>;
///Generic control
pub mod pmic_gpr10;
/**PMIC_GPR11 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr11`] module*/
#[doc(alias = "PMIC_GPR11")]
pub type PmicGpr11 = crate::Reg<pmic_gpr11::PmicGpr11Spec>;
///Generic control
pub mod pmic_gpr11;
/**PMIC_GPR12 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr12`] module*/
#[doc(alias = "PMIC_GPR12")]
pub type PmicGpr12 = crate::Reg<pmic_gpr12::PmicGpr12Spec>;
///Generic control
pub mod pmic_gpr12;
/**PMIC_GPR13 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr13`] module*/
#[doc(alias = "PMIC_GPR13")]
pub type PmicGpr13 = crate::Reg<pmic_gpr13::PmicGpr13Spec>;
///Generic control
pub mod pmic_gpr13;
/**PMIC_GPR14 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr14`] module*/
#[doc(alias = "PMIC_GPR14")]
pub type PmicGpr14 = crate::Reg<pmic_gpr14::PmicGpr14Spec>;
///Generic control
pub mod pmic_gpr14;
/**PMIC_GPR15 (rw) register accessor: Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmic_gpr15`] module*/
#[doc(alias = "PMIC_GPR15")]
pub type PmicGpr15 = crate::Reg<pmic_gpr15::PmicGpr15Spec>;
///Generic control
pub mod pmic_gpr15;
