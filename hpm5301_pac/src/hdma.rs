#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    idmisc: Idmisc,
    _reserved1: [u8; 0x08],
    dmacfg: Dmacfg,
    dmactrl: Dmactrl,
    ch_abort: ChAbort,
    _reserved4: [u8; 0x08],
    inthalfsts: Inthalfsts,
    inttcsts: Inttcsts,
    intabortsts: Intabortsts,
    interrsts: Interrsts,
    ch_en: ChEn,
    _reserved9: [u8; 0x08],
    chctrl: (),
}
impl RegisterBlock {
    ///0x04 - ID Misc
    #[inline(always)]
    pub const fn idmisc(&self) -> &Idmisc {
        &self.idmisc
    }
    ///0x10 - DMAC Configuration Register
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    ///0x14 - DMAC Control Register
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    ///0x18 - Channel Abort Register
    #[inline(always)]
    pub const fn ch_abort(&self) -> &ChAbort {
        &self.ch_abort
    }
    ///0x24 - Harlf Complete Interrupt Status
    #[inline(always)]
    pub const fn inthalfsts(&self) -> &Inthalfsts {
        &self.inthalfsts
    }
    ///0x28 - Trans Complete Interrupt Status Register
    #[inline(always)]
    pub const fn inttcsts(&self) -> &Inttcsts {
        &self.inttcsts
    }
    ///0x2c - Abort Interrupt Status Register
    #[inline(always)]
    pub const fn intabortsts(&self) -> &Intabortsts {
        &self.intabortsts
    }
    ///0x30 - Error Interrupt Status Register
    #[inline(always)]
    pub const fn interrsts(&self) -> &Interrsts {
        &self.interrsts
    }
    ///0x34 - Channel Enable Register
    #[inline(always)]
    pub const fn ch_en(&self) -> &ChEn {
        &self.ch_en
    }
    ///0x40..0x3c0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CHCTRLch0` cluster.</div>
    #[inline(always)]
    pub const fn chctrl(&self, n: usize) -> &Chctrl {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x40..0x3c0 - no description available
    #[inline(always)]
    pub fn chctrl_iter(&self) -> impl Iterator<Item = &Chctrl> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(32 * n)
                .cast()
        })
    }
    ///0x40..0x5c - no description available
    #[inline(always)]
    pub const fn chctrlch0(&self) -> &Chctrl {
        self.chctrl(0)
    }
    ///0x60..0x7c - no description available
    #[inline(always)]
    pub const fn chctrlch1(&self) -> &Chctrl {
        self.chctrl(1)
    }
    ///0x80..0x9c - no description available
    #[inline(always)]
    pub const fn chctrlch2(&self) -> &Chctrl {
        self.chctrl(2)
    }
    ///0xa0..0xbc - no description available
    #[inline(always)]
    pub const fn chctrlch3(&self) -> &Chctrl {
        self.chctrl(3)
    }
    ///0xc0..0xdc - no description available
    #[inline(always)]
    pub const fn chctrlch4(&self) -> &Chctrl {
        self.chctrl(4)
    }
    ///0xe0..0xfc - no description available
    #[inline(always)]
    pub const fn chctrlch5(&self) -> &Chctrl {
        self.chctrl(5)
    }
    ///0x100..0x11c - no description available
    #[inline(always)]
    pub const fn chctrlch6(&self) -> &Chctrl {
        self.chctrl(6)
    }
    ///0x120..0x13c - no description available
    #[inline(always)]
    pub const fn chctrlch7(&self) -> &Chctrl {
        self.chctrl(7)
    }
    ///0x140..0x15c - no description available
    #[inline(always)]
    pub const fn chctrlch8(&self) -> &Chctrl {
        self.chctrl(8)
    }
    ///0x160..0x17c - no description available
    #[inline(always)]
    pub const fn chctrlch9(&self) -> &Chctrl {
        self.chctrl(9)
    }
    ///0x180..0x19c - no description available
    #[inline(always)]
    pub const fn chctrlch10(&self) -> &Chctrl {
        self.chctrl(10)
    }
    ///0x1a0..0x1bc - no description available
    #[inline(always)]
    pub const fn chctrlch11(&self) -> &Chctrl {
        self.chctrl(11)
    }
    ///0x1c0..0x1dc - no description available
    #[inline(always)]
    pub const fn chctrlch12(&self) -> &Chctrl {
        self.chctrl(12)
    }
    ///0x1e0..0x1fc - no description available
    #[inline(always)]
    pub const fn chctrlch13(&self) -> &Chctrl {
        self.chctrl(13)
    }
    ///0x200..0x21c - no description available
    #[inline(always)]
    pub const fn chctrlch14(&self) -> &Chctrl {
        self.chctrl(14)
    }
    ///0x220..0x23c - no description available
    #[inline(always)]
    pub const fn chctrlch15(&self) -> &Chctrl {
        self.chctrl(15)
    }
    ///0x240..0x25c - no description available
    #[inline(always)]
    pub const fn chctrlch16(&self) -> &Chctrl {
        self.chctrl(16)
    }
    ///0x260..0x27c - no description available
    #[inline(always)]
    pub const fn chctrlch17(&self) -> &Chctrl {
        self.chctrl(17)
    }
    ///0x280..0x29c - no description available
    #[inline(always)]
    pub const fn chctrlch18(&self) -> &Chctrl {
        self.chctrl(18)
    }
    ///0x2a0..0x2bc - no description available
    #[inline(always)]
    pub const fn chctrlch19(&self) -> &Chctrl {
        self.chctrl(19)
    }
    ///0x2c0..0x2dc - no description available
    #[inline(always)]
    pub const fn chctrlch20(&self) -> &Chctrl {
        self.chctrl(20)
    }
    ///0x2e0..0x2fc - no description available
    #[inline(always)]
    pub const fn chctrlch21(&self) -> &Chctrl {
        self.chctrl(21)
    }
    ///0x300..0x31c - no description available
    #[inline(always)]
    pub const fn chctrlch22(&self) -> &Chctrl {
        self.chctrl(22)
    }
    ///0x320..0x33c - no description available
    #[inline(always)]
    pub const fn chctrlch23(&self) -> &Chctrl {
        self.chctrl(23)
    }
    ///0x340..0x35c - no description available
    #[inline(always)]
    pub const fn chctrlch24(&self) -> &Chctrl {
        self.chctrl(24)
    }
    ///0x360..0x37c - no description available
    #[inline(always)]
    pub const fn chctrlch25(&self) -> &Chctrl {
        self.chctrl(25)
    }
    ///0x380..0x39c - no description available
    #[inline(always)]
    pub const fn chctrlch26(&self) -> &Chctrl {
        self.chctrl(26)
    }
    ///0x3a0..0x3bc - no description available
    #[inline(always)]
    pub const fn chctrlch27(&self) -> &Chctrl {
        self.chctrl(27)
    }
    ///0x3c0..0x3dc - no description available
    #[inline(always)]
    pub const fn chctrlch28(&self) -> &Chctrl {
        self.chctrl(28)
    }
    ///0x3e0..0x3fc - no description available
    #[inline(always)]
    pub const fn chctrlch29(&self) -> &Chctrl {
        self.chctrl(29)
    }
    ///0x400..0x41c - no description available
    #[inline(always)]
    pub const fn chctrlch30(&self) -> &Chctrl {
        self.chctrl(30)
    }
    ///0x420..0x43c - no description available
    #[inline(always)]
    pub const fn chctrlch31(&self) -> &Chctrl {
        self.chctrl(31)
    }
}
/**IDMisc (rw) register accessor: ID Misc

You can [`read`](crate::Reg::read) this register and get [`idmisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@idmisc`] module*/
#[doc(alias = "IDMisc")]
pub type Idmisc = crate::Reg<idmisc::IdmiscSpec>;
///ID Misc
pub mod idmisc;
/**DMACfg (rw) register accessor: DMAC Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacfg`] module*/
#[doc(alias = "DMACfg")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
///DMAC Configuration Register
pub mod dmacfg;
/**DMACtrl (rw) register accessor: DMAC Control Register

You can [`read`](crate::Reg::read) this register and get [`dmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmactrl`] module*/
#[doc(alias = "DMACtrl")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
///DMAC Control Register
pub mod dmactrl;
/**ChAbort (rw) register accessor: Channel Abort Register

You can [`read`](crate::Reg::read) this register and get [`ch_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_abort`] module*/
pub type ChAbort = crate::Reg<ch_abort::ChAbortSpec>;
///Channel Abort Register
pub mod ch_abort;
/**INTHALFSTS (rw) register accessor: Harlf Complete Interrupt Status

You can [`read`](crate::Reg::read) this register and get [`inthalfsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inthalfsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inthalfsts`] module*/
#[doc(alias = "INTHALFSTS")]
pub type Inthalfsts = crate::Reg<inthalfsts::InthalfstsSpec>;
///Harlf Complete Interrupt Status
pub mod inthalfsts;
/**INTTCSTS (rw) register accessor: Trans Complete Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`inttcsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttcsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inttcsts`] module*/
#[doc(alias = "INTTCSTS")]
pub type Inttcsts = crate::Reg<inttcsts::InttcstsSpec>;
///Trans Complete Interrupt Status Register
pub mod inttcsts;
/**INTABORTSTS (rw) register accessor: Abort Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`intabortsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intabortsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intabortsts`] module*/
#[doc(alias = "INTABORTSTS")]
pub type Intabortsts = crate::Reg<intabortsts::IntabortstsSpec>;
///Abort Interrupt Status Register
pub mod intabortsts;
/**INTERRSTS (rw) register accessor: Error Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`interrsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@interrsts`] module*/
#[doc(alias = "INTERRSTS")]
pub type Interrsts = crate::Reg<interrsts::InterrstsSpec>;
///Error Interrupt Status Register
pub mod interrsts;
/**ChEN (rw) register accessor: Channel Enable Register

You can [`read`](crate::Reg::read) this register and get [`ch_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_en`] module*/
#[doc(alias = "ChEN")]
pub type ChEn = crate::Reg<ch_en::ChEnSpec>;
///Channel Enable Register
pub mod ch_en;
///no description available
pub use self::chctrl::Chctrl;
///Cluster
///no description available
pub mod chctrl;
