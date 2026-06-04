#[repr(C)]
///Register block
pub struct RegisterBlock {
    muxcfg: [Muxcfg; 32],
}
impl RegisterBlock {
    ///0x00..0x80 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `MUXCFGHDMA_MUX0` register.</div>
    #[inline(always)]
    pub const fn muxcfg(&self, n: usize) -> &Muxcfg {
        &self.muxcfg[n]
    }
    ///Iterator for array of:
    ///0x00..0x80 - no description available
    #[inline(always)]
    pub fn muxcfg_iter(&self) -> impl Iterator<Item = &Muxcfg> {
        self.muxcfg.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux0(&self) -> &Muxcfg {
        self.muxcfg(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux1(&self) -> &Muxcfg {
        self.muxcfg(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux2(&self) -> &Muxcfg {
        self.muxcfg(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux3(&self) -> &Muxcfg {
        self.muxcfg(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux4(&self) -> &Muxcfg {
        self.muxcfg(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux5(&self) -> &Muxcfg {
        self.muxcfg(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux6(&self) -> &Muxcfg {
        self.muxcfg(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux7(&self) -> &Muxcfg {
        self.muxcfg(7)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux8(&self) -> &Muxcfg {
        self.muxcfg(8)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux9(&self) -> &Muxcfg {
        self.muxcfg(9)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux10(&self) -> &Muxcfg {
        self.muxcfg(10)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux11(&self) -> &Muxcfg {
        self.muxcfg(11)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux12(&self) -> &Muxcfg {
        self.muxcfg(12)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux13(&self) -> &Muxcfg {
        self.muxcfg(13)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux14(&self) -> &Muxcfg {
        self.muxcfg(14)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux15(&self) -> &Muxcfg {
        self.muxcfg(15)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux16(&self) -> &Muxcfg {
        self.muxcfg(16)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux17(&self) -> &Muxcfg {
        self.muxcfg(17)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux18(&self) -> &Muxcfg {
        self.muxcfg(18)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux19(&self) -> &Muxcfg {
        self.muxcfg(19)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux20(&self) -> &Muxcfg {
        self.muxcfg(20)
    }
    ///0x54 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux21(&self) -> &Muxcfg {
        self.muxcfg(21)
    }
    ///0x58 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux22(&self) -> &Muxcfg {
        self.muxcfg(22)
    }
    ///0x5c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux23(&self) -> &Muxcfg {
        self.muxcfg(23)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux24(&self) -> &Muxcfg {
        self.muxcfg(24)
    }
    ///0x64 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux25(&self) -> &Muxcfg {
        self.muxcfg(25)
    }
    ///0x68 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux26(&self) -> &Muxcfg {
        self.muxcfg(26)
    }
    ///0x6c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux27(&self) -> &Muxcfg {
        self.muxcfg(27)
    }
    ///0x70 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux28(&self) -> &Muxcfg {
        self.muxcfg(28)
    }
    ///0x74 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux29(&self) -> &Muxcfg {
        self.muxcfg(29)
    }
    ///0x78 - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux30(&self) -> &Muxcfg {
        self.muxcfg(30)
    }
    ///0x7c - no description available
    #[inline(always)]
    pub const fn muxcfghdma_mux31(&self) -> &Muxcfg {
        self.muxcfg(31)
    }
}
/**MUXCFG (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`muxcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@muxcfg`] module*/
#[doc(alias = "MUXCFG")]
pub type Muxcfg = crate::Reg<muxcfg::MuxcfgSpec>;
///no description available
pub mod muxcfg;
