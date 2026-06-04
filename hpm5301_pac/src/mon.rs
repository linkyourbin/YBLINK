#[repr(C)]
///Register block
pub struct RegisterBlock {
    monitor: [Monitor; 4],
    _reserved1: [u8; 0x20],
    irq_flag: IrqFlag,
    irq_enable: IrqEnable,
}
impl RegisterBlock {
    ///0x00..0x20 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `MONITORglitch0` cluster.</div>
    #[inline(always)]
    pub const fn monitor(&self, n: usize) -> &Monitor {
        &self.monitor[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - no description available
    #[inline(always)]
    pub fn monitor_iter(&self) -> impl Iterator<Item = &Monitor> {
        self.monitor.iter()
    }
    ///0x00..0x08 - no description available
    #[inline(always)]
    pub const fn monitorglitch0(&self) -> &Monitor {
        self.monitor(0)
    }
    ///0x08..0x10 - no description available
    #[inline(always)]
    pub const fn monitorglitch1(&self) -> &Monitor {
        self.monitor(1)
    }
    ///0x10..0x18 - no description available
    #[inline(always)]
    pub const fn monitorclock0(&self) -> &Monitor {
        self.monitor(2)
    }
    ///0x18..0x20 - no description available
    #[inline(always)]
    pub const fn monitorclock1(&self) -> &Monitor {
        self.monitor(3)
    }
    ///0x40 - No description available
    #[inline(always)]
    pub const fn irq_flag(&self) -> &IrqFlag {
        &self.irq_flag
    }
    ///0x44 - No description available
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IrqEnable {
        &self.irq_enable
    }
}
///no description available
pub use self::monitor::Monitor;
///Cluster
///no description available
pub mod monitor;
/**IRQ_FLAG (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`irq_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irq_flag`] module*/
#[doc(alias = "IRQ_FLAG")]
pub type IrqFlag = crate::Reg<irq_flag::IrqFlagSpec>;
///No description available
pub mod irq_flag;
/**IRQ_ENABLE (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irq_enable`] module*/
#[doc(alias = "IRQ_ENABLE")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
///No description available
pub mod irq_enable;
