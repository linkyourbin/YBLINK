#[repr(C)]
///Register block
pub struct RegisterBlock {
    bandgap: Bandgap,
    ldo1p1: Ldo1p1,
    ldo2p5: Ldo2p5,
    _reserved3: [u8; 0x04],
    dcdc_mode: DcdcMode,
    dcdc_lpmode: DcdcLpmode,
    dcdc_prot: DcdcProt,
    dcdc_current: DcdcCurrent,
    dcdc_advmode: DcdcAdvmode,
    dcdc_advparam: DcdcAdvparam,
    dcdc_misc: DcdcMisc,
    dcdc_debug: DcdcDebug,
    dcdc_start_time: DcdcStartTime,
    dcdc_resume_time: DcdcResumeTime,
    _reserved13: [u8; 0x08],
    power_trap: PowerTrap,
    wake_cause: WakeCause,
    wake_mask: WakeMask,
    scg_ctrl: ScgCtrl,
    _reserved17: [u8; 0x10],
    rc24m: Rc24m,
    rc24m_track: Rc24mTrack,
    track_target: TrackTarget,
    status: Status,
}
impl RegisterBlock {
    ///0x00 - BANGGAP control
    #[inline(always)]
    pub const fn bandgap(&self) -> &Bandgap {
        &self.bandgap
    }
    ///0x04 - 1V LDO config
    #[inline(always)]
    pub const fn ldo1p1(&self) -> &Ldo1p1 {
        &self.ldo1p1
    }
    ///0x08 - 2.5V LDO config
    #[inline(always)]
    pub const fn ldo2p5(&self) -> &Ldo2p5 {
        &self.ldo2p5
    }
    ///0x10 - DCDC mode select
    #[inline(always)]
    pub const fn dcdc_mode(&self) -> &DcdcMode {
        &self.dcdc_mode
    }
    ///0x14 - DCDC low power mode
    #[inline(always)]
    pub const fn dcdc_lpmode(&self) -> &DcdcLpmode {
        &self.dcdc_lpmode
    }
    ///0x18 - DCDC protection
    #[inline(always)]
    pub const fn dcdc_prot(&self) -> &DcdcProt {
        &self.dcdc_prot
    }
    ///0x1c - DCDC current estimation
    #[inline(always)]
    pub const fn dcdc_current(&self) -> &DcdcCurrent {
        &self.dcdc_current
    }
    ///0x20 - DCDC advance setting
    #[inline(always)]
    pub const fn dcdc_advmode(&self) -> &DcdcAdvmode {
        &self.dcdc_advmode
    }
    ///0x24 - DCDC advance parameter
    #[inline(always)]
    pub const fn dcdc_advparam(&self) -> &DcdcAdvparam {
        &self.dcdc_advparam
    }
    ///0x28 - DCDC misc parameter
    #[inline(always)]
    pub const fn dcdc_misc(&self) -> &DcdcMisc {
        &self.dcdc_misc
    }
    ///0x2c - DCDC Debug
    #[inline(always)]
    pub const fn dcdc_debug(&self) -> &DcdcDebug {
        &self.dcdc_debug
    }
    ///0x30 - DCDC ramp time
    #[inline(always)]
    pub const fn dcdc_start_time(&self) -> &DcdcStartTime {
        &self.dcdc_start_time
    }
    ///0x34 - DCDC resume time
    #[inline(always)]
    pub const fn dcdc_resume_time(&self) -> &DcdcResumeTime {
        &self.dcdc_resume_time
    }
    ///0x40 - SOC power trap
    #[inline(always)]
    pub const fn power_trap(&self) -> &PowerTrap {
        &self.power_trap
    }
    ///0x44 - Wake up source
    #[inline(always)]
    pub const fn wake_cause(&self) -> &WakeCause {
        &self.wake_cause
    }
    ///0x48 - Wake up mask
    #[inline(always)]
    pub const fn wake_mask(&self) -> &WakeMask {
        &self.wake_mask
    }
    ///0x4c - Clock gate control in PMIC
    #[inline(always)]
    pub const fn scg_ctrl(&self) -> &ScgCtrl {
        &self.scg_ctrl
    }
    ///0x60 - RC 24M config
    #[inline(always)]
    pub const fn rc24m(&self) -> &Rc24m {
        &self.rc24m
    }
    ///0x64 - RC 24M track mode
    #[inline(always)]
    pub const fn rc24m_track(&self) -> &Rc24mTrack {
        &self.rc24m_track
    }
    ///0x68 - RC 24M track target
    #[inline(always)]
    pub const fn track_target(&self) -> &TrackTarget {
        &self.track_target
    }
    ///0x6c - RC 24M track status
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
/**BANDGAP (rw) register accessor: BANGGAP control

You can [`read`](crate::Reg::read) this register and get [`bandgap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bandgap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bandgap`] module*/
#[doc(alias = "BANDGAP")]
pub type Bandgap = crate::Reg<bandgap::BandgapSpec>;
///BANGGAP control
pub mod bandgap;
/**LDO1P1 (rw) register accessor: 1V LDO config

You can [`read`](crate::Reg::read) this register and get [`ldo1p1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo1p1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ldo1p1`] module*/
#[doc(alias = "LDO1P1")]
pub type Ldo1p1 = crate::Reg<ldo1p1::Ldo1p1Spec>;
///1V LDO config
pub mod ldo1p1;
/**LDO2P5 (rw) register accessor: 2.5V LDO config

You can [`read`](crate::Reg::read) this register and get [`ldo2p5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo2p5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ldo2p5`] module*/
#[doc(alias = "LDO2P5")]
pub type Ldo2p5 = crate::Reg<ldo2p5::Ldo2p5Spec>;
///2.5V LDO config
pub mod ldo2p5;
/**DCDC_MODE (rw) register accessor: DCDC mode select

You can [`read`](crate::Reg::read) this register and get [`dcdc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_mode`] module*/
#[doc(alias = "DCDC_MODE")]
pub type DcdcMode = crate::Reg<dcdc_mode::DcdcModeSpec>;
///DCDC mode select
pub mod dcdc_mode;
/**DCDC_LPMODE (rw) register accessor: DCDC low power mode

You can [`read`](crate::Reg::read) this register and get [`dcdc_lpmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_lpmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_lpmode`] module*/
#[doc(alias = "DCDC_LPMODE")]
pub type DcdcLpmode = crate::Reg<dcdc_lpmode::DcdcLpmodeSpec>;
///DCDC low power mode
pub mod dcdc_lpmode;
/**DCDC_PROT (rw) register accessor: DCDC protection

You can [`read`](crate::Reg::read) this register and get [`dcdc_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_prot`] module*/
#[doc(alias = "DCDC_PROT")]
pub type DcdcProt = crate::Reg<dcdc_prot::DcdcProtSpec>;
///DCDC protection
pub mod dcdc_prot;
/**DCDC_CURRENT (rw) register accessor: DCDC current estimation

You can [`read`](crate::Reg::read) this register and get [`dcdc_current::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_current::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_current`] module*/
#[doc(alias = "DCDC_CURRENT")]
pub type DcdcCurrent = crate::Reg<dcdc_current::DcdcCurrentSpec>;
///DCDC current estimation
pub mod dcdc_current;
/**DCDC_ADVMODE (rw) register accessor: DCDC advance setting

You can [`read`](crate::Reg::read) this register and get [`dcdc_advmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_advmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_advmode`] module*/
#[doc(alias = "DCDC_ADVMODE")]
pub type DcdcAdvmode = crate::Reg<dcdc_advmode::DcdcAdvmodeSpec>;
///DCDC advance setting
pub mod dcdc_advmode;
/**DCDC_ADVPARAM (rw) register accessor: DCDC advance parameter

You can [`read`](crate::Reg::read) this register and get [`dcdc_advparam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_advparam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_advparam`] module*/
#[doc(alias = "DCDC_ADVPARAM")]
pub type DcdcAdvparam = crate::Reg<dcdc_advparam::DcdcAdvparamSpec>;
///DCDC advance parameter
pub mod dcdc_advparam;
/**DCDC_MISC (rw) register accessor: DCDC misc parameter

You can [`read`](crate::Reg::read) this register and get [`dcdc_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_misc`] module*/
#[doc(alias = "DCDC_MISC")]
pub type DcdcMisc = crate::Reg<dcdc_misc::DcdcMiscSpec>;
///DCDC misc parameter
pub mod dcdc_misc;
/**DCDC_DEBUG (rw) register accessor: DCDC Debug

You can [`read`](crate::Reg::read) this register and get [`dcdc_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_debug`] module*/
#[doc(alias = "DCDC_DEBUG")]
pub type DcdcDebug = crate::Reg<dcdc_debug::DcdcDebugSpec>;
///DCDC Debug
pub mod dcdc_debug;
/**DCDC_START_TIME (rw) register accessor: DCDC ramp time

You can [`read`](crate::Reg::read) this register and get [`dcdc_start_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_start_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_start_time`] module*/
#[doc(alias = "DCDC_START_TIME")]
pub type DcdcStartTime = crate::Reg<dcdc_start_time::DcdcStartTimeSpec>;
///DCDC ramp time
pub mod dcdc_start_time;
/**DCDC_RESUME_TIME (rw) register accessor: DCDC resume time

You can [`read`](crate::Reg::read) this register and get [`dcdc_resume_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_resume_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_resume_time`] module*/
#[doc(alias = "DCDC_RESUME_TIME")]
pub type DcdcResumeTime = crate::Reg<dcdc_resume_time::DcdcResumeTimeSpec>;
///DCDC resume time
pub mod dcdc_resume_time;
/**POWER_TRAP (rw) register accessor: SOC power trap

You can [`read`](crate::Reg::read) this register and get [`power_trap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_trap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_trap`] module*/
#[doc(alias = "POWER_TRAP")]
pub type PowerTrap = crate::Reg<power_trap::PowerTrapSpec>;
///SOC power trap
pub mod power_trap;
/**WAKE_CAUSE (rw) register accessor: Wake up source

You can [`read`](crate::Reg::read) this register and get [`wake_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wake_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wake_cause`] module*/
#[doc(alias = "WAKE_CAUSE")]
pub type WakeCause = crate::Reg<wake_cause::WakeCauseSpec>;
///Wake up source
pub mod wake_cause;
/**WAKE_MASK (rw) register accessor: Wake up mask

You can [`read`](crate::Reg::read) this register and get [`wake_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wake_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wake_mask`] module*/
#[doc(alias = "WAKE_MASK")]
pub type WakeMask = crate::Reg<wake_mask::WakeMaskSpec>;
///Wake up mask
pub mod wake_mask;
/**SCG_CTRL (rw) register accessor: Clock gate control in PMIC

You can [`read`](crate::Reg::read) this register and get [`scg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scg_ctrl`] module*/
#[doc(alias = "SCG_CTRL")]
pub type ScgCtrl = crate::Reg<scg_ctrl::ScgCtrlSpec>;
///Clock gate control in PMIC
pub mod scg_ctrl;
/**RC24M (rw) register accessor: RC 24M config

You can [`read`](crate::Reg::read) this register and get [`rc24m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc24m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rc24m`] module*/
#[doc(alias = "RC24M")]
pub type Rc24m = crate::Reg<rc24m::Rc24mSpec>;
///RC 24M config
pub mod rc24m;
/**RC24M_TRACK (rw) register accessor: RC 24M track mode

You can [`read`](crate::Reg::read) this register and get [`rc24m_track::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc24m_track::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rc24m_track`] module*/
#[doc(alias = "RC24M_TRACK")]
pub type Rc24mTrack = crate::Reg<rc24m_track::Rc24mTrackSpec>;
///RC 24M track mode
pub mod rc24m_track;
/**TRACK_TARGET (rw) register accessor: RC 24M track target

You can [`read`](crate::Reg::read) this register and get [`track_target::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`track_target::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@track_target`] module*/
#[doc(alias = "TRACK_TARGET")]
pub type TrackTarget = crate::Reg<track_target::TrackTargetSpec>;
///RC 24M track target
pub mod track_target;
/**STATUS (rw) register accessor: RC 24M track status

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
///RC 24M track status
pub mod status;
