#[repr(C)]
///no description available
#[doc(alias = "POWER")]
pub struct Power {
    status: Status,
    lf_wait: LfWait,
    _reserved2: [u8; 0x04],
    off_wait: OffWait,
    ret_wait: RetWait,
}
impl Power {
    ///0x00 - Power Setting
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x04 - Power Setting
    #[inline(always)]
    pub const fn lf_wait(&self) -> &LfWait {
        &self.lf_wait
    }
    ///0x0c - Power Setting
    #[inline(always)]
    pub const fn off_wait(&self) -> &OffWait {
        &self.off_wait
    }
    ///0x10 - Power Setting
    #[inline(always)]
    pub const fn ret_wait(&self) -> &RetWait {
        &self.ret_wait
    }
}
/**status (rw) register accessor: Power Setting

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
///Power Setting
pub mod status;
/**lf_wait (rw) register accessor: Power Setting

You can [`read`](crate::Reg::read) this register and get [`lf_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lf_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lf_wait`] module*/
#[doc(alias = "lf_wait")]
pub type LfWait = crate::Reg<lf_wait::LfWaitSpec>;
///Power Setting
pub mod lf_wait;
/**off_wait (rw) register accessor: Power Setting

You can [`read`](crate::Reg::read) this register and get [`off_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`off_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@off_wait`] module*/
#[doc(alias = "off_wait")]
pub type OffWait = crate::Reg<off_wait::OffWaitSpec>;
///Power Setting
pub mod off_wait;
/**ret_wait (rw) register accessor: Power Setting

You can [`read`](crate::Reg::read) this register and get [`ret_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ret_wait`] module*/
#[doc(alias = "ret_wait")]
pub type RetWait = crate::Reg<ret_wait::RetWaitSpec>;
///Power Setting
pub mod ret_wait;
