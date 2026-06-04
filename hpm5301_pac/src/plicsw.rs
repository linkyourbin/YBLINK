#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    pending: Pending,
    _reserved1: [u8; 0x0ffc],
    inten: Inten,
    _reserved2: [u8; 0x001f_e000],
    claim: Claim,
}
impl RegisterBlock {
    ///0x1000 - Pending status
    #[inline(always)]
    pub const fn pending(&self) -> &Pending {
        &self.pending
    }
    ///0x2000 - Interrupt enable
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    ///0x200004 - Claim and complete.
    #[inline(always)]
    pub const fn claim(&self) -> &Claim {
        &self.claim
    }
}
/**PENDING (rw) register accessor: Pending status

You can [`read`](crate::Reg::read) this register and get [`pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pending`] module*/
#[doc(alias = "PENDING")]
pub type Pending = crate::Reg<pending::PendingSpec>;
///Pending status
pub mod pending;
/**INTEN (rw) register accessor: Interrupt enable

You can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inten`] module*/
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
///Interrupt enable
pub mod inten;
/**CLAIM (rw) register accessor: Claim and complete.

You can [`read`](crate::Reg::read) this register and get [`claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@claim`] module*/
#[doc(alias = "CLAIM")]
pub type Claim = crate::Reg<claim::ClaimSpec>;
///Claim and complete.
pub mod claim;
