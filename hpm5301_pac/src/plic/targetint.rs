#[repr(C)]
///no description available
#[doc(alias = "TARGETINT")]
pub struct Targetint {
    inten: [Inten; 6],
}
impl Targetint {
    ///0x00..0x18 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `INTENINTEN0` register.</div>
    #[inline(always)]
    pub const fn inten(&self, n: usize) -> &Inten {
        &self.inten[n]
    }
    ///Iterator for array of:
    ///0x00..0x18 - no description available
    #[inline(always)]
    pub fn inten_iter(&self) -> impl Iterator<Item = &Inten> {
        self.inten.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn inteninten0(&self) -> &Inten {
        self.inten(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn inteninten1(&self) -> &Inten {
        self.inten(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn inteninten2(&self) -> &Inten {
        self.inten(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn inteninten3(&self) -> &Inten {
        self.inten(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn inteninten4(&self) -> &Inten {
        self.inten(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn inteninten5(&self) -> &Inten {
        self.inten(5)
    }
}
/**INTEN (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inten`] module*/
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
///no description available
pub mod inten;
