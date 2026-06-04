#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    gptimer0ld: Gptimer0ld,
    gptimer0ctrl: Gptimer0ctrl,
    gptimer1ld: Gptimer1ld,
    gptimer1ctrl: Gptimer1ctrl,
    sbuscfg: Sbuscfg,
    _reserved5: [u8; 0xac],
    usbcmd: Usbcmd,
    usbsts: Usbsts,
    usbintr: Usbintr,
    frindex: Frindex,
    _reserved9: [u8; 0x04],
    _reserved_9_union_154: [u8; 0x04],
    _reserved_10_union_158: [u8; 0x04],
    _reserved11: [u8; 0x04],
    burstsize: Burstsize,
    txfilltuning: Txfilltuning,
    _reserved13: [u8; 0x10],
    endptnak: Endptnak,
    endptnaken: Endptnaken,
    _reserved15: [u8; 0x04],
    portsc1: Portsc1,
    _reserved16: [u8; 0x1c],
    otgsc: Otgsc,
    usbmode: Usbmode,
    endptsetupstat: Endptsetupstat,
    endptprime: Endptprime,
    endptflush: Endptflush,
    endptstat: Endptstat,
    endptcomplete: Endptcomplete,
    endptctrl: [Endptctrl; 16],
    otg_ctrl0: OtgCtrl0,
    _reserved25: [u8; 0x0c],
    phy_ctrl0: PhyCtrl0,
    phy_ctrl1: PhyCtrl1,
    _reserved27: [u8; 0x08],
    top_status: TopStatus,
    phy_status: PhyStatus,
}
impl RegisterBlock {
    ///0x80 - General Purpose Timer #0 Load Register
    #[inline(always)]
    pub const fn gptimer0ld(&self) -> &Gptimer0ld {
        &self.gptimer0ld
    }
    ///0x84 - General Purpose Timer #0 Controller Register
    #[inline(always)]
    pub const fn gptimer0ctrl(&self) -> &Gptimer0ctrl {
        &self.gptimer0ctrl
    }
    ///0x88 - General Purpose Timer #1 Load Register
    #[inline(always)]
    pub const fn gptimer1ld(&self) -> &Gptimer1ld {
        &self.gptimer1ld
    }
    ///0x8c - General Purpose Timer #1 Controller Register
    #[inline(always)]
    pub const fn gptimer1ctrl(&self) -> &Gptimer1ctrl {
        &self.gptimer1ctrl
    }
    ///0x90 - System Bus Config Register
    #[inline(always)]
    pub const fn sbuscfg(&self) -> &Sbuscfg {
        &self.sbuscfg
    }
    ///0x140 - USB Command Register
    #[inline(always)]
    pub const fn usbcmd(&self) -> &Usbcmd {
        &self.usbcmd
    }
    ///0x144 - USB Status Register
    #[inline(always)]
    pub const fn usbsts(&self) -> &Usbsts {
        &self.usbsts
    }
    ///0x148 - Interrupt Enable Register
    #[inline(always)]
    pub const fn usbintr(&self) -> &Usbintr {
        &self.usbintr
    }
    ///0x14c - USB Frame Index Register
    #[inline(always)]
    pub const fn frindex(&self) -> &Frindex {
        &self.frindex
    }
    ///0x154 - Frame List Base Address Register
    #[inline(always)]
    pub const fn union_154_periodiclistbase(&self) -> &Union154Periodiclistbase {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    ///0x154 - Device Address Register
    #[inline(always)]
    pub const fn union_154_deviceaddr(&self) -> &Union154Deviceaddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    ///0x158 - Endpoint List Address Register
    #[inline(always)]
    pub const fn union_158_endptlistaddr(&self) -> &Union158Endptlistaddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    ///0x158 - Next Asynch. Address Register
    #[inline(always)]
    pub const fn union_158_asynclistaddr(&self) -> &Union158Asynclistaddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    ///0x160 - Programmable Burst Size Register
    #[inline(always)]
    pub const fn burstsize(&self) -> &Burstsize {
        &self.burstsize
    }
    ///0x164 - TX FIFO Fill Tuning Register
    #[inline(always)]
    pub const fn txfilltuning(&self) -> &Txfilltuning {
        &self.txfilltuning
    }
    ///0x178 - Endpoint NAK Register
    #[inline(always)]
    pub const fn endptnak(&self) -> &Endptnak {
        &self.endptnak
    }
    ///0x17c - Endpoint NAK Enable Register
    #[inline(always)]
    pub const fn endptnaken(&self) -> &Endptnaken {
        &self.endptnaken
    }
    ///0x184 - Port Status & Control
    #[inline(always)]
    pub const fn portsc1(&self) -> &Portsc1 {
        &self.portsc1
    }
    ///0x1a4 - On-The-Go Status & control Register
    #[inline(always)]
    pub const fn otgsc(&self) -> &Otgsc {
        &self.otgsc
    }
    ///0x1a8 - USB Device Mode Register
    #[inline(always)]
    pub const fn usbmode(&self) -> &Usbmode {
        &self.usbmode
    }
    ///0x1ac - Endpoint Setup Status Register
    #[inline(always)]
    pub const fn endptsetupstat(&self) -> &Endptsetupstat {
        &self.endptsetupstat
    }
    ///0x1b0 - Endpoint Prime Register
    #[inline(always)]
    pub const fn endptprime(&self) -> &Endptprime {
        &self.endptprime
    }
    ///0x1b4 - Endpoint Flush Register
    #[inline(always)]
    pub const fn endptflush(&self) -> &Endptflush {
        &self.endptflush
    }
    ///0x1b8 - Endpoint Status Register
    #[inline(always)]
    pub const fn endptstat(&self) -> &Endptstat {
        &self.endptstat
    }
    ///0x1bc - Endpoint Complete Register
    #[inline(always)]
    pub const fn endptcomplete(&self) -> &Endptcomplete {
        &self.endptcomplete
    }
    ///0x1c0..0x200 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ENDPTCTRLENDPTCTRL0` register.</div>
    #[inline(always)]
    pub const fn endptctrl(&self, n: usize) -> &Endptctrl {
        &self.endptctrl[n]
    }
    ///Iterator for array of:
    ///0x1c0..0x200 - no description available
    #[inline(always)]
    pub fn endptctrl_iter(&self) -> impl Iterator<Item = &Endptctrl> {
        self.endptctrl.iter()
    }
    ///0x1c0 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl0(&self) -> &Endptctrl {
        self.endptctrl(0)
    }
    ///0x1c4 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl1(&self) -> &Endptctrl {
        self.endptctrl(1)
    }
    ///0x1c8 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl2(&self) -> &Endptctrl {
        self.endptctrl(2)
    }
    ///0x1cc - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl3(&self) -> &Endptctrl {
        self.endptctrl(3)
    }
    ///0x1d0 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl4(&self) -> &Endptctrl {
        self.endptctrl(4)
    }
    ///0x1d4 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl5(&self) -> &Endptctrl {
        self.endptctrl(5)
    }
    ///0x1d8 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl6(&self) -> &Endptctrl {
        self.endptctrl(6)
    }
    ///0x1dc - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl7(&self) -> &Endptctrl {
        self.endptctrl(7)
    }
    ///0x1e0 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl8(&self) -> &Endptctrl {
        self.endptctrl(8)
    }
    ///0x1e4 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl9(&self) -> &Endptctrl {
        self.endptctrl(9)
    }
    ///0x1e8 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl10(&self) -> &Endptctrl {
        self.endptctrl(10)
    }
    ///0x1ec - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl11(&self) -> &Endptctrl {
        self.endptctrl(11)
    }
    ///0x1f0 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl12(&self) -> &Endptctrl {
        self.endptctrl(12)
    }
    ///0x1f4 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl13(&self) -> &Endptctrl {
        self.endptctrl(13)
    }
    ///0x1f8 - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl14(&self) -> &Endptctrl {
        self.endptctrl(14)
    }
    ///0x1fc - no description available
    #[inline(always)]
    pub const fn endptctrlendptctrl15(&self) -> &Endptctrl {
        self.endptctrl(15)
    }
    ///0x200 - No description available
    #[inline(always)]
    pub const fn otg_ctrl0(&self) -> &OtgCtrl0 {
        &self.otg_ctrl0
    }
    ///0x210 - No description available
    #[inline(always)]
    pub const fn phy_ctrl0(&self) -> &PhyCtrl0 {
        &self.phy_ctrl0
    }
    ///0x214 - No description available
    #[inline(always)]
    pub const fn phy_ctrl1(&self) -> &PhyCtrl1 {
        &self.phy_ctrl1
    }
    ///0x220 - No description available
    #[inline(always)]
    pub const fn top_status(&self) -> &TopStatus {
        &self.top_status
    }
    ///0x224 - No description available
    #[inline(always)]
    pub const fn phy_status(&self) -> &PhyStatus {
        &self.phy_status
    }
}
/**GPTIMER0LD (rw) register accessor: General Purpose Timer #0 Load Register

You can [`read`](crate::Reg::read) this register and get [`gptimer0ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptimer0ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gptimer0ld`] module*/
#[doc(alias = "GPTIMER0LD")]
pub type Gptimer0ld = crate::Reg<gptimer0ld::Gptimer0ldSpec>;
///General Purpose Timer #0 Load Register
pub mod gptimer0ld;
/**GPTIMER0CTRL (rw) register accessor: General Purpose Timer #0 Controller Register

You can [`read`](crate::Reg::read) this register and get [`gptimer0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptimer0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gptimer0ctrl`] module*/
#[doc(alias = "GPTIMER0CTRL")]
pub type Gptimer0ctrl = crate::Reg<gptimer0ctrl::Gptimer0ctrlSpec>;
///General Purpose Timer #0 Controller Register
pub mod gptimer0ctrl;
/**GPTIMER1LD (rw) register accessor: General Purpose Timer #1 Load Register

You can [`read`](crate::Reg::read) this register and get [`gptimer1ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptimer1ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gptimer1ld`] module*/
#[doc(alias = "GPTIMER1LD")]
pub type Gptimer1ld = crate::Reg<gptimer1ld::Gptimer1ldSpec>;
///General Purpose Timer #1 Load Register
pub mod gptimer1ld;
/**GPTIMER1CTRL (rw) register accessor: General Purpose Timer #1 Controller Register

You can [`read`](crate::Reg::read) this register and get [`gptimer1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptimer1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gptimer1ctrl`] module*/
#[doc(alias = "GPTIMER1CTRL")]
pub type Gptimer1ctrl = crate::Reg<gptimer1ctrl::Gptimer1ctrlSpec>;
///General Purpose Timer #1 Controller Register
pub mod gptimer1ctrl;
/**SBUSCFG (rw) register accessor: System Bus Config Register

You can [`read`](crate::Reg::read) this register and get [`sbuscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbuscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sbuscfg`] module*/
#[doc(alias = "SBUSCFG")]
pub type Sbuscfg = crate::Reg<sbuscfg::SbuscfgSpec>;
///System Bus Config Register
pub mod sbuscfg;
/**USBCMD (rw) register accessor: USB Command Register

You can [`read`](crate::Reg::read) this register and get [`usbcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbcmd`] module*/
#[doc(alias = "USBCMD")]
pub type Usbcmd = crate::Reg<usbcmd::UsbcmdSpec>;
///USB Command Register
pub mod usbcmd;
/**USBSTS (rw) register accessor: USB Status Register

You can [`read`](crate::Reg::read) this register and get [`usbsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbsts`] module*/
#[doc(alias = "USBSTS")]
pub type Usbsts = crate::Reg<usbsts::UsbstsSpec>;
///USB Status Register
pub mod usbsts;
/**USBINTR (rw) register accessor: Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`usbintr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbintr`] module*/
#[doc(alias = "USBINTR")]
pub type Usbintr = crate::Reg<usbintr::UsbintrSpec>;
///Interrupt Enable Register
pub mod usbintr;
/**FRINDEX (rw) register accessor: USB Frame Index Register

You can [`read`](crate::Reg::read) this register and get [`frindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frindex`] module*/
#[doc(alias = "FRINDEX")]
pub type Frindex = crate::Reg<frindex::FrindexSpec>;
///USB Frame Index Register
pub mod frindex;
/**UNION_154_DEVICEADDR (rw) register accessor: Device Address Register

You can [`read`](crate::Reg::read) this register and get [`union_154_deviceaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_154_deviceaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_154_deviceaddr`] module*/
#[doc(alias = "UNION_154_DEVICEADDR")]
pub type Union154Deviceaddr = crate::Reg<union_154_deviceaddr::Union154DeviceaddrSpec>;
///Device Address Register
pub mod union_154_deviceaddr;
/**UNION_154_PERIODICLISTBASE (rw) register accessor: Frame List Base Address Register

You can [`read`](crate::Reg::read) this register and get [`union_154_periodiclistbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_154_periodiclistbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_154_periodiclistbase`] module*/
#[doc(alias = "UNION_154_PERIODICLISTBASE")]
pub type Union154Periodiclistbase =
    crate::Reg<union_154_periodiclistbase::Union154PeriodiclistbaseSpec>;
