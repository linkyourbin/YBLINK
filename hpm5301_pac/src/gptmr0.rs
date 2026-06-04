#[repr(C)]
///Register block
pub struct RegisterBlock {
    channel: (),
    _reserved1: [u8; 0x0200],
    sr: Sr,
    irqen: Irqen,
    gcr: Gcr,
}
impl RegisterBlock {
    ///0x00..0xd0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CHANNELch0` cluster.</div>
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0xd0 - no description available
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() })
    }
    ///0x00..0x34 - no description available
    #[inline(always)]
    pub const fn channelch0(&self) -> &Channel {
        self.channel(0)
    }
    ///0x40..0x74 - no description available
    #[inline(always)]
    pub const fn channelch1(&self) -> &Channel {
        self.channel(1)
    }
    ///0x80..0xb4 - no description available
    #[inline(always)]
    pub const fn channelch2(&self) -> &Channel {
        self.channel(2)
    }
    ///0xc0..0xf4 - no description available
    #[inline(always)]
    pub const fn channelch3(&self) -> &Channel {
        self.channel(3)
    }
    ///0x200 - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x204 - Interrupt request enable register
    #[inline(always)]
    pub const fn irqen(&self) -> &Irqen {
        &self.irqen
    }
    ///0x208 - Global control register
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
}
///no description available
pub use self::channel::Channel;
///Cluster
///no description available
pub mod channel;
/**SR (rw) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
///Status register
pub mod sr;
/**IRQEN (rw) register accessor: Interrupt request enable register

You can [`read`](crate::Reg::read) this register and get [`irqen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irqen`] module*/
#[doc(alias = "IRQEN")]
pub type Irqen = crate::Reg<irqen::IrqenSpec>;
///Interrupt request enable register
pub mod irqen;
/**GCR (rw) register accessor: Global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gcr`] module*/
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
///Global control register
pub mod gcr;
