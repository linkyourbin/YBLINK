#[repr(C)]
///Register block
pub struct RegisterBlock {
    mtime: Mtime,
    mtimecmp: Mtimecmp,
}
impl RegisterBlock {
    ///0x00..0x08 - Machine Time
    #[inline(always)]
    pub const fn mtime(&self) -> &Mtime {
        &self.mtime
    }
    ///0x08..0x10 - Machine Time Compare
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &Mtimecmp {
        &self.mtimecmp
    }
}
/**MTIME (rw) register accessor: Machine Time

You can [`read`](crate::Reg::read) this register and get [`mtime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mtime`] module*/
#[doc(alias = "MTIME")]
pub type Mtime = crate::Reg<mtime::MtimeSpec>;
///Machine Time
pub mod mtime;
/**MTIMECMP (rw) register accessor: Machine Time Compare

You can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mtimecmp`] module*/
#[doc(alias = "MTIMECMP")]
pub type Mtimecmp = crate::Reg<mtimecmp::MtimecmpSpec>;
///Machine Time Compare
pub mod mtimecmp;
