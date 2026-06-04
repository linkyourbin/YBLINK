#[repr(C)]
///no description available
#[doc(alias = "CHN")]
pub struct Chn {
    pre_set: PreSet,
    clr: Clr,
    poly: Poly,
    init_data: InitData,
    xorout: Xorout,
    misc_setting: MiscSetting,
    data: Data,
    result: Result,
}
impl Chn {
    ///0x00 - &index0 pre set for crc setting
    #[inline(always)]
    pub const fn pre_set(&self) -> &PreSet {
        &self.pre_set
    }
    ///0x04 - chn&index0 clear crc result and setting
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    ///0x08 - chn&index0 poly
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
    ///0x0c - chn&index0 init_data
    #[inline(always)]
    pub const fn init_data(&self) -> &InitData {
        &self.init_data
    }
    ///0x10 - chn&index0 xorout
    #[inline(always)]
    pub const fn xorout(&self) -> &Xorout {
        &self.xorout
    }
    ///0x14 - chn&index0 misc_setting
    #[inline(always)]
    pub const fn misc_setting(&self) -> &MiscSetting {
        &self.misc_setting
    }
    ///0x18 - chn&index0 data
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    ///0x1c - chn&index0 result
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
}
/**pre_set (rw) register accessor: &index0 pre set for crc setting

You can [`read`](crate::Reg::read) this register and get [`pre_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pre_set`] module*/
#[doc(alias = "pre_set")]
pub type PreSet = crate::Reg<pre_set::PreSetSpec>;
///&index0 pre set for crc setting
pub mod pre_set;
/**clr (rw) register accessor: chn&index0 clear crc result and setting

You can [`read`](crate::Reg::read) this register and get [`clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clr`] module*/
#[doc(alias = "clr")]
pub type Clr = crate::Reg<clr::ClrSpec>;
///chn&index0 clear crc result and setting
pub mod clr;
/**poly (rw) register accessor: chn&index0 poly

You can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@poly`] module*/
#[doc(alias = "poly")]
pub type Poly = crate::Reg<poly::PolySpec>;
///chn&index0 poly
pub mod poly;
/**init_data (rw) register accessor: chn&index0 init_data

You can [`read`](crate::Reg::read) this register and get [`init_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@init_data`] module*/
#[doc(alias = "init_data")]
pub type InitData = crate::Reg<init_data::InitDataSpec>;
///chn&index0 init_data
pub mod init_data;
/**xorout (rw) register accessor: chn&index0 xorout

You can [`read`](crate::Reg::read) this register and get [`xorout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xorout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xorout`] module*/
#[doc(alias = "xorout")]
pub type Xorout = crate::Reg<xorout::XoroutSpec>;
///chn&index0 xorout
pub mod xorout;
/**misc_setting (rw) register accessor: chn&index0 misc_setting

You can [`read`](crate::Reg::read) this register and get [`misc_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc_setting`] module*/
#[doc(alias = "misc_setting")]
pub type MiscSetting = crate::Reg<misc_setting::MiscSettingSpec>;
///chn&index0 misc_setting
pub mod misc_setting;
/**data (rw) register accessor: chn&index0 data

You can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
#[doc(alias = "data")]
pub type Data = crate::Reg<data::DataSpec>;
///chn&index0 data
pub mod data;
/**result (rw) register accessor: chn&index0 result

You can [`read`](crate::Reg::read) this register and get [`result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@result`] module*/
#[doc(alias = "result")]
pub type Result = crate::Reg<result::ResultSpec>;
///chn&index0 result
pub mod result;
