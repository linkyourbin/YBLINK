#[repr(C)]
///Register block
pub struct RegisterBlock {
    cr: Cr,
    sr: Sr,
    txreg: Txreg,
    rxreg: Rxreg,
    txwrd: [Txwrd; 1],
    _reserved5: [u8; 0x0c],
    rxwrd: [Rxwrd; 1],
}
impl RegisterBlock {
    ///0x00 - Command Registers
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x04 - Status Registers
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x08 - Transmit word message to other core.
    #[inline(always)]
    pub const fn txreg(&self) -> &Txreg {
        &self.txreg
    }
    ///0x0c - Receive word message from other core.
    #[inline(always)]
    pub const fn rxreg(&self) -> &Rxreg {
        &self.rxreg
    }
    ///0x10 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TXWRDTXFIFO0` register.</div>
    #[inline(always)]
    pub const fn txwrd(&self, n: usize) -> &Txwrd {
        &self.txwrd[n]
    }
    ///Iterator for array of:
    ///0x10 - no description available
    #[inline(always)]
    pub fn txwrd_iter(&self) -> impl Iterator<Item = &Txwrd> {
        self.txwrd.iter()
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn txwrdtxfifo0(&self) -> &Txwrd {
        self.txwrd(0)
    }
    ///0x20 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `RXWRDRXFIFO0` register.</div>
    #[inline(always)]
    pub const fn rxwrd(&self, n: usize) -> &Rxwrd {
        &self.rxwrd[n]
    }
    ///Iterator for array of:
    ///0x20 - no description available
    #[inline(always)]
    pub fn rxwrd_iter(&self) -> impl Iterator<Item = &Rxwrd> {
        self.rxwrd.iter()
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn rxwrdrxfifo0(&self) -> &Rxwrd {
        self.rxwrd(0)
    }
}
/**CR (rw) register accessor: Command Registers

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
///Command Registers
pub mod cr;
/**SR (rw) register accessor: Status Registers

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
///Status Registers
pub mod sr;
/**TXREG (rw) register accessor: Transmit word message to other core.

You can [`read`](crate::Reg::read) this register and get [`txreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txreg`] module*/
#[doc(alias = "TXREG")]
pub type Txreg = crate::Reg<txreg::TxregSpec>;
///Transmit word message to other core.
pub mod txreg;
/**RXREG (rw) register accessor: Receive word message from other core.

You can [`read`](crate::Reg::read) this register and get [`rxreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxreg`] module*/
#[doc(alias = "RXREG")]
pub type Rxreg = crate::Reg<rxreg::RxregSpec>;
///Receive word message from other core.
pub mod rxreg;
/**TXWRD (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`txwrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txwrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txwrd`] module*/
#[doc(alias = "TXWRD")]
pub type Txwrd = crate::Reg<txwrd::TxwrdSpec>;
///no description available
pub mod txwrd;
/**RXWRD (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`rxwrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxwrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxwrd`] module*/
#[doc(alias = "RXWRD")]
pub type Rxwrd = crate::Reg<rxwrd::RxwrdSpec>;
///no description available
pub mod rxwrd;
