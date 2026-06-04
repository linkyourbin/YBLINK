#[repr(C)]
///no description available
#[doc(alias = "CHANNEL")]
pub struct Channel {
    cfg: Cfg,
    daccfg: Daccfg,
    _reserved2: [u8; 0x08],
    sr: Sr,
    irqen: Irqen,
    dmaen: Dmaen,
}
impl Channel {
    ///0x00 - Configure Register
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    ///0x04 - DAC configure register
    #[inline(always)]
    pub const fn daccfg(&self) -> &Daccfg {
        &self.daccfg
    }
    ///0x10 - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x14 - Interrupt request enable register
    #[inline(always)]
    pub const fn irqen(&self) -> &Irqen {
        &self.irqen
    }
    ///0x18 - DMA request enable register
    #[inline(always)]
    pub const fn dmaen(&self) -> &Dmaen {
        &self.dmaen
    }
}
/**cfg (rw) register accessor: Configure Register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfg`] module*/
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
///Configure Register
pub mod cfg;
/**daccfg (rw) register accessor: DAC configure register

You can [`read`](crate::Reg::read) this register and get [`daccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daccfg`] module*/
#[doc(alias = "daccfg")]
pub type Daccfg = crate::Reg<daccfg::DaccfgSpec>;
///DAC configure register
pub mod daccfg;
/**sr (rw) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
#[doc(alias = "sr")]
pub type Sr = crate::Reg<sr::SrSpec>;
///Status register
pub mod sr;
/**irqen (rw) register accessor: Interrupt request enable register

You can [`read`](crate::Reg::read) this register and get [`irqen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irqen`] module*/
#[doc(alias = "irqen")]
pub type Irqen = crate::Reg<irqen::IrqenSpec>;
///Interrupt request enable register
pub mod irqen;
/**dmaen (rw) register accessor: DMA request enable register

You can [`read`](crate::Reg::read) this register and get [`dmaen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmaen`] module*/
#[doc(alias = "dmaen")]
pub type Dmaen = crate::Reg<dmaen::DmaenSpec>;
///DMA request enable register
pub mod dmaen;
