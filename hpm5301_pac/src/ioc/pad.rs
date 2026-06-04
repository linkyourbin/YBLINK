#[repr(C)]
///no description available
#[doc(alias = "PAD")]
pub struct Pad {
    func_ctl: FuncCtl,
    pad_ctl: PadCtl,
}
impl Pad {
    ///0x00 - ALT SELECT
    #[inline(always)]
    pub const fn func_ctl(&self) -> &FuncCtl {
        &self.func_ctl
    }
    ///0x04 - PAD SETTINGS
    #[inline(always)]
    pub const fn pad_ctl(&self) -> &PadCtl {
        &self.pad_ctl
    }
}
/**FUNC_CTL (rw) register accessor: ALT SELECT

You can [`read`](crate::Reg::read) this register and get [`func_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_ctl`] module*/
#[doc(alias = "FUNC_CTL")]
pub type FuncCtl = crate::Reg<func_ctl::FuncCtlSpec>;
///ALT SELECT
pub mod func_ctl;
/**PAD_CTL (rw) register accessor: PAD SETTINGS

You can [`read`](crate::Reg::read) this register and get [`pad_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_ctl`] module*/
#[doc(alias = "PAD_CTL")]
pub type PadCtl = crate::Reg<pad_ctl::PadCtlSpec>;
///PAD SETTINGS
pub mod pad_ctl;
