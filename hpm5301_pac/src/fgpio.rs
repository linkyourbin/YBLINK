#[repr(C)]
///Register block
pub struct RegisterBlock {
    di: (),
    _reserved1: [u8; 0x0100],
    do_: [Do; 15],
    _reserved2: [u8; 0x10],
    oe: [Oe; 15],
    _reserved3: [u8; 0x10],
    if_: (),
    _reserved4: [u8; 0x0100],
    ie: [Ie; 15],
    _reserved5: [u8; 0x10],
    pl: [Pl; 15],
    _reserved6: [u8; 0x10],
    tp: [Tp; 15],
    _reserved7: [u8; 0x10],
    as_: [As; 15],
    _reserved8: [u8; 0x10],
    pd: [Pd; 15],
}
impl RegisterBlock {
    ///0x00..0x3c - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `DIgpioa` cluster.</div>
    #[inline(always)]
    pub const fn di(&self, n: usize) -> &Di {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x3c - no description available
    #[inline(always)]
    pub fn di_iter(&self) -> impl Iterator<Item = &Di> {
        (0..15).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() })
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn digpioa(&self) -> &Di {
        self.di(0)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn digpiob(&self) -> &Di {
        self.di(1)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn dirsv2(&self) -> &Di {
        self.di(2)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn dirsv3(&self) -> &Di {
        self.di(3)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn dirsv4(&self) -> &Di {
        self.di(4)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn dirsv5(&self) -> &Di {
        self.di(5)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn dirsv6(&self) -> &Di {
        self.di(6)
    }
    ///0x70 - no description available
    #[inline(always)]
    pub const fn dirsv7(&self) -> &Di {
        self.di(7)
    }
    ///0x80 - no description available
    #[inline(always)]
    pub const fn dirsv8(&self) -> &Di {
        self.di(8)
    }
    ///0x90 - no description available
    #[inline(always)]
    pub const fn dirsv9(&self) -> &Di {
        self.di(9)
    }
    ///0xa0 - no description available
    #[inline(always)]
    pub const fn dirsv10(&self) -> &Di {
        self.di(10)
    }
    ///0xb0 - no description available
    #[inline(always)]
    pub const fn dirsv11(&self) -> &Di {
        self.di(11)
    }
    ///0xc0 - no description available
    #[inline(always)]
    pub const fn dirsv12(&self) -> &Di {
        self.di(12)
    }
    ///0xd0 - no description available
    #[inline(always)]
    pub const fn digpiox(&self) -> &Di {
        self.di(13)
    }
    ///0xe0 - no description available
    #[inline(always)]
    pub const fn digpioy(&self) -> &Di {
        self.di(14)
    }
    ///0x100..0x1f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `DOgpioa` cluster.</div>
    #[inline(always)]
    pub const fn do_(&self, n: usize) -> &Do {
        &self.do_[n]
    }
    ///Iterator for array of:
    ///0x100..0x1f0 - no description available
    #[inline(always)]
    pub fn do__iter(&self) -> impl Iterator<Item = &Do> {
        self.do_.iter()
    }
    ///0x100..0x110 - no description available
    #[inline(always)]
    pub const fn dogpioa(&self) -> &Do {
        self.do_(0)
    }
    ///0x110..0x120 - no description available
    #[inline(always)]
    pub const fn dogpiob(&self) -> &Do {
        self.do_(1)
    }
    ///0x120..0x130 - no description available
    #[inline(always)]
    pub const fn dorsv2(&self) -> &Do {
        self.do_(2)
    }
    ///0x130..0x140 - no description available
    #[inline(always)]
    pub const fn dorsv3(&self) -> &Do {
        self.do_(3)
    }
    ///0x140..0x150 - no description available
    #[inline(always)]
    pub const fn dorsv4(&self) -> &Do {
        self.do_(4)
    }
    ///0x150..0x160 - no description available
    #[inline(always)]
    pub const fn dorsv5(&self) -> &Do {
        self.do_(5)
    }
    ///0x160..0x170 - no description available
    #[inline(always)]
    pub const fn dorsv6(&self) -> &Do {
        self.do_(6)
    }
    ///0x170..0x180 - no description available
    #[inline(always)]
    pub const fn dorsv7(&self) -> &Do {
        self.do_(7)
    }
    ///0x180..0x190 - no description available
    #[inline(always)]
    pub const fn dorsv8(&self) -> &Do {
        self.do_(8)
    }
    ///0x190..0x1a0 - no description available
    #[inline(always)]
    pub const fn dorsv9(&self) -> &Do {
        self.do_(9)
    }
    ///0x1a0..0x1b0 - no description available
    #[inline(always)]
    pub const fn dorsv10(&self) -> &Do {
        self.do_(10)
    }
    ///0x1b0..0x1c0 - no description available
    #[inline(always)]
    pub const fn dorsv11(&self) -> &Do {
        self.do_(11)
    }
    ///0x1c0..0x1d0 - no description available
    #[inline(always)]
    pub const fn dorsv12(&self) -> &Do {
        self.do_(12)
    }
    ///0x1d0..0x1e0 - no description available
    #[inline(always)]
    pub const fn dogpiox(&self) -> &Do {
        self.do_(13)
    }
    ///0x1e0..0x1f0 - no description available
    #[inline(always)]
    pub const fn dogpioy(&self) -> &Do {
        self.do_(14)
    }
    ///0x200..0x2f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `OEgpioa` cluster.</div>
    #[inline(always)]
    pub const fn oe(&self, n: usize) -> &Oe {
        &self.oe[n]
    }
    ///Iterator for array of:
    ///0x200..0x2f0 - no description available
    #[inline(always)]
    pub fn oe_iter(&self) -> impl Iterator<Item = &Oe> {
        self.oe.iter()
    }
    ///0x200..0x210 - no description available
    #[inline(always)]
    pub const fn oegpioa(&self) -> &Oe {
        self.oe(0)
    }
    ///0x210..0x220 - no description available
    #[inline(always)]
    pub const fn oegpiob(&self) -> &Oe {
        self.oe(1)
    }
    ///0x220..0x230 - no description available
    #[inline(always)]
    pub const fn oersv2(&self) -> &Oe {
        self.oe(2)
    }
    ///0x230..0x240 - no description available
    #[inline(always)]
    pub const fn oersv3(&self) -> &Oe {
        self.oe(3)
    }
    ///0x240..0x250 - no description available
    #[inline(always)]
    pub const fn oersv4(&self) -> &Oe {
        self.oe(4)
    }
    ///0x250..0x260 - no description available
    #[inline(always)]
    pub const fn oersv5(&self) -> &Oe {
        self.oe(5)
    }
    ///0x260..0x270 - no description available
    #[inline(always)]
    pub const fn oersv6(&self) -> &Oe {
        self.oe(6)
    }
    ///0x270..0x280 - no description available
    #[inline(always)]
    pub const fn oersv7(&self) -> &Oe {
        self.oe(7)
    }
    ///0x280..0x290 - no description available
    #[inline(always)]
    pub const fn oersv8(&self) -> &Oe {
        self.oe(8)
    }
    ///0x290..0x2a0 - no description available
    #[inline(always)]
    pub const fn oersv9(&self) -> &Oe {
        self.oe(9)
    }
    ///0x2a0..0x2b0 - no description available
    #[inline(always)]
    pub const fn oersv10(&self) -> &Oe {
        self.oe(10)
    }
    ///0x2b0..0x2c0 - no description available
    #[inline(always)]
    pub const fn oersv11(&self) -> &Oe {
        self.oe(11)
    }
    ///0x2c0..0x2d0 - no description available
    #[inline(always)]
    pub const fn oersv12(&self) -> &Oe {
        self.oe(12)
    }
    ///0x2d0..0x2e0 - no description available
    #[inline(always)]
    pub const fn oegpiox(&self) -> &Oe {
        self.oe(13)
    }
    ///0x2e0..0x2f0 - no description available
    #[inline(always)]
    pub const fn oegpioy(&self) -> &Oe {
        self.oe(14)
    }
    ///0x300..0x33c - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `IFgpioa` cluster.</div>
    #[inline(always)]
    pub const fn if_(&self, n: usize) -> &If {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(768)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x300..0x33c - no description available
    #[inline(always)]
    pub fn if__iter(&self) -> impl Iterator<Item = &If> {
        (0..15).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(768)
                .add(16 * n)
                .cast()
        })
    }
    ///0x300 - no description available
    #[inline(always)]
    pub const fn ifgpioa(&self) -> &If {
        self.if_(0)
    }
    ///0x310 - no description available
    #[inline(always)]
    pub const fn ifgpiob(&self) -> &If {
        self.if_(1)
    }
    ///0x320 - no description available
    #[inline(always)]
    pub const fn ifrsv2(&self) -> &If {
        self.if_(2)
    }
    ///0x330 - no description available
    #[inline(always)]
    pub const fn ifrsv3(&self) -> &If {
        self.if_(3)
    }
    ///0x340 - no description available
    #[inline(always)]
    pub const fn ifrsv4(&self) -> &If {
        self.if_(4)
    }
    ///0x350 - no description available
    #[inline(always)]
    pub const fn ifrsv5(&self) -> &If {
        self.if_(5)
    }
    ///0x360 - no description available
    #[inline(always)]
    pub const fn ifrsv6(&self) -> &If {
        self.if_(6)
    }
    ///0x370 - no description available
    #[inline(always)]
    pub const fn ifrsv7(&self) -> &If {
        self.if_(7)
    }
    ///0x380 - no description available
    #[inline(always)]
    pub const fn ifrsv8(&self) -> &If {
        self.if_(8)
    }
    ///0x390 - no description available
    #[inline(always)]
    pub const fn ifrsv9(&self) -> &If {
        self.if_(9)
    }
    ///0x3a0 - no description available
    #[inline(always)]
    pub const fn ifrsv10(&self) -> &If {
        self.if_(10)
    }
    ///0x3b0 - no description available
    #[inline(always)]
    pub const fn ifrsv11(&self) -> &If {
        self.if_(11)
    }
    ///0x3c0 - no description available
    #[inline(always)]
    pub const fn ifrsv12(&self) -> &If {
        self.if_(12)
    }
    ///0x3d0 - no description available
    #[inline(always)]
    pub const fn ifgpiox(&self) -> &If {
        self.if_(13)
    }
    ///0x3e0 - no description available
    #[inline(always)]
    pub const fn ifgpioy(&self) -> &If {
        self.if_(14)
    }
    ///0x400..0x4f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `IEgpioa` cluster.</div>
    #[inline(always)]
    pub const fn ie(&self, n: usize) -> &Ie {
        &self.ie[n]
    }
    ///Iterator for array of:
    ///0x400..0x4f0 - no description available
    #[inline(always)]
    pub fn ie_iter(&self) -> impl Iterator<Item = &Ie> {
        self.ie.iter()
    }
    ///0x400..0x410 - no description available
    #[inline(always)]
    pub const fn iegpioa(&self) -> &Ie {
        self.ie(0)
    }
    ///0x410..0x420 - no description available
    #[inline(always)]
    pub const fn iegpiob(&self) -> &Ie {
        self.ie(1)
    }
    ///0x420..0x430 - no description available
    #[inline(always)]
    pub const fn iersv2(&self) -> &Ie {
        self.ie(2)
    }
    ///0x430..0x440 - no description available
    #[inline(always)]
    pub const fn iersv3(&self) -> &Ie {
        self.ie(3)
    }
    ///0x440..0x450 - no description available
    #[inline(always)]
    pub const fn iersv4(&self) -> &Ie {
        self.ie(4)
    }
    ///0x450..0x460 - no description available
    #[inline(always)]
    pub const fn iersv5(&self) -> &Ie {
        self.ie(5)
    }
    ///0x460..0x470 - no description available
    #[inline(always)]
    pub const fn iersv6(&self) -> &Ie {
        self.ie(6)
    }
    ///0x470..0x480 - no description available
    #[inline(always)]
    pub const fn iersv7(&self) -> &Ie {
        self.ie(7)
    }
    ///0x480..0x490 - no description available
    #[inline(always)]
    pub const fn iersv8(&self) -> &Ie {
        self.ie(8)
    }
    ///0x490..0x4a0 - no description available
    #[inline(always)]
    pub const fn iersv9(&self) -> &Ie {
        self.ie(9)
    }
    ///0x4a0..0x4b0 - no description available
    #[inline(always)]
    pub const fn iersv10(&self) -> &Ie {
        self.ie(10)
    }
    ///0x4b0..0x4c0 - no description available
    #[inline(always)]
    pub const fn iersv11(&self) -> &Ie {
        self.ie(11)
    }
    ///0x4c0..0x4d0 - no description available
    #[inline(always)]
    pub const fn iersv12(&self) -> &Ie {
        self.ie(12)
    }
    ///0x4d0..0x4e0 - no description available
    #[inline(always)]
    pub const fn iegpiox(&self) -> &Ie {
        self.ie(13)
    }
    ///0x4e0..0x4f0 - no description available
    #[inline(always)]
    pub const fn iegpioy(&self) -> &Ie {
        self.ie(14)
    }
    ///0x500..0x5f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `PLgpioa` cluster.</div>
    #[inline(always)]
    pub const fn pl(&self, n: usize) -> &Pl {
        &self.pl[n]
    }
    ///Iterator for array of:
    ///0x500..0x5f0 - no description available
    #[inline(always)]
    pub fn pl_iter(&self) -> impl Iterator<Item = &Pl> {
        self.pl.iter()
    }
    ///0x500..0x510 - no description available
    #[inline(always)]
    pub const fn plgpioa(&self) -> &Pl {
        self.pl(0)
    }
    ///0x510..0x520 - no description available
    #[inline(always)]
    pub const fn plgpiob(&self) -> &Pl {
        self.pl(1)
    }
    ///0x520..0x530 - no description available
    #[inline(always)]
    pub const fn plrsv2(&self) -> &Pl {
        self.pl(2)
    }
    ///0x530..0x540 - no description available
    #[inline(always)]
    pub const fn plrsv3(&self) -> &Pl {
        self.pl(3)
    }
    ///0x540..0x550 - no description available
    #[inline(always)]
    pub const fn plrsv4(&self) -> &Pl {
        self.pl(4)
    }
    ///0x550..0x560 - no description available
    #[inline(always)]
    pub const fn plrsv5(&self) -> &Pl {
        self.pl(5)
    }
    ///0x560..0x570 - no description available
    #[inline(always)]
    pub const fn plrsv6(&self) -> &Pl {
        self.pl(6)
    }
    ///0x570..0x580 - no description available
    #[inline(always)]
    pub const fn plrsv7(&self) -> &Pl {
        self.pl(7)
    }
    ///0x580..0x590 - no description available
    #[inline(always)]
    pub const fn plrsv8(&self) -> &Pl {
        self.pl(8)
    }
    ///0x590..0x5a0 - no description available
    #[inline(always)]
    pub const fn plrsv9(&self) -> &Pl {
        self.pl(9)
    }
    ///0x5a0..0x5b0 - no description available
    #[inline(always)]
    pub const fn plrsv10(&self) -> &Pl {
        self.pl(10)
    }
    ///0x5b0..0x5c0 - no description available
    #[inline(always)]
    pub const fn plrsv11(&self) -> &Pl {
        self.pl(11)
    }
    ///0x5c0..0x5d0 - no description available
    #[inline(always)]
    pub const fn plrsv12(&self) -> &Pl {
        self.pl(12)
    }
    ///0x5d0..0x5e0 - no description available
    #[inline(always)]
    pub const fn plgpiox(&self) -> &Pl {
        self.pl(13)
    }
    ///0x5e0..0x5f0 - no description available
    #[inline(always)]
    pub const fn plgpioy(&self) -> &Pl {
        self.pl(14)
    }
    ///0x600..0x6f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `TPgpioa` cluster.</div>
    #[inline(always)]
    pub const fn tp(&self, n: usize) -> &Tp {
        &self.tp[n]
    }
    ///Iterator for array of:
    ///0x600..0x6f0 - no description available
    #[inline(always)]
    pub fn tp_iter(&self) -> impl Iterator<Item = &Tp> {
        self.tp.iter()
    }
    ///0x600..0x610 - no description available
    #[inline(always)]
    pub const fn tpgpioa(&self) -> &Tp {
        self.tp(0)
    }
    ///0x610..0x620 - no description available
    #[inline(always)]
    pub const fn tpgpiob(&self) -> &Tp {
        self.tp(1)
    }
    ///0x620..0x630 - no description available
    #[inline(always)]
    pub const fn tprsv2(&self) -> &Tp {
        self.tp(2)
    }
    ///0x630..0x640 - no description available
    #[inline(always)]
    pub const fn tprsv3(&self) -> &Tp {
        self.tp(3)
    }
    ///0x640..0x650 - no description available
    #[inline(always)]
    pub const fn tprsv4(&self) -> &Tp {
        self.tp(4)
    }
    ///0x650..0x660 - no description available
    #[inline(always)]
    pub const fn tprsv5(&self) -> &Tp {
        self.tp(5)
    }
    ///0x660..0x670 - no description available
    #[inline(always)]
    pub const fn tprsv6(&self) -> &Tp {
        self.tp(6)
    }
    ///0x670..0x680 - no description available
    #[inline(always)]
    pub const fn tprsv7(&self) -> &Tp {
        self.tp(7)
    }
    ///0x680..0x690 - no description available
    #[inline(always)]
    pub const fn tprsv8(&self) -> &Tp {
        self.tp(8)
    }
    ///0x690..0x6a0 - no description available
    #[inline(always)]
    pub const fn tprsv9(&self) -> &Tp {
        self.tp(9)
    }
    ///0x6a0..0x6b0 - no description available
    #[inline(always)]
    pub const fn tprsv10(&self) -> &Tp {
        self.tp(10)
    }
    ///0x6b0..0x6c0 - no description available
    #[inline(always)]
    pub const fn tprsv11(&self) -> &Tp {
        self.tp(11)
    }
    ///0x6c0..0x6d0 - no description available
    #[inline(always)]
    pub const fn tprsv12(&self) -> &Tp {
        self.tp(12)
    }
    ///0x6d0..0x6e0 - no description available
    #[inline(always)]
    pub const fn tpgpiox(&self) -> &Tp {
        self.tp(13)
    }
    ///0x6e0..0x6f0 - no description available
    #[inline(always)]
    pub const fn tpgpioy(&self) -> &Tp {
        self.tp(14)
    }
    ///0x700..0x7f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `ASgpioa` cluster.</div>
    #[inline(always)]
    pub const fn as_(&self, n: usize) -> &As {
        &self.as_[n]
    }
    ///Iterator for array of:
    ///0x700..0x7f0 - no description available
    #[inline(always)]
    pub fn as__iter(&self) -> impl Iterator<Item = &As> {
        self.as_.iter()
    }
    ///0x700..0x710 - no description available
    #[inline(always)]
    pub const fn asgpioa(&self) -> &As {
        self.as_(0)
    }
    ///0x710..0x720 - no description available
    #[inline(always)]
    pub const fn asgpiob(&self) -> &As {
        self.as_(1)
    }
    ///0x720..0x730 - no description available
    #[inline(always)]
    pub const fn asrsv2(&self) -> &As {
        self.as_(2)
    }
    ///0x730..0x740 - no description available
    #[inline(always)]
    pub const fn asrsv3(&self) -> &As {
        self.as_(3)
    }
    ///0x740..0x750 - no description available
    #[inline(always)]
    pub const fn asrsv4(&self) -> &As {
        self.as_(4)
    }
    ///0x750..0x760 - no description available
    #[inline(always)]
    pub const fn asrsv5(&self) -> &As {
        self.as_(5)
    }
    ///0x760..0x770 - no description available
    #[inline(always)]
    pub const fn asrsv6(&self) -> &As {
        self.as_(6)
    }
    ///0x770..0x780 - no description available
    #[inline(always)]
    pub const fn asrsv7(&self) -> &As {
        self.as_(7)
    }
    ///0x780..0x790 - no description available
    #[inline(always)]
    pub const fn asrsv8(&self) -> &As {
        self.as_(8)
    }
    ///0x790..0x7a0 - no description available
    #[inline(always)]
    pub const fn asrsv9(&self) -> &As {
        self.as_(9)
    }
    ///0x7a0..0x7b0 - no description available
    #[inline(always)]
    pub const fn asrsv10(&self) -> &As {
        self.as_(10)
    }
    ///0x7b0..0x7c0 - no description available
    #[inline(always)]
    pub const fn asrsv11(&self) -> &As {
        self.as_(11)
    }
    ///0x7c0..0x7d0 - no description available
    #[inline(always)]
    pub const fn asrsv12(&self) -> &As {
        self.as_(12)
    }
    ///0x7d0..0x7e0 - no description available
    #[inline(always)]
    pub const fn asgpiox(&self) -> &As {
        self.as_(13)
    }
    ///0x7e0..0x7f0 - no description available
    #[inline(always)]
    pub const fn asgpioy(&self) -> &As {
        self.as_(14)
    }
    ///0x800..0x8f0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `PDgpioa` cluster.</div>
    #[inline(always)]
    pub const fn pd(&self, n: usize) -> &Pd {
        &self.pd[n]
    }
    ///Iterator for array of:
    ///0x800..0x8f0 - no description available
    #[inline(always)]
    pub fn pd_iter(&self) -> impl Iterator<Item = &Pd> {
        self.pd.iter()
    }
    ///0x800..0x810 - no description available
    #[inline(always)]
    pub const fn pdgpioa(&self) -> &Pd {
        self.pd(0)
    }
    ///0x810..0x820 - no description available
    #[inline(always)]
    pub const fn pdgpiob(&self) -> &Pd {
        self.pd(1)
    }
    ///0x820..0x830 - no description available
    #[inline(always)]
    pub const fn pdrsv2(&self) -> &Pd {
        self.pd(2)
    }
    ///0x830..0x840 - no description available
    #[inline(always)]
    pub const fn pdrsv3(&self) -> &Pd {
        self.pd(3)
    }
    ///0x840..0x850 - no description available
    #[inline(always)]
    pub const fn pdrsv4(&self) -> &Pd {
        self.pd(4)
    }
    ///0x850..0x860 - no description available
    #[inline(always)]
    pub const fn pdrsv5(&self) -> &Pd {
        self.pd(5)
    }
    ///0x860..0x870 - no description available
    #[inline(always)]
    pub const fn pdrsv6(&self) -> &Pd {
        self.pd(6)
    }
    ///0x870..0x880 - no description available
    #[inline(always)]
    pub const fn pdrsv7(&self) -> &Pd {
        self.pd(7)
    }
    ///0x880..0x890 - no description available
    #[inline(always)]
    pub const fn pdrsv8(&self) -> &Pd {
        self.pd(8)
    }
    ///0x890..0x8a0 - no description available
    #[inline(always)]
    pub const fn pdrsv9(&self) -> &Pd {
        self.pd(9)
    }
    ///0x8a0..0x8b0 - no description available
    #[inline(always)]
    pub const fn pdrsv10(&self) -> &Pd {
        self.pd(10)
    }
    ///0x8b0..0x8c0 - no description available
    #[inline(always)]
    pub const fn pdrsv11(&self) -> &Pd {
        self.pd(11)
    }
    ///0x8c0..0x8d0 - no description available
    #[inline(always)]
    pub const fn pdrsv12(&self) -> &Pd {
        self.pd(12)
    }
    ///0x8d0..0x8e0 - no description available
    #[inline(always)]
    pub const fn pdgpiox(&self) -> &Pd {
        self.pd(13)
    }
    ///0x8e0..0x8f0 - no description available
    #[inline(always)]
    pub const fn pdgpioy(&self) -> &Pd {
        self.pd(14)
    }
}
///no description available
pub use self::di::Di;
///Cluster
///no description available
pub mod di;
///no description available
pub use self::do_::Do;
///Cluster
///no description available
pub mod do_;
///no description available
pub use self::oe::Oe;
///Cluster
///no description available
pub mod oe;
///no description available
pub use self::if_::If;
///Cluster
///no description available
pub mod if_;
///no description available
pub use self::ie::Ie;
///Cluster
///no description available
pub mod ie;
///no description available
pub use self::pl::Pl;
///Cluster
///no description available
pub mod pl;
///no description available
pub use self::tp::Tp;
///Cluster
///no description available
pub mod tp;
///no description available
pub use self::as_::As;
///Cluster
///no description available
pub mod as_;
///no description available
pub use self::pd::Pd;
///Cluster
///no description available
pub mod pd;
