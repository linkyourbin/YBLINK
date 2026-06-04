#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    cfg: Cfg,
    int_en: IntEn,
    status: Status,
    addr: Addr,
    data: Data,
    ctrl: Ctrl,
    cmd: Cmd,
    setup: Setup,
    tpm: Tpm,
}
impl RegisterBlock {
    ///0x10 - Configuration Register
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    ///0x14 - Interrupt Enable Register
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    ///0x18 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x1c - Address Register
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    ///0x20 - Data Register
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    ///0x24 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x28 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x2c - Setup Register
    #[inline(always)]
    pub const fn setup(&self) -> &Setup {
        &self.setup
    }
    ///0x30 - I2C Timing Paramater Multiplier
    #[inline(always)]
    pub const fn tpm(&self) -> &Tpm {
        &self.tpm
    }
}
/**Cfg (rw) register accessor: Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfg`] module*/
pub type Cfg = crate::Reg<cfg::CfgSpec>;
///Configuration Register
pub mod cfg;
/**IntEn (rw) register accessor: Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_en`] module*/
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
///Interrupt Enable Register
pub mod int_en;
/**Status (rw) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type Status = crate::Reg<status::StatusSpec>;
///Status Register
pub mod status;
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
/**Cmd (rw) register accessor: Command Register

You can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type Cmd = crate::Reg<cmd::CmdSpec>;
///Command Register
pub mod cmd;
/**Setup (rw) register accessor: Setup Register

You can [`read`](crate::Reg::read) this register and get [`setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setup`] module*/
pub type Setup = crate::Reg<setup::SetupSpec>;
///Setup Register
pub mod setup;
/**TPM (rw) register accessor: I2C Timing Paramater Multiplier

You can [`read`](crate::Reg::read) this register and get [`tpm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tpm`] module*/
#[doc(alias = "TPM")]
pub type Tpm = crate::Reg<tpm::TpmSpec>;
///I2C Timing Paramater Multiplier
pub mod tpm;
