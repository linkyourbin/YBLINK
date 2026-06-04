#[repr(C)]
///no description available
#[doc(alias = "DI")]
pub struct Di {
    value: Value,
}
impl Di {
    ///0x00 - GPIO input value
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
/**VALUE (rw) register accessor: GPIO input value

You can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@value`] module*/
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
///GPIO input value
pub mod value;
