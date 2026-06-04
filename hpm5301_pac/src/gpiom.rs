#[repr(C)]
///Register block
pub struct RegisterBlock {
    assign: [Assign; 15],
}
impl RegisterBlock {
    ///0x00..0x780 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `ASSIGNgpioa` cluster.</div>
    #[inline(always)]
    pub const fn assign(&self, n: usize) -> &Assign {
        &self.assign[n]
    }
    ///Iterator for array of:
    ///0x00..0x780 - no description available
    #[inline(always)]
    pub fn assign_iter(&self) -> impl Iterator<Item = &Assign> {
        self.assign.iter()
    }
    ///0x00..0x80 - no description available
    #[inline(always)]
    pub const fn assigngpioa(&self) -> &Assign {
        self.assign(0)
    }
    ///0x80..0x100 - no description available
    #[inline(always)]
    pub const fn assigngpiob(&self) -> &Assign {
        self.assign(1)
    }
    ///0x100..0x180 - no description available
    #[inline(always)]
    pub const fn assignrsv2(&self) -> &Assign {
        self.assign(2)
    }
    ///0x180..0x200 - no description available
    #[inline(always)]
    pub const fn assignrsv3(&self) -> &Assign {
        self.assign(3)
    }
    ///0x200..0x280 - no description available
    #[inline(always)]
    pub const fn assignrsv4(&self) -> &Assign {
        self.assign(4)
    }
    ///0x280..0x300 - no description available
    #[inline(always)]
    pub const fn assignrsv5(&self) -> &Assign {
        self.assign(5)
    }
    ///0x300..0x380 - no description available
    #[inline(always)]
    pub const fn assignrsv6(&self) -> &Assign {
        self.assign(6)
    }
    ///0x380..0x400 - no description available
    #[inline(always)]
    pub const fn assignrsv7(&self) -> &Assign {
        self.assign(7)
    }
    ///0x400..0x480 - no description available
    #[inline(always)]
    pub const fn assignrsv8(&self) -> &Assign {
        self.assign(8)
    }
    ///0x480..0x500 - no description available
    #[inline(always)]
    pub const fn assignrsv9(&self) -> &Assign {
        self.assign(9)
    }
    ///0x500..0x580 - no description available
    #[inline(always)]
    pub const fn assignrsv10(&self) -> &Assign {
        self.assign(10)
    }
    ///0x580..0x600 - no description available
    #[inline(always)]
    pub const fn assignrsv11(&self) -> &Assign {
        self.assign(11)
    }
    ///0x600..0x680 - no description available
    #[inline(always)]
    pub const fn assignrsv12(&self) -> &Assign {
        self.assign(12)
    }
    ///0x680..0x700 - no description available
    #[inline(always)]
    pub const fn assigngpiox(&self) -> &Assign {
        self.assign(13)
    }
    ///0x700..0x780 - no description available
    #[inline(always)]
    pub const fn assigngpioy(&self) -> &Assign {
        self.assign(14)
    }
}
///no description available
pub use self::assign::Assign;
///Cluster
///no description available
pub mod assign;
