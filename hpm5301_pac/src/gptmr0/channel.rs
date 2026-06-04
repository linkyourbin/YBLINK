#[repr(C)]
///no description available
#[doc(alias = "CHANNEL")]
pub struct Channel {
    cr: Cr,
    cmp: [Cmp; 2],
    rld: Rld,
    cntuptval: Cntuptval,
    _reserved4: [u8; 0x0c],
    cappos: Cappos,
    capneg: Capneg,
    capprd: Capprd,
    capdty: Capdty,
    cnt: Cnt,
}
impl Channel {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x04..0x0c - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CMPCMP0` register.</div>
    #[inline(always)]
    pub const fn cmp(&self, n: usize) -> &Cmp {
        &self.cmp[n]
    }
    ///Iterator for array of:
    ///0x04..0x0c - no description available
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = &Cmp> {
        self.cmp.iter()
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn cmpcmp0(&self) -> &Cmp {
        self.cmp(0)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn cmpcmp1(&self) -> &Cmp {
        self.cmp(1)
    }
    ///0x0c - Reload register
    #[inline(always)]
    pub const fn rld(&self) -> &Rld {
        &self.rld
    }
    ///0x10 - Counter update value register
    #[inline(always)]
    pub const fn cntuptval(&self) -> &Cntuptval {
        &self.cntuptval
    }
    ///0x20 - Capture rising edge register
    #[inline(always)]
    pub const fn cappos(&self) -> &Cappos {
        &self.cappos
    }
    ///0x24 - Capture falling edge register
    #[inline(always)]
    pub const fn capneg(&self) -> &Capneg {
        &self.capneg
    }
    ///0x28 - PWM period measure register
    #[inline(always)]
    pub const fn capprd(&self) -> &Capprd {
        &self.capprd
    }
    ///0x2c - PWM duty cycle measure register
    #[inline(always)]
    pub const fn capdty(&self) -> &Capdty {
        &self.capdty
    }
    ///0x30 - Counter
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
}
/**CR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
///Control Register
pub mod cr;
/**CMP (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmp`] module*/
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CmpSpec>;
///no description available
pub mod cmp;
/**RLD (rw) register accessor: Reload register

You can [`read`](crate::Reg::read) this register and get [`rld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rld`] module*/
#[doc(alias = "RLD")]
pub type Rld = crate::Reg<rld::RldSpec>;
///Reload register
pub mod rld;
/**CNTUPTVAL (rw) register accessor: Counter update value register

You can [`read`](crate::Reg::read) this register and get [`cntuptval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntuptval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cntuptval`] module*/
#[doc(alias = "CNTUPTVAL")]
pub type Cntuptval = crate::Reg<cntuptval::CntuptvalSpec>;
///Counter update value register
pub mod cntuptval;
/**CAPPOS (rw) register accessor: Capture rising edge register

You can [`read`](crate::Reg::read) this register and get [`cappos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cappos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cappos`] module*/
#[doc(alias = "CAPPOS")]
pub type Cappos = crate::Reg<cappos::CapposSpec>;
///Capture rising edge register
pub mod cappos;
/**CAPNEG (rw) register accessor: Capture falling edge register

You can [`read`](crate::Reg::read) this register and get [`capneg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capneg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@capneg`] module*/
#[doc(alias = "CAPNEG")]
pub type Capneg = crate::Reg<capneg::CapnegSpec>;
///Capture falling edge register
pub mod capneg;
/**CAPPRD (rw) register accessor: PWM period measure register

You can [`read`](crate::Reg::read) this register and get [`capprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@capprd`] module*/
#[doc(alias = "CAPPRD")]
pub type Capprd = crate::Reg<capprd::CapprdSpec>;
///PWM period measure register
pub mod capprd;
/**CAPDTY (rw) register accessor: PWM duty cycle measure register

You can [`read`](crate::Reg::read) this register and get [`capdty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capdty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@capdty`] module*/
#[doc(alias = "CAPDTY")]
pub type Capdty = crate::Reg<capdty::CapdtySpec>;
///PWM duty cycle measure register
pub mod capdty;
/**CNT (rw) register accessor: Counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cnt`] module*/
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
///Counter
pub mod cnt;
