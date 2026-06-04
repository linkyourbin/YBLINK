#[repr(C)]
///no description available
#[doc(alias = "GROUP0")]
pub struct Group0 {
    value: Value,
    set: Set,
    clear: Clear,
    toggle: Toggle,
}
impl Group0 {
    ///0x00 - Group setting
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
    ///0x04 - Group setting
    #[inline(always)]
    pub const fn set(&self) -> &Set {
        &self.set
    }
    ///0x08 - Group setting
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
    ///0x0c - Group setting
    #[inline(always)]
    pub const fn toggle(&self) -> &Toggle {
        &self.toggle
    }
}
/**VALUE (rw) register accessor: Group setting

You can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@value`] module*/
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
///Group setting
pub mod value;
/**SET (rw) register accessor: Group setting

You can [`read`](crate::Reg::read) this register and get [`set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set`] module*/
#[doc(alias = "SET")]
pub type Set = crate::Reg<set::SetSpec>;
///Group setting
pub mod set;
/**CLEAR (rw) register accessor: Group setting

You can [`read`](crate::Reg::read) this register and get [`clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clear`] module*/
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
///Group setting
pub mod clear;
/**TOGGLE (rw) register accessor: Group setting

You can [`read`](crate::Reg::read) this register and get [`toggle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toggle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@toggle`] module*/
#[doc(alias = "TOGGLE")]
pub type Toggle = crate::Reg<toggle::ToggleSpec>;
///Group setting
pub mod toggle;
