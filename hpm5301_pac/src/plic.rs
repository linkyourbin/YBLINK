#[repr(C)]
///Register block
pub struct RegisterBlock {
    feature: Feature,
    priority: [Priority; 72],
    _reserved2: [u8; 0x0edc],
    pending: [Pending; 4],
    _reserved3: [u8; 0x70],
    trigger: [Trigger; 4],
    _reserved4: [u8; 0x70],
    number: Number,
    info: Info,
    _reserved6: [u8; 0x0ef8],
    targetint: [Targetint; 1],
    _reserved7: [u8; 0x001f_dfe8],
    targetconfig: [Targetconfig; 1],
}
impl RegisterBlock {
    ///0x00 - Feature enable register
    #[inline(always)]
    pub const fn feature(&self) -> &Feature {
        &self.feature
    }
    ///0x04..0x124 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PRIORITYPRIORITY1` register.</div>
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &Priority {
        &self.priority[n]
    }
    ///Iterator for array of:
    ///0x04..0x124 - no description available
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &Priority> {
        self.priority.iter()
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn prioritypriority1(&self) -> &Priority {
        self.priority(0)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn prioritypriority2(&self) -> &Priority {
        self.priority(1)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn prioritypriority3(&self) -> &Priority {
        self.priority(2)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn prioritypriority4(&self) -> &Priority {
        self.priority(3)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn prioritypriority5(&self) -> &Priority {
        self.priority(4)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn prioritypriority6(&self) -> &Priority {
        self.priority(5)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn prioritypriority7(&self) -> &Priority {
        self.priority(6)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn prioritypriority8(&self) -> &Priority {
        self.priority(7)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn prioritypriority9(&self) -> &Priority {
        self.priority(8)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn prioritypriority10(&self) -> &Priority {
        self.priority(9)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn prioritypriority11(&self) -> &Priority {
        self.priority(10)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn prioritypriority12(&self) -> &Priority {
        self.priority(11)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn prioritypriority13(&self) -> &Priority {
        self.priority(12)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn prioritypriority14(&self) -> &Priority {
        self.priority(13)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn prioritypriority15(&self) -> &Priority {
        self.priority(14)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn prioritypriority16(&self) -> &Priority {
        self.priority(15)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn prioritypriority17(&self) -> &Priority {
        self.priority(16)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn prioritypriority18(&self) -> &Priority {
        self.priority(17)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn prioritypriority19(&self) -> &Priority {
        self.priority(18)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn prioritypriority20(&self) -> &Priority {
        self.priority(19)
    }
    ///0x54 - no description available
    #[inline(always)]
    pub const fn prioritypriority21(&self) -> &Priority {
        self.priority(20)
    }
    ///0x58 - no description available
    #[inline(always)]
    pub const fn prioritypriority22(&self) -> &Priority {
        self.priority(21)
    }
    ///0x5c - no description available
    #[inline(always)]
    pub const fn prioritypriority23(&self) -> &Priority {
        self.priority(22)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn prioritypriority24(&self) -> &Priority {
        self.priority(23)
    }
    ///0x64 - no description available
    #[inline(always)]
    pub const fn prioritypriority25(&self) -> &Priority {
        self.priority(24)
    }
    ///0x68 - no description available
    #[inline(always)]
    pub const fn prioritypriority26(&self) -> &Priority {
        self.priority(25)
    }
    ///0x6c - no description available
    #[inline(always)]
    pub const fn prioritypriority27(&self) -> &Priority {
        self.priority(26)
    }
    ///0x70 - no description available
    #[inline(always)]
    pub const fn prioritypriority28(&self) -> &Priority {
        self.priority(27)
    }
    ///0x74 - no description available
    #[inline(always)]
    pub const fn prioritypriority29(&self) -> &Priority {
        self.priority(28)
    }
    ///0x78 - no description available
    #[inline(always)]
    pub const fn prioritypriority30(&self) -> &Priority {
        self.priority(29)
    }
    ///0x7c - no description available
    #[inline(always)]
    pub const fn prioritypriority31(&self) -> &Priority {
        self.priority(30)
    }
    ///0x80 - no description available
    #[inline(always)]
    pub const fn prioritypriority32(&self) -> &Priority {
        self.priority(31)
    }
    ///0x84 - no description available
    #[inline(always)]
    pub const fn prioritypriority33(&self) -> &Priority {
        self.priority(32)
    }
    ///0x88 - no description available
    #[inline(always)]
    pub const fn prioritypriority34(&self) -> &Priority {
        self.priority(33)
    }
    ///0x8c - no description available
    #[inline(always)]
    pub const fn prioritypriority35(&self) -> &Priority {
        self.priority(34)
    }
    ///0x90 - no description available
    #[inline(always)]
    pub const fn prioritypriority36(&self) -> &Priority {
        self.priority(35)
    }
    ///0x94 - no description available
    #[inline(always)]
    pub const fn prioritypriority37(&self) -> &Priority {
        self.priority(36)
    }
    ///0x98 - no description available
    #[inline(always)]
    pub const fn prioritypriority38(&self) -> &Priority {
        self.priority(37)
    }
    ///0x9c - no description available
    #[inline(always)]
    pub const fn prioritypriority39(&self) -> &Priority {
        self.priority(38)
    }
    ///0xa0 - no description available
    #[inline(always)]
    pub const fn prioritypriority40(&self) -> &Priority {
        self.priority(39)
    }
    ///0xa4 - no description available
    #[inline(always)]
    pub const fn prioritypriority41(&self) -> &Priority {
        self.priority(40)
    }
    ///0xa8 - no description available
    #[inline(always)]
    pub const fn prioritypriority42(&self) -> &Priority {
        self.priority(41)
    }
    ///0xac - no description available
    #[inline(always)]
    pub const fn prioritypriority43(&self) -> &Priority {
        self.priority(42)
    }
    ///0xb0 - no description available
    #[inline(always)]
    pub const fn prioritypriority44(&self) -> &Priority {
        self.priority(43)
    }
    ///0xb4 - no description available
    #[inline(always)]
    pub const fn prioritypriority45(&self) -> &Priority {
        self.priority(44)
    }
    ///0xb8 - no description available
    #[inline(always)]
    pub const fn prioritypriority46(&self) -> &Priority {
        self.priority(45)
    }
    ///0xbc - no description available
    #[inline(always)]
    pub const fn prioritypriority47(&self) -> &Priority {
        self.priority(46)
    }
    ///0xc0 - no description available
    #[inline(always)]
    pub const fn prioritypriority48(&self) -> &Priority {
        self.priority(47)
    }
    ///0xc4 - no description available
    #[inline(always)]
    pub const fn prioritypriority49(&self) -> &Priority {
        self.priority(48)
    }
    ///0xc8 - no description available
    #[inline(always)]
    pub const fn prioritypriority50(&self) -> &Priority {
        self.priority(49)
    }
    ///0xcc - no description available
    #[inline(always)]
    pub const fn prioritypriority51(&self) -> &Priority {
        self.priority(50)
    }
    ///0xd0 - no description available
    #[inline(always)]
    pub const fn prioritypriority52(&self) -> &Priority {
        self.priority(51)
    }
    ///0xd4 - no description available
    #[inline(always)]
    pub const fn prioritypriority53(&self) -> &Priority {
        self.priority(52)
    }
    ///0xd8 - no description available
    #[inline(always)]
    pub const fn prioritypriority54(&self) -> &Priority {
        self.priority(53)
    }
    ///0xdc - no description available
    #[inline(always)]
    pub const fn prioritypriority55(&self) -> &Priority {
        self.priority(54)
    }
    ///0xe0 - no description available
    #[inline(always)]
    pub const fn prioritypriority56(&self) -> &Priority {
        self.priority(55)
    }
    ///0xe4 - no description available
    #[inline(always)]
    pub const fn prioritypriority57(&self) -> &Priority {
        self.priority(56)
    }
    ///0xe8 - no description available
    #[inline(always)]
    pub const fn prioritypriority58(&self) -> &Priority {
        self.priority(57)
    }
    ///0xec - no description available
    #[inline(always)]
    pub const fn prioritypriority59(&self) -> &Priority {
        self.priority(58)
    }
    ///0xf0 - no description available
    #[inline(always)]
    pub const fn prioritypriority60(&self) -> &Priority {
        self.priority(59)
    }
    ///0xf4 - no description available
    #[inline(always)]
    pub const fn prioritypriority61(&self) -> &Priority {
        self.priority(60)
    }
    ///0xf8 - no description available
    #[inline(always)]
    pub const fn prioritypriority62(&self) -> &Priority {
        self.priority(61)
    }
    ///0xfc - no description available
    #[inline(always)]
    pub const fn prioritypriority63(&self) -> &Priority {
        self.priority(62)
    }
    ///0x100 - no description available
    #[inline(always)]
    pub const fn prioritypriority64(&self) -> &Priority {
        self.priority(63)
    }
    ///0x104 - no description available
    #[inline(always)]
    pub const fn prioritypriority65(&self) -> &Priority {
        self.priority(64)
    }
    ///0x108 - no description available
    #[inline(always)]
    pub const fn prioritypriority66(&self) -> &Priority {
        self.priority(65)
    }
    ///0x10c - no description available
    #[inline(always)]
    pub const fn prioritypriority67(&self) -> &Priority {
        self.priority(66)
    }
    ///0x110 - no description available
    #[inline(always)]
    pub const fn prioritypriority68(&self) -> &Priority {
        self.priority(67)
    }
    ///0x114 - no description available
    #[inline(always)]
    pub const fn prioritypriority69(&self) -> &Priority {
        self.priority(68)
    }
    ///0x118 - no description available
    #[inline(always)]
    pub const fn prioritypriority70(&self) -> &Priority {
        self.priority(69)
    }
    ///0x11c - no description available
    #[inline(always)]
    pub const fn prioritypriority71(&self) -> &Priority {
        self.priority(70)
    }
    ///0x120 - no description available
    #[inline(always)]
    pub const fn prioritypriority72(&self) -> &Priority {
        self.priority(71)
    }
    ///0x1000..0x1010 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PENDINGPENDING0` register.</div>
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &Pending {
        &self.pending[n]
    }
    ///Iterator for array of:
    ///0x1000..0x1010 - no description available
    #[inline(always)]
    pub fn pending_iter(&self) -> impl Iterator<Item = &Pending> {
        self.pending.iter()
    }
    ///0x1000 - no description available
    #[inline(always)]
    pub const fn pendingpending0(&self) -> &Pending {
        self.pending(0)
    }
    ///0x1004 - no description available
    #[inline(always)]
    pub const fn pendingpending1(&self) -> &Pending {
        self.pending(1)
    }
    ///0x1008 - no description available
    #[inline(always)]
    pub const fn pendingpending2(&self) -> &Pending {
        self.pending(2)
    }
    ///0x100c - no description available
    #[inline(always)]
    pub const fn pendingpending3(&self) -> &Pending {
        self.pending(3)
    }
    ///0x1080..0x1090 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TRIGGERTRIGGER0` register.</div>
    #[inline(always)]
    pub const fn trigger(&self, n: usize) -> &Trigger {
        &self.trigger[n]
    }
    ///Iterator for array of:
    ///0x1080..0x1090 - no description available
    #[inline(always)]
    pub fn trigger_iter(&self) -> impl Iterator<Item = &Trigger> {
        self.trigger.iter()
    }
    ///0x1080 - no description available
    #[inline(always)]
    pub const fn triggertrigger0(&self) -> &Trigger {
        self.trigger(0)
    }
    ///0x1084 - no description available
    #[inline(always)]
    pub const fn triggertrigger1(&self) -> &Trigger {
        self.trigger(1)
    }
    ///0x1088 - no description available
    #[inline(always)]
    pub const fn triggertrigger2(&self) -> &Trigger {
        self.trigger(2)
    }
    ///0x108c - no description available
    #[inline(always)]
    pub const fn triggertrigger3(&self) -> &Trigger {
        self.trigger(3)
    }
    ///0x1100 - Number of supported interrupt sources and targets
    #[inline(always)]
    pub const fn number(&self) -> &Number {
        &self.number
    }
    ///0x1104 - Version and the maximum priority
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    ///0x2000..0x2018 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `TARGETINTtarget0` cluster.</div>
    #[inline(always)]
    pub const fn targetint(&self, n: usize) -> &Targetint {
        &self.targetint[n]
    }
    ///Iterator for array of:
    ///0x2000..0x2018 - no description available
    #[inline(always)]
    pub fn targetint_iter(&self) -> impl Iterator<Item = &Targetint> {
        self.targetint.iter()
    }
    ///0x2000..0x2018 - no description available
    #[inline(always)]
    pub const fn targetinttarget0(&self) -> &Targetint {
        self.targetint(0)
    }
    ///0x200000..0x200404 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `TARGETCONFIGtarget0` cluster.</div>
    #[inline(always)]
    pub const fn targetconfig(&self, n: usize) -> &Targetconfig {
        &self.targetconfig[n]
    }
    ///Iterator for array of:
    ///0x200000..0x200404 - no description available
    #[inline(always)]
    pub fn targetconfig_iter(&self) -> impl Iterator<Item = &Targetconfig> {
        self.targetconfig.iter()
    }
    ///0x200000..0x200404 - no description available
    #[inline(always)]
    pub const fn targetconfigtarget0(&self) -> &Targetconfig {
        self.targetconfig(0)
    }
}
/**feature (rw) register accessor: Feature enable register

You can [`read`](crate::Reg::read) this register and get [`feature::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@feature`] module*/
#[doc(alias = "feature")]
pub type Feature = crate::Reg<feature::FeatureSpec>;
///Feature enable register
pub mod feature;
/**PRIORITY (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@priority`] module*/
#[doc(alias = "PRIORITY")]
pub type Priority = crate::Reg<priority::PrioritySpec>;
///no description available
pub mod priority;
/**PENDING (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pending`] module*/
#[doc(alias = "PENDING")]
pub type Pending = crate::Reg<pending::PendingSpec>;
///no description available
pub mod pending;
/**TRIGGER (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trigger`] module*/
#[doc(alias = "TRIGGER")]
pub type Trigger = crate::Reg<trigger::TriggerSpec>;
///no description available
pub mod trigger;
/**NUMBER (rw) register accessor: Number of supported interrupt sources and targets

You can [`read`](crate::Reg::read) this register and get [`number::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`number::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@number`] module*/
#[doc(alias = "NUMBER")]
pub type Number = crate::Reg<number::NumberSpec>;
///Number of supported interrupt sources and targets
pub mod number;
/**INFO (rw) register accessor: Version and the maximum priority

You can [`read`](crate::Reg::read) this register and get [`info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@info`] module*/
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
///Version and the maximum priority
pub mod info;
///no description available
pub use self::targetint::Targetint;
///Cluster
///no description available
pub mod targetint;
///no description available
pub use self::targetconfig::Targetconfig;
///Cluster
///no description available
pub mod targetconfig;
