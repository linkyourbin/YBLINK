#[repr(C)]
///no description available
#[doc(alias = "ASSIGN")]
pub struct Assign {
    pin: [Pin; 32],
}
impl Assign {
    ///0x00..0x80 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PINPIN00` register.</div>
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &Pin {
        &self.pin[n]
    }
    ///Iterator for array of:
    ///0x00..0x80 - no description available
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &Pin> {
        self.pin.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn pinpin00(&self) -> &Pin {
        self.pin(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn pinpin01(&self) -> &Pin {
        self.pin(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn pinpin02(&self) -> &Pin {
        self.pin(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn pinpin03(&self) -> &Pin {
        self.pin(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn pinpin04(&self) -> &Pin {
        self.pin(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn pinpin05(&self) -> &Pin {
        self.pin(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn pinpin06(&self) -> &Pin {
        self.pin(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn pinpin07(&self) -> &Pin {
        self.pin(7)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn pinpin08(&self) -> &Pin {
        self.pin(8)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn pinpin09(&self) -> &Pin {
        self.pin(9)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn pinpin10(&self) -> &Pin {
        self.pin(10)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn pinpin11(&self) -> &Pin {
        self.pin(11)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn pinpin12(&self) -> &Pin {
        self.pin(12)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn pinpin13(&self) -> &Pin {
        self.pin(13)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn pinpin14(&self) -> &Pin {
        self.pin(14)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn pinpin15(&self) -> &Pin {
        self.pin(15)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn pinpin16(&self) -> &Pin {
        self.pin(16)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn pinpin17(&self) -> &Pin {
        self.pin(17)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn pinpin18(&self) -> &Pin {
        self.pin(18)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn pinpin19(&self) -> &Pin {
        self.pin(19)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn pinpin20(&self) -> &Pin {
        self.pin(20)
    }
    ///0x54 - no description available
    #[inline(always)]
    pub const fn pinpin21(&self) -> &Pin {
        self.pin(21)
    }
    ///0x58 - no description available
    #[inline(always)]
    pub const fn pinpin22(&self) -> &Pin {
        self.pin(22)
    }
    ///0x5c - no description available
    #[inline(always)]
    pub const fn pinpin23(&self) -> &Pin {
        self.pin(23)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn pinpin24(&self) -> &Pin {
        self.pin(24)
    }
    ///0x64 - no description available
    #[inline(always)]
    pub const fn pinpin25(&self) -> &Pin {
        self.pin(25)
    }
    ///0x68 - no description available
    #[inline(always)]
    pub const fn pinpin26(&self) -> &Pin {
        self.pin(26)
    }
    ///0x6c - no description available
    #[inline(always)]
    pub const fn pinpin27(&self) -> &Pin {
        self.pin(27)
    }
    ///0x70 - no description available
    #[inline(always)]
    pub const fn pinpin28(&self) -> &Pin {
        self.pin(28)
    }
    ///0x74 - no description available
    #[inline(always)]
    pub const fn pinpin29(&self) -> &Pin {
        self.pin(29)
    }
    ///0x78 - no description available
    #[inline(always)]
    pub const fn pinpin30(&self) -> &Pin {
        self.pin(30)
    }
    ///0x7c - no description available
    #[inline(always)]
    pub const fn pinpin31(&self) -> &Pin {
        self.pin(31)
    }
}
/**PIN (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin`] module*/
#[doc(alias = "PIN")]
pub type Pin = crate::Reg<pin::PinSpec>;
///no description available
pub mod pin;
