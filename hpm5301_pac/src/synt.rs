#[repr(C)]
///Register block
pub struct RegisterBlock {
    gcr: Gcr,
    rld: Rld,
    timestamp_new: TimestampNew,
    cnt: Cnt,
    timestamp_sav: TimestampSav,
    timestamp_cur: TimestampCur,
    _reserved6: [u8; 0x08],
    cmp: [Cmp; 4],
}
impl RegisterBlock {
    ///0x00 - Global control register
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    ///0x04 - Counter reload register
    #[inline(always)]
    pub const fn rld(&self) -> &Rld {
        &self.rld
    }
    ///0x08 - timestamp new value register
    #[inline(always)]
    pub const fn timestamp_new(&self) -> &TimestampNew {
        &self.timestamp_new
    }
    ///0x0c - Counter
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    ///0x10 - timestamp trig save value
    #[inline(always)]
    pub const fn timestamp_sav(&self) -> &TimestampSav {
        &self.timestamp_sav
    }
    ///0x14 - timestamp read value
    #[inline(always)]
    pub const fn timestamp_cur(&self) -> &TimestampCur {
        &self.timestamp_cur
    }
    ///0x20..0x30 - no description available
    #[inline(always)]
    pub const fn cmp(&self, n: usize) -> &Cmp {
        &self.cmp[n]
    }
    ///Iterator for array of:
    ///0x20..0x30 - no description available
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = &Cmp> {
        self.cmp.iter()
    }
}
/**gcr (rw) register accessor: Global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gcr`] module*/
#[doc(alias = "gcr")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
///Global control register
pub mod gcr;
/**rld (rw) register accessor: Counter reload register

You can [`read`](crate::Reg::read) this register and get [`rld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rld`] module*/
#[doc(alias = "rld")]
pub type Rld = crate::Reg<rld::RldSpec>;
///Counter reload register
pub mod rld;
/**timestamp_new (rw) register accessor: timestamp new value register

You can [`read`](crate::Reg::read) this register and get [`timestamp_new::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_new::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timestamp_new`] module*/
#[doc(alias = "timestamp_new")]
pub type TimestampNew = crate::Reg<timestamp_new::TimestampNewSpec>;
///timestamp new value register
pub mod timestamp_new;
/**cnt (rw) register accessor: Counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cnt`] module*/
#[doc(alias = "cnt")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
///Counter
pub mod cnt;
/**timestamp_sav (rw) register accessor: timestamp trig save value

You can [`read`](crate::Reg::read) this register and get [`timestamp_sav::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_sav::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timestamp_sav`] module*/
#[doc(alias = "timestamp_sav")]
pub type TimestampSav = crate::Reg<timestamp_sav::TimestampSavSpec>;
///timestamp trig save value
pub mod timestamp_sav;
/**timestamp_cur (rw) register accessor: timestamp read value

You can [`read`](crate::Reg::read) this register and get [`timestamp_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timestamp_cur`] module*/
#[doc(alias = "timestamp_cur")]
pub type TimestampCur = crate::Reg<timestamp_cur::TimestampCurSpec>;
///timestamp read value
pub mod timestamp_cur;
/**CMP (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmp`] module*/
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CmpSpec>;
///no description available
pub mod cmp;
