#[repr(C)]
///no description available
#[doc(alias = "CHCTRL")]
pub struct Chctrl {
    ctrl: Ctrl,
    tran_size: TranSize,
    src_addr: SrcAddr,
    chan_req_ctrl: ChanReqCtrl,
    dst_addr: DstAddr,
    _reserved5: [u8; 0x04],
    llpointer: Llpointer,
}
impl Chctrl {
    ///0x00 - Channel &index0 Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Channel &index0Transfer Size Register
    #[inline(always)]
    pub const fn tran_size(&self) -> &TranSize {
        &self.tran_size
    }
    ///0x08 - Channel &index0 Source Address Low Part Register
    #[inline(always)]
    pub const fn src_addr(&self) -> &SrcAddr {
        &self.src_addr
    }
    ///0x0c - Channel &index0 DMA Request Control Register
    #[inline(always)]
    pub const fn chan_req_ctrl(&self) -> &ChanReqCtrl {
        &self.chan_req_ctrl
    }
    ///0x10 - Channel &index0 Destination Address Low Part Register
    #[inline(always)]
    pub const fn dst_addr(&self) -> &DstAddr {
        &self.dst_addr
    }
    ///0x18 - Channel &index0 Linked List Pointer Low Part Register
    #[inline(always)]
    pub const fn llpointer(&self) -> &Llpointer {
        &self.llpointer
    }
}
/**Ctrl (rw) register accessor: Channel &index0 Control Register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
///Channel &index0 Control Register
pub mod ctrl;
/**TranSize (rw) register accessor: Channel &index0Transfer Size Register

You can [`read`](crate::Reg::read) this register and get [`tran_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tran_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tran_size`] module*/
pub type TranSize = crate::Reg<tran_size::TranSizeSpec>;
///Channel &index0Transfer Size Register
pub mod tran_size;
/**SrcAddr (rw) register accessor: Channel &index0 Source Address Low Part Register

You can [`read`](crate::Reg::read) this register and get [`src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@src_addr`] module*/
pub type SrcAddr = crate::Reg<src_addr::SrcAddrSpec>;
///Channel &index0 Source Address Low Part Register
pub mod src_addr;
/**ChanReqCtrl (rw) register accessor: Channel &index0 DMA Request Control Register

You can [`read`](crate::Reg::read) this register and get [`chan_req_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_req_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chan_req_ctrl`] module*/
pub type ChanReqCtrl = crate::Reg<chan_req_ctrl::ChanReqCtrlSpec>;
///Channel &index0 DMA Request Control Register
pub mod chan_req_ctrl;
/**DstAddr (rw) register accessor: Channel &index0 Destination Address Low Part Register

You can [`read`](crate::Reg::read) this register and get [`dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dst_addr`] module*/
pub type DstAddr = crate::Reg<dst_addr::DstAddrSpec>;
///Channel &index0 Destination Address Low Part Register
pub mod dst_addr;
/**LLPointer (rw) register accessor: Channel &index0 Linked List Pointer Low Part Register

You can [`read`](crate::Reg::read) this register and get [`llpointer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llpointer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@llpointer`] module*/
#[doc(alias = "LLPointer")]
pub type Llpointer = crate::Reg<llpointer::LlpointerSpec>;
///Channel &index0 Linked List Pointer Low Part Register
pub mod llpointer;
