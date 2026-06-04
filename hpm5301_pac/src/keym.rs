#[repr(C)]
///Register block
pub struct RegisterBlock {
    softmkey: [Softmkey; 8],
    softpkey: [Softpkey; 8],
    sec_key_ctl: SecKeyCtl,
    nsc_key_ctl: NscKeyCtl,
    rng: Rng,
    read_control: ReadControl,
}
impl RegisterBlock {
    ///0x00..0x20 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SOFTMKEYSFK0` register.</div>
    #[inline(always)]
    pub const fn softmkey(&self, n: usize) -> &Softmkey {
        &self.softmkey[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - no description available
    #[inline(always)]
    pub fn softmkey_iter(&self) -> impl Iterator<Item = &Softmkey> {
        self.softmkey.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn softmkeysfk0(&self) -> &Softmkey {
        self.softmkey(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn softmkeysfk1(&self) -> &Softmkey {
        self.softmkey(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn softmkeysfk2(&self) -> &Softmkey {
        self.softmkey(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn softmkeysfk3(&self) -> &Softmkey {
        self.softmkey(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn softmkeysfk4(&self) -> &Softmkey {
        self.softmkey(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn softmkeysfk5(&self) -> &Softmkey {
        self.softmkey(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn softmkeysfk6(&self) -> &Softmkey {
        self.softmkey(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn softmkeysfk7(&self) -> &Softmkey {
        self.softmkey(7)
    }
    ///0x20..0x40 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SOFTPKEYSPK0` register.</div>
    #[inline(always)]
    pub const fn softpkey(&self, n: usize) -> &Softpkey {
        &self.softpkey[n]
    }
    ///Iterator for array of:
    ///0x20..0x40 - no description available
    #[inline(always)]
    pub fn softpkey_iter(&self) -> impl Iterator<Item = &Softpkey> {
        self.softpkey.iter()
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn softpkeyspk0(&self) -> &Softpkey {
        self.softpkey(0)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn softpkeyspk1(&self) -> &Softpkey {
        self.softpkey(1)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn softpkeyspk2(&self) -> &Softpkey {
        self.softpkey(2)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn softpkeyspk3(&self) -> &Softpkey {
        self.softpkey(3)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn softpkeyspk4(&self) -> &Softpkey {
        self.softpkey(4)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn softpkeyspk5(&self) -> &Softpkey {
        self.softpkey(5)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn softpkeyspk6(&self) -> &Softpkey {
        self.softpkey(6)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn softpkeyspk7(&self) -> &Softpkey {
        self.softpkey(7)
    }
    ///0x40 - secure key generation
    #[inline(always)]
    pub const fn sec_key_ctl(&self) -> &SecKeyCtl {
        &self.sec_key_ctl
    }
    ///0x44 - non-secure key generation
    #[inline(always)]
    pub const fn nsc_key_ctl(&self) -> &NscKeyCtl {
        &self.nsc_key_ctl
    }
    ///0x48 - Random number interface behavior
    #[inline(always)]
    pub const fn rng(&self) -> &Rng {
        &self.rng
    }
    ///0x4c - key read out control
    #[inline(always)]
    pub const fn read_control(&self) -> &ReadControl {
        &self.read_control
    }
}
/**SOFTMKEY (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`softmkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softmkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@softmkey`] module*/
#[doc(alias = "SOFTMKEY")]
pub type Softmkey = crate::Reg<softmkey::SoftmkeySpec>;
///no description available
pub mod softmkey;
/**SOFTPKEY (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`softpkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softpkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@softpkey`] module*/
#[doc(alias = "SOFTPKEY")]
pub type Softpkey = crate::Reg<softpkey::SoftpkeySpec>;
///no description available
pub mod softpkey;
/**SEC_KEY_CTL (rw) register accessor: secure key generation

You can [`read`](crate::Reg::read) this register and get [`sec_key_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_key_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_key_ctl`] module*/
#[doc(alias = "SEC_KEY_CTL")]
pub type SecKeyCtl = crate::Reg<sec_key_ctl::SecKeyCtlSpec>;
///secure key generation
pub mod sec_key_ctl;
/**NSC_KEY_CTL (rw) register accessor: non-secure key generation

You can [`read`](crate::Reg::read) this register and get [`nsc_key_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsc_key_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nsc_key_ctl`] module*/
#[doc(alias = "NSC_KEY_CTL")]
pub type NscKeyCtl = crate::Reg<nsc_key_ctl::NscKeyCtlSpec>;
///non-secure key generation
pub mod nsc_key_ctl;
/**RNG (rw) register accessor: Random number interface behavior

You can [`read`](crate::Reg::read) this register and get [`rng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rng`] module*/
#[doc(alias = "RNG")]
pub type Rng = crate::Reg<rng::RngSpec>;
///Random number interface behavior
pub mod rng;
/**READ_CONTROL (rw) register accessor: key read out control

You can [`read`](crate::Reg::read) this register and get [`read_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@read_control`] module*/
#[doc(alias = "READ_CONTROL")]
pub type ReadControl = crate::Reg<read_control::ReadControlSpec>;
///key read out control
pub mod read_control;
