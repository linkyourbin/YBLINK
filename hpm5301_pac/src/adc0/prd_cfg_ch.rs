#[repr(C)]
///no description available
#[doc(alias = "PRD_CFG_CH")]
pub struct PrdCfgCh {
    prd_cfg: PrdCfg,
    prd_thshd_cfg: PrdThshdCfg,
    prd_result: PrdResult,
}
impl PrdCfgCh {
    ///0x00 - No description available
    #[inline(always)]
    pub const fn prd_cfg(&self) -> &PrdCfg {
        &self.prd_cfg
    }
    ///0x04 - No description available
    #[inline(always)]
    pub const fn prd_thshd_cfg(&self) -> &PrdThshdCfg {
        &self.prd_thshd_cfg
    }
    ///0x08 - No description available
    #[inline(always)]
    pub const fn prd_result(&self) -> &PrdResult {
        &self.prd_result
    }
}
/**prd_cfg (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`prd_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prd_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prd_cfg`] module*/
#[doc(alias = "prd_cfg")]
pub type PrdCfg = crate::Reg<prd_cfg::PrdCfgSpec>;
///No description available
pub mod prd_cfg;
/**prd_thshd_cfg (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`prd_thshd_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prd_thshd_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prd_thshd_cfg`] module*/
#[doc(alias = "prd_thshd_cfg")]
pub type PrdThshdCfg = crate::Reg<prd_thshd_cfg::PrdThshdCfgSpec>;
///No description available
pub mod prd_thshd_cfg;
/**prd_result (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`prd_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prd_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prd_result`] module*/
#[doc(alias = "prd_result")]
pub type PrdResult = crate::Reg<prd_result::PrdResultSpec>;
///No description available
pub mod prd_result;
