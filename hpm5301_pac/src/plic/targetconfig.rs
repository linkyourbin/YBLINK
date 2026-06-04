#[repr(C)]
///no description available
#[doc(alias = "TARGETCONFIG")]
pub struct Targetconfig {
    threshold: Threshold,
    claim: Claim,
    _reserved2: [u8; 0x03f8],
    pps: Pps,
}
impl Targetconfig {
    ///0x00 - Target0 priority threshold
    #[inline(always)]
    pub const fn threshold(&self) -> &Threshold {
        &self.threshold
    }
    ///0x04 - Target claim and complete
    #[inline(always)]
    pub const fn claim(&self) -> &Claim {
        &self.claim
    }
    ///0x400 - Preempted priority stack
    #[inline(always)]
    pub const fn pps(&self) -> &Pps {
        &self.pps
    }
}
/**THRESHOLD (rw) register accessor: Target0 priority threshold

You can [`read`](crate::Reg::read) this register and get [`threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@threshold`] module*/
#[doc(alias = "THRESHOLD")]
pub type Threshold = crate::Reg<threshold::ThresholdSpec>;
///Target0 priority threshold
pub mod threshold;
/**CLAIM (rw) register accessor: Target claim and complete

You can [`read`](crate::Reg::read) this register and get [`claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@claim`] module*/
#[doc(alias = "CLAIM")]
pub type Claim = crate::Reg<claim::ClaimSpec>;
///Target claim and complete
pub mod claim;
/**PPS (rw) register accessor: Preempted priority stack

You can [`read`](crate::Reg::read) this register and get [`pps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pps`] module*/
#[doc(alias = "PPS")]
pub type Pps = crate::Reg<pps::PpsSpec>;
///Preempted priority stack
pub mod pps;