///Frame List Base Address Register
pub mod union_154_periodiclistbase;
/**UNION_158_ASYNCLISTADDR (rw) register accessor: Next Asynch. Address Register

You can [`read`](crate::Reg::read) this register and get [`union_158_asynclistaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_158_asynclistaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_158_asynclistaddr`] module*/
#[doc(alias = "UNION_158_ASYNCLISTADDR")]
pub type Union158Asynclistaddr = crate::Reg<union_158_asynclistaddr::Union158AsynclistaddrSpec>;
///Next Asynch. Address Register
pub mod union_158_asynclistaddr;
/**UNION_158_ENDPTLISTADDR (rw) register accessor: Endpoint List Address Register

You can [`read`](crate::Reg::read) this register and get [`union_158_endptlistaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_158_endptlistaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@union_158_endptlistaddr`] module*/
#[doc(alias = "UNION_158_ENDPTLISTADDR")]
pub type Union158Endptlistaddr = crate::Reg<union_158_endptlistaddr::Union158EndptlistaddrSpec>;
///Endpoint List Address Register
pub mod union_158_endptlistaddr;
/**BURSTSIZE (rw) register accessor: Programmable Burst Size Register

You can [`read`](crate::Reg::read) this register and get [`burstsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`burstsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@burstsize`] module*/
#[doc(alias = "BURSTSIZE")]
pub type Burstsize = crate::Reg<burstsize::BurstsizeSpec>;
///Programmable Burst Size Register
pub mod burstsize;
/**TXFILLTUNING (rw) register accessor: TX FIFO Fill Tuning Register

You can [`read`](crate::Reg::read) this register and get [`txfilltuning::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfilltuning::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txfilltuning`] module*/
#[doc(alias = "TXFILLTUNING")]
pub type Txfilltuning = crate::Reg<txfilltuning::TxfilltuningSpec>;
///TX FIFO Fill Tuning Register
pub mod txfilltuning;
/**ENDPTNAK (rw) register accessor: Endpoint NAK Register

You can [`read`](crate::Reg::read) this register and get [`endptnak::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptnak::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptnak`] module*/
#[doc(alias = "ENDPTNAK")]
pub type Endptnak = crate::Reg<endptnak::EndptnakSpec>;
///Endpoint NAK Register
pub mod endptnak;
/**ENDPTNAKEN (rw) register accessor: Endpoint NAK Enable Register

You can [`read`](crate::Reg::read) this register and get [`endptnaken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptnaken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptnaken`] module*/
#[doc(alias = "ENDPTNAKEN")]
pub type Endptnaken = crate::Reg<endptnaken::EndptnakenSpec>;
///Endpoint NAK Enable Register
pub mod endptnaken;
/**PORTSC1 (rw) register accessor: Port Status & Control

You can [`read`](crate::Reg::read) this register and get [`portsc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portsc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portsc1`] module*/
#[doc(alias = "PORTSC1")]
pub type Portsc1 = crate::Reg<portsc1::Portsc1Spec>;
///Port Status & Control
pub mod portsc1;
/**OTGSC (rw) register accessor: On-The-Go Status & control Register

You can [`read`](crate::Reg::read) this register and get [`otgsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@otgsc`] module*/
#[doc(alias = "OTGSC")]
pub type Otgsc = crate::Reg<otgsc::OtgscSpec>;
///On-The-Go Status & control Register
pub mod otgsc;
/**USBMODE (rw) register accessor: USB Device Mode Register

You can [`read`](crate::Reg::read) this register and get [`usbmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbmode`] module*/
#[doc(alias = "USBMODE")]
pub type Usbmode = crate::Reg<usbmode::UsbmodeSpec>;
///USB Device Mode Register
pub mod usbmode;
/**ENDPTSETUPSTAT (rw) register accessor: Endpoint Setup Status Register

You can [`read`](crate::Reg::read) this register and get [`endptsetupstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptsetupstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptsetupstat`] module*/
#[doc(alias = "ENDPTSETUPSTAT")]
pub type Endptsetupstat = crate::Reg<endptsetupstat::EndptsetupstatSpec>;
///Endpoint Setup Status Register
pub mod endptsetupstat;
/**ENDPTPRIME (rw) register accessor: Endpoint Prime Register

You can [`read`](crate::Reg::read) this register and get [`endptprime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptprime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptprime`] module*/
#[doc(alias = "ENDPTPRIME")]
pub type Endptprime = crate::Reg<endptprime::EndptprimeSpec>;
///Endpoint Prime Register
pub mod endptprime;
/**ENDPTFLUSH (rw) register accessor: Endpoint Flush Register

You can [`read`](crate::Reg::read) this register and get [`endptflush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptflush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptflush`] module*/
#[doc(alias = "ENDPTFLUSH")]
pub type Endptflush = crate::Reg<endptflush::EndptflushSpec>;
///Endpoint Flush Register
pub mod endptflush;
/**ENDPTSTAT (rw) register accessor: Endpoint Status Register

You can [`read`](crate::Reg::read) this register and get [`endptstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptstat`] module*/
#[doc(alias = "ENDPTSTAT")]
pub type Endptstat = crate::Reg<endptstat::EndptstatSpec>;
///Endpoint Status Register
pub mod endptstat;
/**ENDPTCOMPLETE (rw) register accessor: Endpoint Complete Register

You can [`read`](crate::Reg::read) this register and get [`endptcomplete::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptcomplete::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptcomplete`] module*/
#[doc(alias = "ENDPTCOMPLETE")]
pub type Endptcomplete = crate::Reg<endptcomplete::EndptcompleteSpec>;
///Endpoint Complete Register
pub mod endptcomplete;
/**ENDPTCTRL (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`endptctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endptctrl`] module*/
#[doc(alias = "ENDPTCTRL")]
pub type Endptctrl = crate::Reg<endptctrl::EndptctrlSpec>;
///no description available
pub mod endptctrl;
/**OTG_CTRL0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`otg_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@otg_ctrl0`] module*/
#[doc(alias = "OTG_CTRL0")]
pub type OtgCtrl0 = crate::Reg<otg_ctrl0::OtgCtrl0Spec>;
///No description available
pub mod otg_ctrl0;
/**PHY_CTRL0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`phy_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_ctrl0`] module*/
#[doc(alias = "PHY_CTRL0")]
pub type PhyCtrl0 = crate::Reg<phy_ctrl0::PhyCtrl0Spec>;
///No description available
pub mod phy_ctrl0;
/**PHY_CTRL1 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`phy_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_ctrl1`] module*/
#[doc(alias = "PHY_CTRL1")]
pub type PhyCtrl1 = crate::Reg<phy_ctrl1::PhyCtrl1Spec>;
///No description available
pub mod phy_ctrl1;
/**TOP_STATUS (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`top_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@top_status`] module*/
#[doc(alias = "TOP_STATUS")]
pub type TopStatus = crate::Reg<top_status::TopStatusSpec>;
///No description available
pub mod top_status;
/**PHY_STATUS (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`phy_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_status`] module*/
#[doc(alias = "PHY_STATUS")]
pub type PhyStatus = crate::Reg<phy_status::PhyStatusSpec>;
///No description available
pub mod phy_status;
