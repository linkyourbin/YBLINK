#[repr(C)]
///no description available
#[doc(alias = "CPU")]
pub struct Cpu {
    lp: Lp,
    lock: Lock,
    gpr: [Gpr; 14],
    wakeup_status: [WakeupStatus; 4],
    _reserved4: [u8; 0x30],
    wakeup_enable: [WakeupEnable; 4],
}
impl Cpu {
    ///0x00 - CPU0 LP control
    #[inline(always)]
    pub const fn lp(&self) -> &Lp {
        &self.lp
    }
    ///0x04 - CPU0 Lock GPR
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    ///0x08..0x40 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GPRGPR0` register.</div>
    #[inline(always)]
    pub const fn gpr(&self, n: usize) -> &Gpr {
        &self.gpr[n]
    }
    ///Iterator for array of:
    ///0x08..0x40 - no description available
    #[inline(always)]
    pub fn gpr_iter(&self) -> impl Iterator<Item = &Gpr> {
        self.gpr.iter()
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn gprgpr0(&self) -> &Gpr {
        self.gpr(0)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn gprgpr1(&self) -> &Gpr {
        self.gpr(1)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn gprgpr2(&self) -> &Gpr {
        self.gpr(2)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn gprgpr3(&self) -> &Gpr {
        self.gpr(3)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn gprgpr4(&self) -> &Gpr {
        self.gpr(4)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn gprgpr5(&self) -> &Gpr {
        self.gpr(5)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn gprgpr6(&self) -> &Gpr {
        self.gpr(6)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn gprgpr7(&self) -> &Gpr {
        self.gpr(7)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn gprgpr8(&self) -> &Gpr {
        self.gpr(8)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn gprgpr9(&self) -> &Gpr {
        self.gpr(9)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn gprgpr10(&self) -> &Gpr {
        self.gpr(10)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn gprgpr11(&self) -> &Gpr {
        self.gpr(11)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn gprgpr12(&self) -> &Gpr {
        self.gpr(12)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn gprgpr13(&self) -> &Gpr {
        self.gpr(13)
    }
    ///0x40..0x50 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `WAKEUP_STATUSSTATUS0` register.</div>
    #[inline(always)]
    pub const fn wakeup_status(&self, n: usize) -> &WakeupStatus {
        &self.wakeup_status[n]
    }
    ///Iterator for array of:
    ///0x40..0x50 - no description available
    #[inline(always)]
    pub fn wakeup_status_iter(&self) -> impl Iterator<Item = &WakeupStatus> {
        self.wakeup_status.iter()
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn wakeup_statusstatus0(&self) -> &WakeupStatus {
        self.wakeup_status(0)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn wakeup_statusstatus1(&self) -> &WakeupStatus {
        self.wakeup_status(1)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn wakeup_statusstatus2(&self) -> &WakeupStatus {
        self.wakeup_status(2)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn wakeup_statusstatus3(&self) -> &WakeupStatus {
        self.wakeup_status(3)
    }
    ///0x80..0x90 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `WAKEUP_ENABLEENABLE0` register.</div>
    #[inline(always)]
    pub const fn wakeup_enable(&self, n: usize) -> &WakeupEnable {
        &self.wakeup_enable[n]
    }
    ///Iterator for array of:
    ///0x80..0x90 - no description available
    #[inline(always)]
    pub fn wakeup_enable_iter(&self) -> impl Iterator<Item = &WakeupEnable> {
        self.wakeup_enable.iter()
    }
    ///0x80 - no description available
    #[inline(always)]
    pub const fn wakeup_enableenable0(&self) -> &WakeupEnable {
        self.wakeup_enable(0)
    }
    ///0x84 - no description available
    #[inline(always)]
    pub const fn wakeup_enableenable1(&self) -> &WakeupEnable {
        self.wakeup_enable(1)
    }
    ///0x88 - no description available
    #[inline(always)]
    pub const fn wakeup_enableenable2(&self) -> &WakeupEnable {
        self.wakeup_enable(2)
    }
    ///0x8c - no description available
    #[inline(always)]
    pub const fn wakeup_enableenable3(&self) -> &WakeupEnable {
        self.wakeup_enable(3)
    }
}
/**LP (rw) register accessor: CPU0 LP control

You can [`read`](crate::Reg::read) this register and get [`lp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp`] module*/
#[doc(alias = "LP")]
pub type Lp = crate::Reg<lp::LpSpec>;
///CPU0 LP control
pub mod lp;
/**LOCK (rw) register accessor: CPU0 Lock GPR

You can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lock`] module*/
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
///CPU0 Lock GPR
pub mod lock;
/**GPR (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`gpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpr`] module*/
#[doc(alias = "GPR")]
pub type Gpr = crate::Reg<gpr::GprSpec>;
///no description available
pub mod gpr;
/**WAKEUP_STATUS (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`wakeup_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wakeup_status`] module*/
#[doc(alias = "WAKEUP_STATUS")]
pub type WakeupStatus = crate::Reg<wakeup_status::WakeupStatusSpec>;
///no description available
pub mod wakeup_status;
/**WAKEUP_ENABLE (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`wakeup_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wakeup_enable`] module*/
#[doc(alias = "WAKEUP_ENABLE")]
pub type WakeupEnable = crate::Reg<wakeup_enable::WakeupEnableSpec>;
///no description available
pub mod wakeup_enable;
