#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    wr_trans_cnt: WrTransCnt,
    rd_trans_cnt: RdTransCnt,
    _reserved2: [u8; 0x04],
    trans_fmt: TransFmt,
    direct_io: DirectIo,
    _reserved4: [u8; 0x08],
    trans_ctrl: TransCtrl,
    cmd: Cmd,
    addr: Addr,
    data: Data,
    ctrl: Ctrl,
    status: Status,
    intr_en: IntrEn,
    intr_st: IntrSt,
    timing: Timing,
    _reserved13: [u8; 0x1c],
    slv_st: SlvSt,
    slv_data_cnt: SlvDataCnt,
    slv_data_wcnt: SlvDataWcnt,
    slv_data_rcnt: SlvDataRcnt,
    _reserved17: [u8; 0x0c],
    config: Config,
}
impl RegisterBlock {
    ///0x04 - Transfer count for write data
    #[inline(always)]
    pub const fn wr_trans_cnt(&self) -> &WrTransCnt {
        &self.wr_trans_cnt
    }
    ///0x08 - Transfer count for read data
    #[inline(always)]
    pub const fn rd_trans_cnt(&self) -> &RdTransCnt {
        &self.rd_trans_cnt
    }
    ///0x10 - Transfer Format Register
    #[inline(always)]
    pub const fn trans_fmt(&self) -> &TransFmt {
        &self.trans_fmt
    }
    ///0x14 - Direct IO Control Register
    #[inline(always)]
    pub const fn direct_io(&self) -> &DirectIo {
        &self.direct_io
    }
    ///0x20 - Transfer Control Register
    #[inline(always)]
    pub const fn trans_ctrl(&self) -> &TransCtrl {
        &self.trans_ctrl
    }
    ///0x24 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x28 - Address Register
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    ///0x2c - Data Register
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    ///0x30 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x34 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x38 - Interrupt Enable Register
    #[inline(always)]
    pub const fn intr_en(&self) -> &IntrEn {
        &self.intr_en
    }
    ///0x3c - Interrupt Status Register
    #[inline(always)]
    pub const fn intr_st(&self) -> &IntrSt {
        &self.intr_st
    }
    ///0x40 - Interface Timing Register
    #[inline(always)]
    pub const fn timing(&self) -> &Timing {
        &self.timing
    }
    ///0x60 - Slave Status Register
    #[inline(always)]
    pub const fn slv_st(&self) -> &SlvSt {
        &self.slv_st
    }
    ///0x64 - Slave Data Count Register
    #[inline(always)]
    pub const fn slv_data_cnt(&self) -> &SlvDataCnt {
        &self.slv_data_cnt
    }
    ///0x68 - WCnt
    #[inline(always)]
    pub const fn slv_data_wcnt(&self) -> &SlvDataWcnt {
        &self.slv_data_wcnt
    }
    ///0x6c - RCnt
    #[inline(always)]
    pub const fn slv_data_rcnt(&self) -> &SlvDataRcnt {
        &self.slv_data_rcnt
    }
    ///0x7c - Configuration Register
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
/**wr_trans_cnt (rw) register accessor: Transfer count for write data

You can [`read`](crate::Reg::read) this register and get [`wr_trans_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_trans_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_trans_cnt`] module*/
#[doc(alias = "wr_trans_cnt")]
pub type WrTransCnt = crate::Reg<wr_trans_cnt::WrTransCntSpec>;
///Transfer count for write data
pub mod wr_trans_cnt;
/**rd_trans_cnt (rw) register accessor: Transfer count for read data

You can [`read`](crate::Reg::read) this register and get [`rd_trans_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_trans_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_trans_cnt`] module*/
#[doc(alias = "rd_trans_cnt")]
pub type RdTransCnt = crate::Reg<rd_trans_cnt::RdTransCntSpec>;
///Transfer count for read data
pub mod rd_trans_cnt;
/**TransFmt (rw) register accessor: Transfer Format Register

You can [`read`](crate::Reg::read) this register and get [`trans_fmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trans_fmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trans_fmt`] module*/
pub type TransFmt = crate::Reg<trans_fmt::TransFmtSpec>;
///Transfer Format Register
pub mod trans_fmt;
/**DirectIO (rw) register accessor: Direct IO Control Register

You can [`read`](crate::Reg::read) this register and get [`direct_io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct_io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@direct_io`] module*/
#[doc(alias = "DirectIO")]
pub type DirectIo = crate::Reg<direct_io::DirectIoSpec>;
///Direct IO Control Register
pub mod direct_io;
/**TransCtrl (rw) register accessor: Transfer Control Register

You can [`read`](crate::Reg::read) this register and get [`trans_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trans_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trans_ctrl`] module*/
pub type TransCtrl = crate::Reg<trans_ctrl::TransCtrlSpec>;
///Transfer Control Register
pub mod trans_ctrl;
/**Cmd (rw) register accessor: Command Register

You can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type Cmd = crate::Reg<cmd::CmdSpec>;
///Command Register
pub mod cmd;
/**Addr (rw) register accessor: Address Register

You can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
pub type Addr = crate::Reg<addr::AddrSpec>;
///Address Register
pub mod addr;
/**Data (rw) register accessor: Data Register

You can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
pub type Data = crate::Reg<data::DataSpec>;
///Data Register
pub mod data;
/**Ctrl (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
///Control Register
pub mod ctrl;
/**Status (rw) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type Status = crate::Reg<status::StatusSpec>;
///Status Register
pub mod status;
/**IntrEn (rw) register accessor: Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`intr_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_en`] module*/
pub type IntrEn = crate::Reg<intr_en::IntrEnSpec>;
///Interrupt Enable Register
pub mod intr_en;
/**IntrSt (rw) register accessor: Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`intr_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_st`] module*/
pub type IntrSt = crate::Reg<intr_st::IntrStSpec>;
///Interrupt Status Register
pub mod intr_st;
/**Timing (rw) register accessor: Interface Timing Register

You can [`read`](crate::Reg::read) this register and get [`timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timing`] module*/
pub type Timing = crate::Reg<timing::TimingSpec>;
///Interface Timing Register
pub mod timing;
/**SlvSt (rw) register accessor: Slave Status Register

You can [`read`](crate::Reg::read) this register and get [`slv_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_st`] module*/
pub type SlvSt = crate::Reg<slv_st::SlvStSpec>;
///Slave Status Register
pub mod slv_st;
/**SlvDataCnt (rw) register accessor: Slave Data Count Register

You can [`read`](crate::Reg::read) this register and get [`slv_data_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_data_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_data_cnt`] module*/
pub type SlvDataCnt = crate::Reg<slv_data_cnt::SlvDataCntSpec>;
///Slave Data Count Register
pub mod slv_data_cnt;
/**SlvDataWCnt (rw) register accessor: WCnt

You can [`read`](crate::Reg::read) this register and get [`slv_data_wcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_data_wcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_data_wcnt`] module*/
#[doc(alias = "SlvDataWCnt")]
pub type SlvDataWcnt = crate::Reg<slv_data_wcnt::SlvDataWcntSpec>;
///WCnt
pub mod slv_data_wcnt;
/**SlvDataRCnt (rw) register accessor: RCnt

You can [`read`](crate::Reg::read) this register and get [`slv_data_rcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_data_rcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_data_rcnt`] module*/
#[doc(alias = "SlvDataRCnt")]
pub type SlvDataRcnt = crate::Reg<slv_data_rcnt::SlvDataRcntSpec>;
///RCnt
pub mod slv_data_rcnt;
/**Config (rw) register accessor: Configuration Register

You can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
pub type Config = crate::Reg<config::ConfigSpec>;
///Configuration Register
pub mod config;
