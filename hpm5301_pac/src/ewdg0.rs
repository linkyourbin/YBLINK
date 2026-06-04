#[repr(C)]
///Register block
pub struct RegisterBlock {
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    ot_int_val: OtIntVal,
    ot_rst_val: OtRstVal,
    wdt_refresh_reg: WdtRefreshReg,
    wdt_status: WdtStatus,
    cfg_prot: CfgProt,
    ref_prot: RefProt,
    wdt_en: WdtEn,
    ref_time: RefTime,
}
impl RegisterBlock {
    ///0x00 - wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    ///0x04 - wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    ///0x08 - wdog timeout interrupt counter value
    #[inline(always)]
    pub const fn ot_int_val(&self) -> &OtIntVal {
        &self.ot_int_val
    }
    ///0x0c - wdog timeout reset counter value
    #[inline(always)]
    pub const fn ot_rst_val(&self) -> &OtRstVal {
        &self.ot_rst_val
    }
    ///0x10 - wdog refresh register
    #[inline(always)]
    pub const fn wdt_refresh_reg(&self) -> &WdtRefreshReg {
        &self.wdt_refresh_reg
    }
    ///0x14 - wdog status register
    #[inline(always)]
    pub const fn wdt_status(&self) -> &WdtStatus {
        &self.wdt_status
    }
    ///0x18 - ctrl register protection register
    #[inline(always)]
    pub const fn cfg_prot(&self) -> &CfgProt {
        &self.cfg_prot
    }
    ///0x1c - refresh protection register
    #[inline(always)]
    pub const fn ref_prot(&self) -> &RefProt {
        &self.ref_prot
    }
    ///0x20 - Wdog enable
    #[inline(always)]
    pub const fn wdt_en(&self) -> &WdtEn {
        &self.wdt_en
    }
    ///0x24 - Refresh period value
    #[inline(always)]
    pub const fn ref_time(&self) -> &RefTime {
        &self.ref_time
    }
}
/**CTRL0 (rw) register accessor: wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits

You can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl0`] module*/
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
///wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits
pub mod ctrl0;
/**CTRL1 (rw) register accessor: wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits

You can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl1`] module*/
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
///wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits
pub mod ctrl1;
/**OT_INT_VAL (rw) register accessor: wdog timeout interrupt counter value

You can [`read`](crate::Reg::read) this register and get [`ot_int_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ot_int_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ot_int_val`] module*/
#[doc(alias = "OT_INT_VAL")]
pub type OtIntVal = crate::Reg<ot_int_val::OtIntValSpec>;
///wdog timeout interrupt counter value
pub mod ot_int_val;
/**OT_RST_VAL (rw) register accessor: wdog timeout reset counter value

You can [`read`](crate::Reg::read) this register and get [`ot_rst_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ot_rst_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ot_rst_val`] module*/
#[doc(alias = "OT_RST_VAL")]
pub type OtRstVal = crate::Reg<ot_rst_val::OtRstValSpec>;
///wdog timeout reset counter value
pub mod ot_rst_val;
/**WDT_REFRESH_REG (rw) register accessor: wdog refresh register

You can [`read`](crate::Reg::read) this register and get [`wdt_refresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_refresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdt_refresh_reg`] module*/
#[doc(alias = "WDT_REFRESH_REG")]
pub type WdtRefreshReg = crate::Reg<wdt_refresh_reg::WdtRefreshRegSpec>;
///wdog refresh register
pub mod wdt_refresh_reg;
/**WDT_STATUS (rw) register accessor: wdog status register

You can [`read`](crate::Reg::read) this register and get [`wdt_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdt_status`] module*/
#[doc(alias = "WDT_STATUS")]
pub type WdtStatus = crate::Reg<wdt_status::WdtStatusSpec>;
///wdog status register
pub mod wdt_status;
/**CFG_PROT (rw) register accessor: ctrl register protection register

You can [`read`](crate::Reg::read) this register and get [`cfg_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfg_prot`] module*/
#[doc(alias = "CFG_PROT")]
pub type CfgProt = crate::Reg<cfg_prot::CfgProtSpec>;
///ctrl register protection register
pub mod cfg_prot;
/**REF_PROT (rw) register accessor: refresh protection register

You can [`read`](crate::Reg::read) this register and get [`ref_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ref_prot`] module*/
#[doc(alias = "REF_PROT")]
pub type RefProt = crate::Reg<ref_prot::RefProtSpec>;
///refresh protection register
pub mod ref_prot;
/**WDT_EN (rw) register accessor: Wdog enable

You can [`read`](crate::Reg::read) this register and get [`wdt_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdt_en`] module*/
#[doc(alias = "WDT_EN")]
pub type WdtEn = crate::Reg<wdt_en::WdtEnSpec>;
///Wdog enable
pub mod wdt_en;
/**REF_TIME (rw) register accessor: Refresh period value

You can [`read`](crate::Reg::read) this register and get [`ref_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ref_time`] module*/
#[doc(alias = "REF_TIME")]
pub type RefTime = crate::Reg<ref_time::RefTimeSpec>;
///Refresh period value
pub mod ref_time;
