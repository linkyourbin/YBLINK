#[repr(C)]
///Register block
pub struct RegisterBlock {
    reset_flag: ResetFlag,
    reset_status: ResetStatus,
    reset_hold: ResetHold,
    reset_enable: ResetEnable,
    reset_type: ResetType,
    _reserved5: [u8; 0x08],
    software_reset: SoftwareReset,
}
impl RegisterBlock {
    ///0x00 - flag indicate reset source
    #[inline(always)]
    pub const fn reset_flag(&self) -> &ResetFlag {
        &self.reset_flag
    }
    ///0x04 - reset source status
    #[inline(always)]
    pub const fn reset_status(&self) -> &ResetStatus {
        &self.reset_status
    }
    ///0x08 - reset hold attribute
    #[inline(always)]
    pub const fn reset_hold(&self) -> &ResetHold {
        &self.reset_hold
    }
    ///0x0c - reset source enable
    #[inline(always)]
    pub const fn reset_enable(&self) -> &ResetEnable {
        &self.reset_enable
    }
    ///0x10 - reset type triggered by reset
    #[inline(always)]
    pub const fn reset_type(&self) -> &ResetType {
        &self.reset_type
    }
    ///0x1c - Software reset counter
    #[inline(always)]
    pub const fn software_reset(&self) -> &SoftwareReset {
        &self.software_reset
    }
}
/**RESET_FLAG (rw) register accessor: flag indicate reset source

You can [`read`](crate::Reg::read) this register and get [`reset_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_flag`] module*/
#[doc(alias = "RESET_FLAG")]
pub type ResetFlag = crate::Reg<reset_flag::ResetFlagSpec>;
///flag indicate reset source
pub mod reset_flag;
/**RESET_STATUS (rw) register accessor: reset source status

You can [`read`](crate::Reg::read) this register and get [`reset_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_status`] module*/
#[doc(alias = "RESET_STATUS")]
pub type ResetStatus = crate::Reg<reset_status::ResetStatusSpec>;
///reset source status
pub mod reset_status;
/**RESET_HOLD (rw) register accessor: reset hold attribute

You can [`read`](crate::Reg::read) this register and get [`reset_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_hold`] module*/
#[doc(alias = "RESET_HOLD")]
pub type ResetHold = crate::Reg<reset_hold::ResetHoldSpec>;
///reset hold attribute
pub mod reset_hold;
/**RESET_ENABLE (rw) register accessor: reset source enable

You can [`read`](crate::Reg::read) this register and get [`reset_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_enable`] module*/
#[doc(alias = "RESET_ENABLE")]
pub type ResetEnable = crate::Reg<reset_enable::ResetEnableSpec>;
///reset source enable
pub mod reset_enable;
/**RESET_TYPE (rw) register accessor: reset type triggered by reset

You can [`read`](crate::Reg::read) this register and get [`reset_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_type`] module*/
#[doc(alias = "RESET_TYPE")]
pub type ResetType = crate::Reg<reset_type::ResetTypeSpec>;
///reset type triggered by reset
pub mod reset_type;
/**SOFTWARE_RESET (rw) register accessor: Software reset counter

You can [`read`](crate::Reg::read) this register and get [`software_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`software_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@software_reset`] module*/
#[doc(alias = "SOFTWARE_RESET")]
pub type SoftwareReset = crate::Reg<software_reset::SoftwareResetSpec>;
///Software reset counter
pub mod software_reset;
