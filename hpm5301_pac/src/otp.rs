#[repr(C)]
///Register block
pub struct RegisterBlock {
    shadow: [Shadow; 128],
    shadow_lock: [ShadowLock; 8],
    _reserved2: [u8; 0x01e0],
    fuse: [Fuse; 128],
    fuse_lock: [FuseLock; 8],
    _reserved4: [u8; 0x01e0],
    unlock: Unlock,
    data: Data,
    addr: Addr,
    cmd: Cmd,
    _reserved8: [u8; 0x01f0],
    load_req: LoadReq,
    load_comp: LoadComp,
    _reserved10: [u8; 0x18],
    region: [Region; 4],
    _reserved11: [u8; 0x01d0],
    int_flag: IntFlag,
    int_en: IntEn,
}
impl RegisterBlock {
    ///0x00..0x200 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SHADOWSHADOW000` register.</div>
    #[inline(always)]
    pub const fn shadow(&self, n: usize) -> &Shadow {
        &self.shadow[n]
    }
    ///Iterator for array of:
    ///0x00..0x200 - no description available
    #[inline(always)]
    pub fn shadow_iter(&self) -> impl Iterator<Item = &Shadow> {
        self.shadow.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn shadowshadow000(&self) -> &Shadow {
        self.shadow(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn shadowshadow001(&self) -> &Shadow {
        self.shadow(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn shadowshadow002(&self) -> &Shadow {
        self.shadow(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn shadowshadow003(&self) -> &Shadow {
        self.shadow(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn shadowshadow004(&self) -> &Shadow {
        self.shadow(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn shadowshadow005(&self) -> &Shadow {
        self.shadow(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn shadowshadow006(&self) -> &Shadow {
        self.shadow(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn shadowshadow007(&self) -> &Shadow {
        self.shadow(7)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn shadowshadow008(&self) -> &Shadow {
        self.shadow(8)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn shadowshadow009(&self) -> &Shadow {
        self.shadow(9)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn shadowshadow010(&self) -> &Shadow {
        self.shadow(10)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn shadowshadow011(&self) -> &Shadow {
        self.shadow(11)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn shadowshadow012(&self) -> &Shadow {
        self.shadow(12)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn shadowshadow013(&self) -> &Shadow {
        self.shadow(13)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn shadowshadow014(&self) -> &Shadow {
        self.shadow(14)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn shadowshadow015(&self) -> &Shadow {
        self.shadow(15)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn shadowshadow016(&self) -> &Shadow {
        self.shadow(16)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn shadowshadow017(&self) -> &Shadow {
        self.shadow(17)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn shadowshadow018(&self) -> &Shadow {
        self.shadow(18)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn shadowshadow019(&self) -> &Shadow {
        self.shadow(19)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn shadowshadow020(&self) -> &Shadow {
        self.shadow(20)
    }
    ///0x54 - no description available
    #[inline(always)]
    pub const fn shadowshadow021(&self) -> &Shadow {
        self.shadow(21)
    }
    ///0x58 - no description available
    #[inline(always)]
    pub const fn shadowshadow022(&self) -> &Shadow {
        self.shadow(22)
    }
    ///0x5c - no description available
    #[inline(always)]
    pub const fn shadowshadow023(&self) -> &Shadow {
        self.shadow(23)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn shadowshadow024(&self) -> &Shadow {
        self.shadow(24)
    }
    ///0x64 - no description available
    #[inline(always)]
    pub const fn shadowshadow025(&self) -> &Shadow {
        self.shadow(25)
    }
    ///0x68 - no description available
    #[inline(always)]
    pub const fn shadowshadow026(&self) -> &Shadow {
        self.shadow(26)
    }
    ///0x6c - no description available
    #[inline(always)]
    pub const fn shadowshadow027(&self) -> &Shadow {
        self.shadow(27)
    }
    ///0x70 - no description available
    #[inline(always)]
    pub const fn shadowshadow028(&self) -> &Shadow {
        self.shadow(28)
    }
    ///0x74 - no description available
    #[inline(always)]
    pub const fn shadowshadow029(&self) -> &Shadow {
        self.shadow(29)
    }
    ///0x78 - no description available
    #[inline(always)]
    pub const fn shadowshadow030(&self) -> &Shadow {
        self.shadow(30)
    }
    ///0x7c - no description available
    #[inline(always)]
    pub const fn shadowshadow031(&self) -> &Shadow {
        self.shadow(31)
    }
    ///0x80 - no description available
    #[inline(always)]
    pub const fn shadowshadow032(&self) -> &Shadow {
        self.shadow(32)
    }
    ///0x84 - no description available
    #[inline(always)]
    pub const fn shadowshadow033(&self) -> &Shadow {
        self.shadow(33)
    }
    ///0x88 - no description available
    #[inline(always)]
    pub const fn shadowshadow034(&self) -> &Shadow {
        self.shadow(34)
    }
    ///0x8c - no description available
    #[inline(always)]
    pub const fn shadowshadow035(&self) -> &Shadow {
        self.shadow(35)
    }
    ///0x90 - no description available
    #[inline(always)]
    pub const fn shadowshadow036(&self) -> &Shadow {
        self.shadow(36)
    }
    ///0x94 - no description available
    #[inline(always)]
    pub const fn shadowshadow037(&self) -> &Shadow {
        self.shadow(37)
    }
    ///0x98 - no description available
    #[inline(always)]
    pub const fn shadowshadow038(&self) -> &Shadow {
        self.shadow(38)
    }
    ///0x9c - no description available
    #[inline(always)]
    pub const fn shadowshadow039(&self) -> &Shadow {
        self.shadow(39)
    }
    ///0xa0 - no description available
    #[inline(always)]
    pub const fn shadowshadow040(&self) -> &Shadow {
        self.shadow(40)
    }
    ///0xa4 - no description available
    #[inline(always)]
    pub const fn shadowshadow041(&self) -> &Shadow {
        self.shadow(41)
    }
    ///0xa8 - no description available
    #[inline(always)]
    pub const fn shadowshadow042(&self) -> &Shadow {
        self.shadow(42)
    }
    ///0xac - no description available
    #[inline(always)]
    pub const fn shadowshadow043(&self) -> &Shadow {
        self.shadow(43)
    }
    ///0xb0 - no description available
    #[inline(always)]
    pub const fn shadowshadow044(&self) -> &Shadow {
        self.shadow(44)
    }
    ///0xb4 - no description available
    #[inline(always)]
    pub const fn shadowshadow045(&self) -> &Shadow {
        self.shadow(45)
    }
    ///0xb8 - no description available
    #[inline(always)]
    pub const fn shadowshadow046(&self) -> &Shadow {
        self.shadow(46)
    }
    ///0xbc - no description available
    #[inline(always)]
    pub const fn shadowshadow047(&self) -> &Shadow {
        self.shadow(47)
    }
    ///0xc0 - no description available
    #[inline(always)]
    pub const fn shadowshadow048(&self) -> &Shadow {
        self.shadow(48)
    }
    ///0xc4 - no description available
    #[inline(always)]
    pub const fn shadowshadow049(&self) -> &Shadow {
        self.shadow(49)
    }
    ///0xc8 - no description available
    #[inline(always)]
    pub const fn shadowshadow050(&self) -> &Shadow {
        self.shadow(50)
    }
    ///0xcc - no description available
    #[inline(always)]
    pub const fn shadowshadow051(&self) -> &Shadow {
        self.shadow(51)
    }
    ///0xd0 - no description available
    #[inline(always)]
    pub const fn shadowshadow052(&self) -> &Shadow {
        self.shadow(52)
    }
    ///0xd4 - no description available
    #[inline(always)]
    pub const fn shadowshadow053(&self) -> &Shadow {
        self.shadow(53)
    }
    ///0xd8 - no description available
    #[inline(always)]
    pub const fn shadowshadow054(&self) -> &Shadow {
        self.shadow(54)
    }
    ///0xdc - no description available
    #[inline(always)]
    pub const fn shadowshadow055(&self) -> &Shadow {
        self.shadow(55)
    }
    ///0xe0 - no description available
    #[inline(always)]
    pub const fn shadowshadow056(&self) -> &Shadow {
        self.shadow(56)
    }
    ///0xe4 - no description available
    #[inline(always)]
    pub const fn shadowshadow057(&self) -> &Shadow {
        self.shadow(57)
    }
    ///0xe8 - no description available
    #[inline(always)]
    pub const fn shadowshadow058(&self) -> &Shadow {
        self.shadow(58)
    }
    ///0xec - no description available
    #[inline(always)]
    pub const fn shadowshadow059(&self) -> &Shadow {
        self.shadow(59)
    }
    ///0xf0 - no description available
    #[inline(always)]
    pub const fn shadowshadow060(&self) -> &Shadow {
        self.shadow(60)
    }
    ///0xf4 - no description available
    #[inline(always)]
    pub const fn shadowshadow061(&self) -> &Shadow {
        self.shadow(61)
    }
    ///0xf8 - no description available
    #[inline(always)]
    pub const fn shadowshadow062(&self) -> &Shadow {
        self.shadow(62)
    }
    ///0xfc - no description available
    #[inline(always)]
    pub const fn shadowshadow063(&self) -> &Shadow {
        self.shadow(63)
    }
    ///0x100 - no description available
    #[inline(always)]
    pub const fn shadowshadow064(&self) -> &Shadow {
        self.shadow(64)
    }
    ///0x104 - no description available
    #[inline(always)]
    pub const fn shadowshadow065(&self) -> &Shadow {
        self.shadow(65)
    }
    ///0x108 - no description available
    #[inline(always)]
    pub const fn shadowshadow066(&self) -> &Shadow {
        self.shadow(66)
    }
    ///0x10c - no description available
    #[inline(always)]
    pub const fn shadowshadow067(&self) -> &Shadow {
        self.shadow(67)
    }
    ///0x110 - no description available
    #[inline(always)]
    pub const fn shadowshadow068(&self) -> &Shadow {
        self.shadow(68)
    }
    ///0x114 - no description available
    #[inline(always)]
    pub const fn shadowshadow069(&self) -> &Shadow {
        self.shadow(69)
    }
    ///0x118 - no description available
    #[inline(always)]
    pub const fn shadowshadow070(&self) -> &Shadow {
        self.shadow(70)
    }
    ///0x11c - no description available
    #[inline(always)]
    pub const fn shadowshadow071(&self) -> &Shadow {
        self.shadow(71)
    }
    ///0x120 - no description available
    #[inline(always)]
    pub const fn shadowshadow072(&self) -> &Shadow {
        self.shadow(72)
    }
    ///0x124 - no description available
    #[inline(always)]
    pub const fn shadowshadow073(&self) -> &Shadow {
        self.shadow(73)
    }
    ///0x128 - no description available
    #[inline(always)]
    pub const fn shadowshadow074(&self) -> &Shadow {
        self.shadow(74)
    }
    ///0x12c - no description available
    #[inline(always)]
    pub const fn shadowshadow075(&self) -> &Shadow {
        self.shadow(75)
    }
    ///0x130 - no description available
    #[inline(always)]
    pub const fn shadowshadow076(&self) -> &Shadow {
        self.shadow(76)
    }
    ///0x134 - no description available
    #[inline(always)]
    pub const fn shadowshadow077(&self) -> &Shadow {
        self.shadow(77)
    }
    ///0x138 - no description available
    #[inline(always)]
    pub const fn shadowshadow078(&self) -> &Shadow {
        self.shadow(78)
    }
    ///0x13c - no description available
    #[inline(always)]
    pub const fn shadowshadow079(&self) -> &Shadow {
        self.shadow(79)
    }
    ///0x140 - no description available
    #[inline(always)]
    pub const fn shadowshadow080(&self) -> &Shadow {
        self.shadow(80)
    }
    ///0x144 - no description available
    #[inline(always)]
    pub const fn shadowshadow081(&self) -> &Shadow {
        self.shadow(81)
    }
    ///0x148 - no description available
    #[inline(always)]
    pub const fn shadowshadow082(&self) -> &Shadow {
        self.shadow(82)
    }
    ///0x14c - no description available
    #[inline(always)]
    pub const fn shadowshadow083(&self) -> &Shadow {
        self.shadow(83)
    }
    ///0x150 - no description available
    #[inline(always)]
    pub const fn shadowshadow084(&self) -> &Shadow {
        self.shadow(84)
    }
    ///0x154 - no description available
    #[inline(always)]
    pub const fn shadowshadow085(&self) -> &Shadow {
        self.shadow(85)
    }
    ///0x158 - no description available
    #[inline(always)]
    pub const fn shadowshadow086(&self) -> &Shadow {
        self.shadow(86)
    }
    ///0x15c - no description available
    #[inline(always)]
    pub const fn shadowshadow087(&self) -> &Shadow {
        self.shadow(87)
    }
    ///0x160 - no description available
    #[inline(always)]
    pub const fn shadowshadow088(&self) -> &Shadow {
        self.shadow(88)
    }
    ///0x164 - no description available
    #[inline(always)]
    pub const fn shadowshadow089(&self) -> &Shadow {
        self.shadow(89)
    }
    ///0x168 - no description available
    #[inline(always)]
    pub const fn shadowshadow090(&self) -> &Shadow {
        self.shadow(90)
    }
    ///0x16c - no description available
    #[inline(always)]
    pub const fn shadowshadow091(&self) -> &Shadow {
        self.shadow(91)
    }
    ///0x170 - no description available
    #[inline(always)]
    pub const fn shadowshadow092(&self) -> &Shadow {
        self.shadow(92)
    }
    ///0x174 - no description available
    #[inline(always)]
    pub const fn shadowshadow093(&self) -> &Shadow {
        self.shadow(93)
    }
    ///0x178 - no description available
    #[inline(always)]
    pub const fn shadowshadow094(&self) -> &Shadow {
        self.shadow(94)
    }
    ///0x17c - no description available
    #[inline(always)]
    pub const fn shadowshadow095(&self) -> &Shadow {
        self.shadow(95)
    }
    ///0x180 - no description available
    #[inline(always)]
    pub const fn shadowshadow096(&self) -> &Shadow {
        self.shadow(96)
    }
    ///0x184 - no description available
    #[inline(always)]
    pub const fn shadowshadow097(&self) -> &Shadow {
        self.shadow(97)
    }
    ///0x188 - no description available
    #[inline(always)]
    pub const fn shadowshadow098(&self) -> &Shadow {
        self.shadow(98)
    }
    ///0x18c - no description available
    #[inline(always)]
    pub const fn shadowshadow099(&self) -> &Shadow {
        self.shadow(99)
    }
    ///0x190 - no description available
    #[inline(always)]
    pub const fn shadowshadow100(&self) -> &Shadow {
        self.shadow(100)
    }
    ///0x194 - no description available
    #[inline(always)]
    pub const fn shadowshadow101(&self) -> &Shadow {
        self.shadow(101)
    }
    ///0x198 - no description available
    #[inline(always)]
    pub const fn shadowshadow102(&self) -> &Shadow {
        self.shadow(102)
    }
    ///0x19c - no description available
    #[inline(always)]
    pub const fn shadowshadow103(&self) -> &Shadow {
        self.shadow(103)
    }
    ///0x1a0 - no description available
    #[inline(always)]
    pub const fn shadowshadow104(&self) -> &Shadow {
        self.shadow(104)
    }
    ///0x1a4 - no description available
    #[inline(always)]
    pub const fn shadowshadow105(&self) -> &Shadow {
        self.shadow(105)
    }
    ///0x1a8 - no description available
    #[inline(always)]
    pub const fn shadowshadow106(&self) -> &Shadow {
        self.shadow(106)
    }
    ///0x1ac - no description available
    #[inline(always)]
    pub const fn shadowshadow107(&self) -> &Shadow {
        self.shadow(107)
    }
    ///0x1b0 - no description available
    #[inline(always)]
    pub const fn shadowshadow108(&self) -> &Shadow {
        self.shadow(108)
    }
    ///0x1b4 - no description available
    #[inline(always)]
    pub const fn shadowshadow109(&self) -> &Shadow {
        self.shadow(109)
    }
    ///0x1b8 - no description available
    #[inline(always)]
    pub const fn shadowshadow110(&self) -> &Shadow {
        self.shadow(110)
    }
    ///0x1bc - no description available
    #[inline(always)]
    pub const fn shadowshadow111(&self) -> &Shadow {
        self.shadow(111)
    }
    ///0x1c0 - no description available
    #[inline(always)]
    pub const fn shadowshadow112(&self) -> &Shadow {
        self.shadow(112)
    }
    ///0x1c4 - no description available
    #[inline(always)]
    pub const fn shadowshadow113(&self) -> &Shadow {
        self.shadow(113)
    }
    ///0x1c8 - no description available
    #[inline(always)]
    pub const fn shadowshadow114(&self) -> &Shadow {
        self.shadow(114)
    }
    ///0x1cc - no description available
    #[inline(always)]
    pub const fn shadowshadow115(&self) -> &Shadow {
        self.shadow(115)
    }
    ///0x1d0 - no description available
    #[inline(always)]
    pub const fn shadowshadow116(&self) -> &Shadow {
        self.shadow(116)
    }
    ///0x1d4 - no description available
    #[inline(always)]
    pub const fn shadowshadow117(&self) -> &Shadow {
        self.shadow(117)
    }
    ///0x1d8 - no description available
    #[inline(always)]
    pub const fn shadowshadow118(&self) -> &Shadow {
        self.shadow(118)
    }
    ///0x1dc - no description available
    #[inline(always)]
    pub const fn shadowshadow119(&self) -> &Shadow {
        self.shadow(119)
    }
    ///0x1e0 - no description available
    #[inline(always)]
    pub const fn shadowshadow120(&self) -> &Shadow {
        self.shadow(120)
    }
    ///0x1e4 - no description available
    #[inline(always)]
    pub const fn shadowshadow121(&self) -> &Shadow {
        self.shadow(121)
    }
    ///0x1e8 - no description available
    #[inline(always)]
    pub const fn shadowshadow122(&self) -> &Shadow {
        self.shadow(122)
    }
    ///0x1ec - no description available
    #[inline(always)]
    pub const fn shadowshadow123(&self) -> &Shadow {
        self.shadow(123)
    }
    ///0x1f0 - no description available
    #[inline(always)]
    pub const fn shadowshadow124(&self) -> &Shadow {
        self.shadow(124)
    }
    ///0x1f4 - no description available
    #[inline(always)]
    pub const fn shadowshadow125(&self) -> &Shadow {
        self.shadow(125)
    }
    ///0x1f8 - no description available
    #[inline(always)]
    pub const fn shadowshadow126(&self) -> &Shadow {
        self.shadow(126)
    }
    ///0x1fc - no description available
    #[inline(always)]
    pub const fn shadowshadow127(&self) -> &Shadow {
        self.shadow(127)
    }
    ///0x200..0x220 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SHADOW_LOCKLOCK00` register.</div>
    #[inline(always)]
    pub const fn shadow_lock(&self, n: usize) -> &ShadowLock {
        &self.shadow_lock[n]
    }
    ///Iterator for array of:
    ///0x200..0x220 - no description available
    #[inline(always)]
    pub fn shadow_lock_iter(&self) -> impl Iterator<Item = &ShadowLock> {
        self.shadow_lock.iter()
    }
    ///0x200 - no description available
    #[inline(always)]
    pub const fn shadow_locklock00(&self) -> &ShadowLock {
        self.shadow_lock(0)
    }
    ///0x204 - no description available
    #[inline(always)]
    pub const fn shadow_locklock01(&self) -> &ShadowLock {
        self.shadow_lock(1)
    }
    ///0x208 - no description available
    #[inline(always)]
    pub const fn shadow_locklock02(&self) -> &ShadowLock {
        self.shadow_lock(2)
    }
    ///0x20c - no description available
    #[inline(always)]
    pub const fn shadow_locklock03(&self) -> &ShadowLock {
        self.shadow_lock(3)
    }
    ///0x210 - no description available
    #[inline(always)]
    pub const fn shadow_locklock04(&self) -> &ShadowLock {
        self.shadow_lock(4)
    }
    ///0x214 - no description available
    #[inline(always)]
    pub const fn shadow_locklock05(&self) -> &ShadowLock {
        self.shadow_lock(5)
    }
    ///0x218 - no description available
    #[inline(always)]
    pub const fn shadow_locklock06(&self) -> &ShadowLock {
        self.shadow_lock(6)
    }
    ///0x21c - no description available
    #[inline(always)]
    pub const fn shadow_locklock07(&self) -> &ShadowLock {
        self.shadow_lock(7)
    }
    ///0x400..0x600 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `FUSEFUSE000` register.</div>
    #[inline(always)]
    pub const fn fuse(&self, n: usize) -> &Fuse {
        &self.fuse[n]
    }
    ///Iterator for array of:
    ///0x400..0x600 - no description available
    #[inline(always)]
    pub fn fuse_iter(&self) -> impl Iterator<Item = &Fuse> {
        self.fuse.iter()
    }
    ///0x400 - no description available
    #[inline(always)]
    pub const fn fusefuse000(&self) -> &Fuse {
        self.fuse(0)
    }
    ///0x404 - no description available
    #[inline(always)]
    pub const fn fusefuse001(&self) -> &Fuse {
        self.fuse(1)
    }
    ///0x408 - no description available
    #[inline(always)]
    pub const fn fusefuse002(&self) -> &Fuse {
        self.fuse(2)
    }
    ///0x40c - no description available
    #[inline(always)]
    pub const fn fusefuse003(&self) -> &Fuse {
        self.fuse(3)
    }
    ///0x410 - no description available
    #[inline(always)]
    pub const fn fusefuse004(&self) -> &Fuse {
        self.fuse(4)
    }
    ///0x414 - no description available
    #[inline(always)]
    pub const fn fusefuse005(&self) -> &Fuse {
        self.fuse(5)
    }
    ///0x418 - no description available
    #[inline(always)]
    pub const fn fusefuse006(&self) -> &Fuse {
        self.fuse(6)
    }
    ///0x41c - no description available
    #[inline(always)]
    pub const fn fusefuse007(&self) -> &Fuse {
        self.fuse(7)
    }
    ///0x420 - no description available
    #[inline(always)]
    pub const fn fusefuse008(&self) -> &Fuse {
        self.fuse(8)
    }
    ///0x424 - no description available
    #[inline(always)]
    pub const fn fusefuse009(&self) -> &Fuse {
        self.fuse(9)
    }
    ///0x428 - no description available
    #[inline(always)]
    pub const fn fusefuse010(&self) -> &Fuse {
        self.fuse(10)
    }
    ///0x42c - no description available
    #[inline(always)]
    pub const fn fusefuse011(&self) -> &Fuse {
        self.fuse(11)
    }
    ///0x430 - no description available
    #[inline(always)]
    pub const fn fusefuse012(&self) -> &Fuse {
        self.fuse(12)
    }
    ///0x434 - no description available
    #[inline(always)]
    pub const fn fusefuse013(&self) -> &Fuse {
        self.fuse(13)
    }
    ///0x438 - no description available
    #[inline(always)]
    pub const fn fusefuse014(&self) -> &Fuse {
        self.fuse(14)
    }
    ///0x43c - no description available
    #[inline(always)]
    pub const fn fusefuse015(&self) -> &Fuse {
        self.fuse(15)
    }
    ///0x440 - no description available
    #[inline(always)]
    pub const fn fusefuse016(&self) -> &Fuse {
        self.fuse(16)
    }
    ///0x444 - no description available
    #[inline(always)]
    pub const fn fusefuse017(&self) -> &Fuse {
        self.fuse(17)
    }
    ///0x448 - no description available
    #[inline(always)]
    pub const fn fusefuse018(&self) -> &Fuse {
        self.fuse(18)
    }
    ///0x44c - no description available
    #[inline(always)]
    pub const fn fusefuse019(&self) -> &Fuse {
        self.fuse(19)
    }
    ///0x450 - no description available
    #[inline(always)]
    pub const fn fusefuse020(&self) -> &Fuse {
        self.fuse(20)
    }
    ///0x454 - no description available
    #[inline(always)]
    pub const fn fusefuse021(&self) -> &Fuse {
        self.fuse(21)
    }
    ///0x458 - no description available
    #[inline(always)]
    pub const fn fusefuse022(&self) -> &Fuse {
        self.fuse(22)
    }
    ///0x45c - no description available
    #[inline(always)]
    pub const fn fusefuse023(&self) -> &Fuse {
        self.fuse(23)
    }
    ///0x460 - no description available
    #[inline(always)]
    pub const fn fusefuse024(&self) -> &Fuse {
        self.fuse(24)
    }
    ///0x464 - no description available
    #[inline(always)]
    pub const fn fusefuse025(&self) -> &Fuse {
        self.fuse(25)
    }
    ///0x468 - no description available
    #[inline(always)]
    pub const fn fusefuse026(&self) -> &Fuse {
        self.fuse(26)
    }
    ///0x46c - no description available
    #[inline(always)]
    pub const fn fusefuse027(&self) -> &Fuse {
        self.fuse(27)
    }
    ///0x470 - no description available
    #[inline(always)]
    pub const fn fusefuse028(&self) -> &Fuse {
        self.fuse(28)
    }
    ///0x474 - no description available
    #[inline(always)]
    pub const fn fusefuse029(&self) -> &Fuse {
        self.fuse(29)
    }
    ///0x478 - no description available
    #[inline(always)]
    pub const fn fusefuse030(&self) -> &Fuse {
        self.fuse(30)
    }
    ///0x47c - no description available
    #[inline(always)]
    pub const fn fusefuse031(&self) -> &Fuse {
        self.fuse(31)
    }
    ///0x480 - no description available
    #[inline(always)]
    pub const fn fusefuse032(&self) -> &Fuse {
        self.fuse(32)
    }
    ///0x484 - no description available
    #[inline(always)]
    pub const fn fusefuse033(&self) -> &Fuse {
        self.fuse(33)
    }
    ///0x488 - no description available
    #[inline(always)]
    pub const fn fusefuse034(&self) -> &Fuse {
        self.fuse(34)
    }
    ///0x48c - no description available
    #[inline(always)]
    pub const fn fusefuse035(&self) -> &Fuse {
        self.fuse(35)
    }
    ///0x490 - no description available
    #[inline(always)]
    pub const fn fusefuse036(&self) -> &Fuse {
        self.fuse(36)
    }
    ///0x494 - no description available
    #[inline(always)]
    pub const fn fusefuse037(&self) -> &Fuse {
        self.fuse(37)
    }
    ///0x498 - no description available
    #[inline(always)]
    pub const fn fusefuse038(&self) -> &Fuse {
        self.fuse(38)
    }
    ///0x49c - no description available
    #[inline(always)]
    pub const fn fusefuse039(&self) -> &Fuse {
        self.fuse(39)
    }
    ///0x4a0 - no description available
    #[inline(always)]
    pub const fn fusefuse040(&self) -> &Fuse {
        self.fuse(40)
    }
    ///0x4a4 - no description available
    #[inline(always)]
    pub const fn fusefuse041(&self) -> &Fuse {
        self.fuse(41)
    }
    ///0x4a8 - no description available
    #[inline(always)]
    pub const fn fusefuse042(&self) -> &Fuse {
        self.fuse(42)
    }
    ///0x4ac - no description available
    #[inline(always)]
    pub const fn fusefuse043(&self) -> &Fuse {
        self.fuse(43)
    }
    ///0x4b0 - no description available
    #[inline(always)]
    pub const fn fusefuse044(&self) -> &Fuse {
        self.fuse(44)
    }
    ///0x4b4 - no description available
    #[inline(always)]
    pub const fn fusefuse045(&self) -> &Fuse {
        self.fuse(45)
    }
    ///0x4b8 - no description available
    #[inline(always)]
    pub const fn fusefuse046(&self) -> &Fuse {
        self.fuse(46)
    }
    ///0x4bc - no description available
    #[inline(always)]
    pub const fn fusefuse047(&self) -> &Fuse {
        self.fuse(47)
    }
    ///0x4c0 - no description available
    #[inline(always)]
    pub const fn fusefuse048(&self) -> &Fuse {
        self.fuse(48)
    }
    ///0x4c4 - no description available
    #[inline(always)]
    pub const fn fusefuse049(&self) -> &Fuse {
        self.fuse(49)
    }
    ///0x4c8 - no description available
    #[inline(always)]
    pub const fn fusefuse050(&self) -> &Fuse {
        self.fuse(50)
    }
    ///0x4cc - no description available
    #[inline(always)]
    pub const fn fusefuse051(&self) -> &Fuse {
        self.fuse(51)
    }
    ///0x4d0 - no description available
    #[inline(always)]
    pub const fn fusefuse052(&self) -> &Fuse {
        self.fuse(52)
    }
    ///0x4d4 - no description available
    #[inline(always)]
    pub const fn fusefuse053(&self) -> &Fuse {
        self.fuse(53)
    }
    ///0x4d8 - no description available
    #[inline(always)]
    pub const fn fusefuse054(&self) -> &Fuse {
        self.fuse(54)
    }
    ///0x4dc - no description available
    #[inline(always)]
    pub const fn fusefuse055(&self) -> &Fuse {
        self.fuse(55)
    }
    ///0x4e0 - no description available
    #[inline(always)]
    pub const fn fusefuse056(&self) -> &Fuse {
        self.fuse(56)
    }
    ///0x4e4 - no description available
    #[inline(always)]
    pub const fn fusefuse057(&self) -> &Fuse {
        self.fuse(57)
    }
    ///0x4e8 - no description available
    #[inline(always)]
    pub const fn fusefuse058(&self) -> &Fuse {
        self.fuse(58)
    }
    ///0x4ec - no description available
    #[inline(always)]
    pub const fn fusefuse059(&self) -> &Fuse {
        self.fuse(59)
    }
    ///0x4f0 - no description available
    #[inline(always)]
    pub const fn fusefuse060(&self) -> &Fuse {
        self.fuse(60)
    }
    ///0x4f4 - no description available
    #[inline(always)]
    pub const fn fusefuse061(&self) -> &Fuse {
        self.fuse(61)
    }
    ///0x4f8 - no description available
    #[inline(always)]
    pub const fn fusefuse062(&self) -> &Fuse {
        self.fuse(62)
    }
    ///0x4fc - no description available
    #[inline(always)]
    pub const fn fusefuse063(&self) -> &Fuse {
        self.fuse(63)
    }
    ///0x500 - no description available
    #[inline(always)]
    pub const fn fusefuse064(&self) -> &Fuse {
        self.fuse(64)
    }
    ///0x504 - no description available
    #[inline(always)]
    pub const fn fusefuse065(&self) -> &Fuse {
        self.fuse(65)
    }
    ///0x508 - no description available
    #[inline(always)]
    pub const fn fusefuse066(&self) -> &Fuse {
        self.fuse(66)
    }
    ///0x50c - no description available
    #[inline(always)]
    pub const fn fusefuse067(&self) -> &Fuse {
        self.fuse(67)
    }
    ///0x510 - no description available
    #[inline(always)]
    pub const fn fusefuse068(&self) -> &Fuse {
        self.fuse(68)
    }
    ///0x514 - no description available
    #[inline(always)]
    pub const fn fusefuse069(&self) -> &Fuse {
        self.fuse(69)
    }
    ///0x518 - no description available
    #[inline(always)]
    pub const fn fusefuse070(&self) -> &Fuse {
        self.fuse(70)
    }
    ///0x51c - no description available
    #[inline(always)]
    pub const fn fusefuse071(&self) -> &Fuse {
        self.fuse(71)
    }
    ///0x520 - no description available
    #[inline(always)]
    pub const fn fusefuse072(&self) -> &Fuse {
        self.fuse(72)
    }
    ///0x524 - no description available
    #[inline(always)]
    pub const fn fusefuse073(&self) -> &Fuse {
        self.fuse(73)
    }
    ///0x528 - no description available
    #[inline(always)]
    pub const fn fusefuse074(&self) -> &Fuse {
        self.fuse(74)
    }
    ///0x52c - no description available
    #[inline(always)]
    pub const fn fusefuse075(&self) -> &Fuse {
        self.fuse(75)
    }
    ///0x530 - no description available
    #[inline(always)]
    pub const fn fusefuse076(&self) -> &Fuse {
        self.fuse(76)
    }
    ///0x534 - no description available
    #[inline(always)]
    pub const fn fusefuse077(&self) -> &Fuse {
        self.fuse(77)
    }
    ///0x538 - no description available
    #[inline(always)]
    pub const fn fusefuse078(&self) -> &Fuse {
        self.fuse(78)
    }
    ///0x53c - no description available
    #[inline(always)]
    pub const fn fusefuse079(&self) -> &Fuse {
        self.fuse(79)
    }
    ///0x540 - no description available
    #[inline(always)]
    pub const fn fusefuse080(&self) -> &Fuse {
        self.fuse(80)
    }
    ///0x544 - no description available
    #[inline(always)]
    pub const fn fusefuse081(&self) -> &Fuse {
        self.fuse(81)
    }
    ///0x548 - no description available
    #[inline(always)]
    pub const fn fusefuse082(&self) -> &Fuse {
        self.fuse(82)
    }
    ///0x54c - no description available
    #[inline(always)]
    pub const fn fusefuse083(&self) -> &Fuse {
        self.fuse(83)
    }
    ///0x550 - no description available
    #[inline(always)]
    pub const fn fusefuse084(&self) -> &Fuse {
        self.fuse(84)
    }
    ///0x554 - no description available
    #[inline(always)]
    pub const fn fusefuse085(&self) -> &Fuse {
        self.fuse(85)
    }
    ///0x558 - no description available
    #[inline(always)]
    pub const fn fusefuse086(&self) -> &Fuse {
        self.fuse(86)
    }
    ///0x55c - no description available
    #[inline(always)]
    pub const fn fusefuse087(&self) -> &Fuse {
        self.fuse(87)
    }
    ///0x560 - no description available
    #[inline(always)]
    pub const fn fusefuse088(&self) -> &Fuse {
        self.fuse(88)
    }
    ///0x564 - no description available
    #[inline(always)]
    pub const fn fusefuse089(&self) -> &Fuse {
        self.fuse(89)
    }
    ///0x568 - no description available
    #[inline(always)]
    pub const fn fusefuse090(&self) -> &Fuse {
        self.fuse(90)
    }
    ///0x56c - no description available
    #[inline(always)]
    pub const fn fusefuse091(&self) -> &Fuse {
        self.fuse(91)
    }
    ///0x570 - no description available
    #[inline(always)]
    pub const fn fusefuse092(&self) -> &Fuse {
        self.fuse(92)
    }
    ///0x574 - no description available
    #[inline(always)]
    pub const fn fusefuse093(&self) -> &Fuse {
        self.fuse(93)
    }
    ///0x578 - no description available
    #[inline(always)]
    pub const fn fusefuse094(&self) -> &Fuse {
        self.fuse(94)
    }
    ///0x57c - no description available
    #[inline(always)]
    pub const fn fusefuse095(&self) -> &Fuse {
        self.fuse(95)
    }
    ///0x580 - no description available
    #[inline(always)]
    pub const fn fusefuse096(&self) -> &Fuse {
        self.fuse(96)
    }
    ///0x584 - no description available
    #[inline(always)]
    pub const fn fusefuse097(&self) -> &Fuse {
        self.fuse(97)
    }
    ///0x588 - no description available
    #[inline(always)]
    pub const fn fusefuse098(&self) -> &Fuse {
        self.fuse(98)
    }
    ///0x58c - no description available
    #[inline(always)]
    pub const fn fusefuse099(&self) -> &Fuse {
        self.fuse(99)
    }
    ///0x590 - no description available
    #[inline(always)]
    pub const fn fusefuse100(&self) -> &Fuse {
        self.fuse(100)
    }
    ///0x594 - no description available
    #[inline(always)]
    pub const fn fusefuse101(&self) -> &Fuse {
        self.fuse(101)
    }
    ///0x598 - no description available
    #[inline(always)]
    pub const fn fusefuse102(&self) -> &Fuse {
        self.fuse(102)
    }
    ///0x59c - no description available
    #[inline(always)]
    pub const fn fusefuse103(&self) -> &Fuse {
        self.fuse(103)
    }
    ///0x5a0 - no description available
    #[inline(always)]
    pub const fn fusefuse104(&self) -> &Fuse {
        self.fuse(104)
    }
    ///0x5a4 - no description available
    #[inline(always)]
    pub const fn fusefuse105(&self) -> &Fuse {
        self.fuse(105)
    }
    ///0x5a8 - no description available
    #[inline(always)]
    pub const fn fusefuse106(&self) -> &Fuse {
        self.fuse(106)
    }
    ///0x5ac - no description available
    #[inline(always)]
    pub const fn fusefuse107(&self) -> &Fuse {
        self.fuse(107)
    }
    ///0x5b0 - no description available
    #[inline(always)]
    pub const fn fusefuse108(&self) -> &Fuse {
        self.fuse(108)
    }
    ///0x5b4 - no description available
    #[inline(always)]
    pub const fn fusefuse109(&self) -> &Fuse {
        self.fuse(109)
    }
    ///0x5b8 - no description available
    #[inline(always)]
    pub const fn fusefuse110(&self) -> &Fuse {
        self.fuse(110)
    }
    ///0x5bc - no description available
    #[inline(always)]
    pub const fn fusefuse111(&self) -> &Fuse {
        self.fuse(111)
    }
    ///0x5c0 - no description available
    #[inline(always)]
    pub const fn fusefuse112(&self) -> &Fuse {
        self.fuse(112)
    }
    ///0x5c4 - no description available
    #[inline(always)]
    pub const fn fusefuse113(&self) -> &Fuse {
        self.fuse(113)
    }
    ///0x5c8 - no description available
    #[inline(always)]
    pub const fn fusefuse114(&self) -> &Fuse {
        self.fuse(114)
    }
    ///0x5cc - no description available
    #[inline(always)]
    pub const fn fusefuse115(&self) -> &Fuse {
        self.fuse(115)
    }
    ///0x5d0 - no description available
    #[inline(always)]
    pub const fn fusefuse116(&self) -> &Fuse {
        self.fuse(116)
    }
    ///0x5d4 - no description available
    #[inline(always)]
    pub const fn fusefuse117(&self) -> &Fuse {
        self.fuse(117)
    }
    ///0x5d8 - no description available
    #[inline(always)]
    pub const fn fusefuse118(&self) -> &Fuse {
        self.fuse(118)
    }
    ///0x5dc - no description available
    #[inline(always)]
    pub const fn fusefuse119(&self) -> &Fuse {
        self.fuse(119)
    }
    ///0x5e0 - no description available
    #[inline(always)]
    pub const fn fusefuse120(&self) -> &Fuse {
        self.fuse(120)
    }
    ///0x5e4 - no description available
    #[inline(always)]
    pub const fn fusefuse121(&self) -> &Fuse {
        self.fuse(121)
    }
    ///0x5e8 - no description available
    #[inline(always)]
    pub const fn fusefuse122(&self) -> &Fuse {
        self.fuse(122)
    }
    ///0x5ec - no description available
    #[inline(always)]
    pub const fn fusefuse123(&self) -> &Fuse {
        self.fuse(123)
    }
    ///0x5f0 - no description available
    #[inline(always)]
    pub const fn fusefuse124(&self) -> &Fuse {
        self.fuse(124)
    }
    ///0x5f4 - no description available
    #[inline(always)]
    pub const fn fusefuse125(&self) -> &Fuse {
        self.fuse(125)
    }
    ///0x5f8 - no description available
    #[inline(always)]
    pub const fn fusefuse126(&self) -> &Fuse {
        self.fuse(126)
    }
    ///0x5fc - no description available
    #[inline(always)]
    pub const fn fusefuse127(&self) -> &Fuse {
        self.fuse(127)
    }
    ///0x600..0x620 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `FUSE_LOCKLOCK00` register.</div>
    #[inline(always)]
    pub const fn fuse_lock(&self, n: usize) -> &FuseLock {
        &self.fuse_lock[n]
    }
    ///Iterator for array of:
    ///0x600..0x620 - no description available
    #[inline(always)]
    pub fn fuse_lock_iter(&self) -> impl Iterator<Item = &FuseLock> {
        self.fuse_lock.iter()
    }
    ///0x600 - no description available
    #[inline(always)]
    pub const fn fuse_locklock00(&self) -> &FuseLock {
        self.fuse_lock(0)
    }
    ///0x604 - no description available
    #[inline(always)]
    pub const fn fuse_locklock01(&self) -> &FuseLock {
        self.fuse_lock(1)
    }
    ///0x608 - no description available
    #[inline(always)]
    pub const fn fuse_locklock02(&self) -> &FuseLock {
        self.fuse_lock(2)
    }
    ///0x60c - no description available
    #[inline(always)]
    pub const fn fuse_locklock03(&self) -> &FuseLock {
        self.fuse_lock(3)
    }
    ///0x610 - no description available
    #[inline(always)]
    pub const fn fuse_locklock04(&self) -> &FuseLock {
        self.fuse_lock(4)
    }
    ///0x614 - no description available
    #[inline(always)]
    pub const fn fuse_locklock05(&self) -> &FuseLock {
        self.fuse_lock(5)
    }
    ///0x618 - no description available
    #[inline(always)]
    pub const fn fuse_locklock06(&self) -> &FuseLock {
        self.fuse_lock(6)
    }
    ///0x61c - no description available
    #[inline(always)]
    pub const fn fuse_locklock07(&self) -> &FuseLock {
        self.fuse_lock(7)
    }
    ///0x800 - UNLOCK
    #[inline(always)]
    pub const fn unlock(&self) -> &Unlock {
        &self.unlock
    }
    ///0x804 - DATA
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    ///0x808 - ADDR
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    ///0x80c - CMD
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0xa00 - LOAD Request
    #[inline(always)]
    pub const fn load_req(&self) -> &LoadReq {
        &self.load_req
    }
    ///0xa04 - LOAD complete
    #[inline(always)]
    pub const fn load_comp(&self) -> &LoadComp {
        &self.load_comp
    }
    ///0xa20..0xa30 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `REGIONLOAD_REGION0` register.</div>
    #[inline(always)]
    pub const fn region(&self, n: usize) -> &Region {
        &self.region[n]
    }
    ///Iterator for array of:
    ///0xa20..0xa30 - no description available
    #[inline(always)]
    pub fn region_iter(&self) -> impl Iterator<Item = &Region> {
        self.region.iter()
    }
    ///0xa20 - no description available
    #[inline(always)]
    pub const fn regionload_region0(&self) -> &Region {
        self.region(0)
    }
    ///0xa24 - no description available
    #[inline(always)]
    pub const fn regionload_region1(&self) -> &Region {
        self.region(1)
    }
    ///0xa28 - no description available
    #[inline(always)]
    pub const fn regionload_region2(&self) -> &Region {
        self.region(2)
    }
    ///0xa2c - no description available
    #[inline(always)]
    pub const fn regionload_region3(&self) -> &Region {
        self.region(3)
    }
    ///0xc00 - interrupt flag
    #[inline(always)]
    pub const fn int_flag(&self) -> &IntFlag {
        &self.int_flag
    }
    ///0xc04 - interrupt enable
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
}
/**SHADOW (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`shadow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@shadow`] module*/
#[doc(alias = "SHADOW")]
pub type Shadow = crate::Reg<shadow::ShadowSpec>;
///no description available
pub mod shadow;
/**SHADOW_LOCK (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`shadow_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadow_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@shadow_lock`] module*/
#[doc(alias = "SHADOW_LOCK")]
pub type ShadowLock = crate::Reg<shadow_lock::ShadowLockSpec>;
///no description available
pub mod shadow_lock;
/**FUSE (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`fuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fuse`] module*/
#[doc(alias = "FUSE")]
pub type Fuse = crate::Reg<fuse::FuseSpec>;
///no description available
pub mod fuse;
/**FUSE_LOCK (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`fuse_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fuse_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fuse_lock`] module*/
#[doc(alias = "FUSE_LOCK")]
pub type FuseLock = crate::Reg<fuse_lock::FuseLockSpec>;
///no description available
pub mod fuse_lock;
/**UNLOCK (rw) register accessor: UNLOCK

You can [`read`](crate::Reg::read) this register and get [`unlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@unlock`] module*/
#[doc(alias = "UNLOCK")]
pub type Unlock = crate::Reg<unlock::UnlockSpec>;
///UNLOCK
pub mod unlock;
/**DATA (rw) register accessor: DATA

You can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
///DATA
pub mod data;
/**ADDR (rw) register accessor: ADDR

You can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
///ADDR
pub mod addr;
/**CMD (rw) register accessor: CMD

You can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
///CMD
pub mod cmd;
/**LOAD_REQ (rw) register accessor: LOAD Request

You can [`read`](crate::Reg::read) this register and get [`load_req::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load_req`] module*/
#[doc(alias = "LOAD_REQ")]
pub type LoadReq = crate::Reg<load_req::LoadReqSpec>;
///LOAD Request
pub mod load_req;
/**LOAD_COMP (rw) register accessor: LOAD complete

You can [`read`](crate::Reg::read) this register and get [`load_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load_comp`] module*/
#[doc(alias = "LOAD_COMP")]
pub type LoadComp = crate::Reg<load_comp::LoadCompSpec>;
///LOAD complete
pub mod load_comp;
/**REGION (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`region::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@region`] module*/
#[doc(alias = "REGION")]
pub type Region = crate::Reg<region::RegionSpec>;
///no description available
pub mod region;
/**INT_FLAG (rw) register accessor: interrupt flag

You can [`read`](crate::Reg::read) this register and get [`int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_flag`] module*/
#[doc(alias = "INT_FLAG")]
pub type IntFlag = crate::Reg<int_flag::IntFlagSpec>;
///interrupt flag
pub mod int_flag;
/**INT_EN (rw) register accessor: interrupt enable

You can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_en`] module*/
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
///interrupt enable
pub mod int_en;
