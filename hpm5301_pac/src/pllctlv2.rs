#[repr(C)]
///Register block
pub struct RegisterBlock {
    xtal: Xtal,
    _reserved1: [u8; 0x7c],
    pll: (),
}
impl RegisterBlock {
    ///0x00 - OSC configuration
    #[inline(always)]
    pub const fn xtal(&self) -> &Xtal {
        &self.xtal
    }
    ///0x80..0x118 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `PLLpll0` cluster.</div>
    #[inline(always)]
    pub const fn pll(&self, n: usize) -> &Pll {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(128 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x80..0x118 - no description available
    #[inline(always)]
    pub fn pll_iter(&self) -> impl Iterator<Item = &Pll> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(128 * n)
                .cast()
        })
    }
    ///0x80..0xcc - no description available
    #[inline(always)]
    pub const fn pllpll0(&self) -> &Pll {
        self.pll(0)
    }
    ///0x100..0x14c - no description available
    #[inline(always)]
    pub const fn pllpll1(&self) -> &Pll {
        self.pll(1)
    }
}
/**XTAL (rw) register accessor: OSC configuration

You can [`read`](crate::Reg::read) this register and get [`xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal`] module*/
#[doc(alias = "XTAL")]
pub type Xtal = crate::Reg<xtal::XtalSpec>;
///OSC configuration
pub mod xtal;
///no description available
pub use self::pll::Pll;
///Cluster
///no description available
pub mod pll;
