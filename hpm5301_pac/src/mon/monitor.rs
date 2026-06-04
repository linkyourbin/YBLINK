#[repr(C)]
///no description available
#[doc(alias = "MONITOR")]
pub struct Monitor {
    control: Control,
    status: Status,
}
impl Monitor {
    ///0x00 - Glitch and clock monitor control
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    ///0x04 - Glitch and clock monitor status
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
/**CONTROL (rw) register accessor: Glitch and clock monitor control

You can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@control`] module*/
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
///Glitch and clock monitor control
pub mod control;
/**STATUS (rw) register accessor: Glitch and clock monitor status

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
///Glitch and clock monitor status
pub mod status;
