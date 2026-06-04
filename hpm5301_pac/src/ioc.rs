#[repr(C)]
///Register block
pub struct RegisterBlock {
    pad: [Pad; 456],
}
impl RegisterBlock {
    ///0x00..0xe40 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `PADpa00` cluster.</div>
    #[inline(always)]
    pub const fn pad(&self, n: usize) -> &Pad {
        &self.pad[n]
    }
    ///Iterator for array of:
    ///0x00..0xe40 - no description available
    #[inline(always)]
    pub fn pad_iter(&self) -> impl Iterator<Item = &Pad> {
        self.pad.iter()
    }
    ///0x00..0x08 - no description available
    #[inline(always)]
    pub const fn padpa00(&self) -> &Pad {
        self.pad(0)
    }
    ///0x08..0x10 - no description available
    #[inline(always)]
    pub const fn padpa01(&self) -> &Pad {
        self.pad(1)
    }
    ///0x10..0x18 - no description available
    #[inline(always)]
    pub const fn padpa02(&self) -> &Pad {
        self.pad(2)
    }
    ///0x18..0x20 - no description available
    #[inline(always)]
    pub const fn padpa03(&self) -> &Pad {
        self.pad(3)
    }
    ///0x20..0x28 - no description available
    #[inline(always)]
    pub const fn padpa04(&self) -> &Pad {
        self.pad(4)
    }
    ///0x28..0x30 - no description available
    #[inline(always)]
    pub const fn padpa05(&self) -> &Pad {
        self.pad(5)
    }
    ///0x30..0x38 - no description available
    #[inline(always)]
    pub const fn padpa06(&self) -> &Pad {
        self.pad(6)
    }
    ///0x38..0x40 - no description available
    #[inline(always)]
    pub const fn padpa07(&self) -> &Pad {
        self.pad(7)
    }
    ///0x40..0x48 - no description available
    #[inline(always)]
    pub const fn padpa08(&self) -> &Pad {
        self.pad(8)
    }
    ///0x48..0x50 - no description available
    #[inline(always)]
    pub const fn padpa09(&self) -> &Pad {
        self.pad(9)
    }
    ///0x50..0x58 - no description available
    #[inline(always)]
    pub const fn padpa10(&self) -> &Pad {
        self.pad(10)
    }
    ///0x58..0x60 - no description available
    #[inline(always)]
    pub const fn padpa11(&self) -> &Pad {
        self.pad(11)
    }
    ///0x60..0x68 - no description available
    #[inline(always)]
    pub const fn padpa12(&self) -> &Pad {
        self.pad(12)
    }
    ///0x68..0x70 - no description available
    #[inline(always)]
    pub const fn padpa13(&self) -> &Pad {
        self.pad(13)
    }
    ///0x70..0x78 - no description available
    #[inline(always)]
    pub const fn padpa14(&self) -> &Pad {
        self.pad(14)
    }
    ///0x78..0x80 - no description available
    #[inline(always)]
    pub const fn padpa15(&self) -> &Pad {
        self.pad(15)
    }
    ///0x80..0x88 - no description available
    #[inline(always)]
    pub const fn padpa16(&self) -> &Pad {
        self.pad(16)
    }
    ///0x88..0x90 - no description available
    #[inline(always)]
    pub const fn padpa17(&self) -> &Pad {
        self.pad(17)
    }
    ///0x90..0x98 - no description available
    #[inline(always)]
    pub const fn padpa18(&self) -> &Pad {
        self.pad(18)
    }
    ///0x98..0xa0 - no description available
    #[inline(always)]
    pub const fn padpa19(&self) -> &Pad {
        self.pad(19)
    }
    ///0xa0..0xa8 - no description available
    #[inline(always)]
    pub const fn padpa20(&self) -> &Pad {
        self.pad(20)
    }
    ///0xa8..0xb0 - no description available
    #[inline(always)]
    pub const fn padpa21(&self) -> &Pad {
        self.pad(21)
    }
    ///0xb0..0xb8 - no description available
    #[inline(always)]
    pub const fn padpa22(&self) -> &Pad {
        self.pad(22)
    }
    ///0xb8..0xc0 - no description available
    #[inline(always)]
    pub const fn padpa23(&self) -> &Pad {
        self.pad(23)
    }
    ///0xc0..0xc8 - no description available
    #[inline(always)]
    pub const fn padpa24(&self) -> &Pad {
        self.pad(24)
    }
    ///0xc8..0xd0 - no description available
    #[inline(always)]
    pub const fn padpa25(&self) -> &Pad {
        self.pad(25)
    }
    ///0xd0..0xd8 - no description available
    #[inline(always)]
    pub const fn padpa26(&self) -> &Pad {
        self.pad(26)
    }
    ///0xd8..0xe0 - no description available
    #[inline(always)]
    pub const fn padpa27(&self) -> &Pad {
        self.pad(27)
    }
    ///0xe0..0xe8 - no description available
    #[inline(always)]
    pub const fn padpa28(&self) -> &Pad {
        self.pad(28)
    }
    ///0xe8..0xf0 - no description available
    #[inline(always)]
    pub const fn padpa29(&self) -> &Pad {
        self.pad(29)
    }
    ///0xf0..0xf8 - no description available
    #[inline(always)]
    pub const fn padpa30(&self) -> &Pad {
        self.pad(30)
    }
    ///0xf8..0x100 - no description available
    #[inline(always)]
    pub const fn padpa31(&self) -> &Pad {
        self.pad(31)
    }
    ///0x100..0x108 - no description available
    #[inline(always)]
    pub const fn padpb00(&self) -> &Pad {
        self.pad(32)
    }
    ///0x108..0x110 - no description available
    #[inline(always)]
    pub const fn padpb01(&self) -> &Pad {
        self.pad(33)
    }
    ///0x110..0x118 - no description available
    #[inline(always)]
    pub const fn padpb02(&self) -> &Pad {
        self.pad(34)
    }
    ///0x118..0x120 - no description available
    #[inline(always)]
    pub const fn padpb03(&self) -> &Pad {
        self.pad(35)
    }
    ///0x120..0x128 - no description available
    #[inline(always)]
    pub const fn padpb04(&self) -> &Pad {
        self.pad(36)
    }
    ///0x128..0x130 - no description available
    #[inline(always)]
    pub const fn padpb05(&self) -> &Pad {
        self.pad(37)
    }
    ///0x130..0x138 - no description available
    #[inline(always)]
    pub const fn padpb06(&self) -> &Pad {
        self.pad(38)
    }
    ///0x138..0x140 - no description available
    #[inline(always)]
    pub const fn padpb07(&self) -> &Pad {
        self.pad(39)
    }
    ///0x140..0x148 - no description available
    #[inline(always)]
    pub const fn padpb08(&self) -> &Pad {
        self.pad(40)
    }
    ///0x148..0x150 - no description available
    #[inline(always)]
    pub const fn padpb09(&self) -> &Pad {
        self.pad(41)
    }
    ///0x150..0x158 - no description available
    #[inline(always)]
    pub const fn padpb10(&self) -> &Pad {
        self.pad(42)
    }
    ///0x158..0x160 - no description available
    #[inline(always)]
    pub const fn padpb11(&self) -> &Pad {
        self.pad(43)
    }
    ///0x160..0x168 - no description available
    #[inline(always)]
    pub const fn padpb12(&self) -> &Pad {
        self.pad(44)
    }
    ///0x168..0x170 - no description available
    #[inline(always)]
    pub const fn padpb13(&self) -> &Pad {
        self.pad(45)
    }
    ///0x170..0x178 - no description available
    #[inline(always)]
    pub const fn padpb14(&self) -> &Pad {
        self.pad(46)
    }
    ///0x178..0x180 - no description available
    #[inline(always)]
    pub const fn padpb15(&self) -> &Pad {
        self.pad(47)
    }
    ///0x180..0x188 - no description available
    #[inline(always)]
    pub const fn padrsv48(&self) -> &Pad {
        self.pad(48)
    }
    ///0x188..0x190 - no description available
    #[inline(always)]
    pub const fn padrsv49(&self) -> &Pad {
        self.pad(49)
    }
    ///0x190..0x198 - no description available
    #[inline(always)]
    pub const fn padrsv50(&self) -> &Pad {
        self.pad(50)
    }
    ///0x198..0x1a0 - no description available
    #[inline(always)]
    pub const fn padrsv51(&self) -> &Pad {
        self.pad(51)
    }
    ///0x1a0..0x1a8 - no description available
    #[inline(always)]
    pub const fn padrsv52(&self) -> &Pad {
        self.pad(52)
    }
    ///0x1a8..0x1b0 - no description available
    #[inline(always)]
    pub const fn padrsv53(&self) -> &Pad {
        self.pad(53)
    }
    ///0x1b0..0x1b8 - no description available
    #[inline(always)]
    pub const fn padrsv54(&self) -> &Pad {
        self.pad(54)
    }
    ///0x1b8..0x1c0 - no description available
    #[inline(always)]
    pub const fn padrsv55(&self) -> &Pad {
        self.pad(55)
    }
    ///0x1c0..0x1c8 - no description available
    #[inline(always)]
    pub const fn padrsv56(&self) -> &Pad {
        self.pad(56)
    }
    ///0x1c8..0x1d0 - no description available
    #[inline(always)]
    pub const fn padrsv57(&self) -> &Pad {
        self.pad(57)
    }
    ///0x1d0..0x1d8 - no description available
    #[inline(always)]
    pub const fn padrsv58(&self) -> &Pad {
        self.pad(58)
    }
    ///0x1d8..0x1e0 - no description available
    #[inline(always)]
    pub const fn padrsv59(&self) -> &Pad {
        self.pad(59)
    }
    ///0x1e0..0x1e8 - no description available
    #[inline(always)]
    pub const fn padrsv60(&self) -> &Pad {
        self.pad(60)
    }
    ///0x1e8..0x1f0 - no description available
    #[inline(always)]
    pub const fn padrsv61(&self) -> &Pad {
        self.pad(61)
    }
    ///0x1f0..0x1f8 - no description available
    #[inline(always)]
    pub const fn padrsv62(&self) -> &Pad {
        self.pad(62)
    }
    ///0x1f8..0x200 - no description available
    #[inline(always)]
    pub const fn padrsv63(&self) -> &Pad {
        self.pad(63)
    }
    ///0x200..0x208 - no description available
    #[inline(always)]
    pub const fn padrsv64(&self) -> &Pad {
        self.pad(64)
    }
    ///0x208..0x210 - no description available
    #[inline(always)]
    pub const fn padrsv65(&self) -> &Pad {
        self.pad(65)
    }
    ///0x210..0x218 - no description available
    #[inline(always)]
    pub const fn padrsv66(&self) -> &Pad {
        self.pad(66)
    }
    ///0x218..0x220 - no description available
    #[inline(always)]
    pub const fn padrsv67(&self) -> &Pad {
        self.pad(67)
    }
    ///0x220..0x228 - no description available
    #[inline(always)]
    pub const fn padrsv68(&self) -> &Pad {
        self.pad(68)
    }
    ///0x228..0x230 - no description available
    #[inline(always)]
    pub const fn padrsv69(&self) -> &Pad {
        self.pad(69)
    }
    ///0x230..0x238 - no description available
    #[inline(always)]
    pub const fn padrsv70(&self) -> &Pad {
        self.pad(70)
    }
    ///0x238..0x240 - no description available
    #[inline(always)]
    pub const fn padrsv71(&self) -> &Pad {
        self.pad(71)
    }
    ///0x240..0x248 - no description available
    #[inline(always)]
    pub const fn padrsv72(&self) -> &Pad {
        self.pad(72)
    }
    ///0x248..0x250 - no description available
    #[inline(always)]
    pub const fn padrsv73(&self) -> &Pad {
        self.pad(73)
    }
    ///0x250..0x258 - no description available
    #[inline(always)]
    pub const fn padrsv74(&self) -> &Pad {
        self.pad(74)
    }
    ///0x258..0x260 - no description available
    #[inline(always)]
    pub const fn padrsv75(&self) -> &Pad {
        self.pad(75)
    }
    ///0x260..0x268 - no description available
    #[inline(always)]
    pub const fn padrsv76(&self) -> &Pad {
        self.pad(76)
    }
    ///0x268..0x270 - no description available
    #[inline(always)]
    pub const fn padrsv77(&self) -> &Pad {
        self.pad(77)
    }
    ///0x270..0x278 - no description available
    #[inline(always)]
    pub const fn padrsv78(&self) -> &Pad {
        self.pad(78)
    }
    ///0x278..0x280 - no description available
    #[inline(always)]
    pub const fn padrsv79(&self) -> &Pad {
        self.pad(79)
    }
    ///0x280..0x288 - no description available
    #[inline(always)]
    pub const fn padrsv80(&self) -> &Pad {
        self.pad(80)
    }
    ///0x288..0x290 - no description available
    #[inline(always)]
    pub const fn padrsv81(&self) -> &Pad {
        self.pad(81)
    }
    ///0x290..0x298 - no description available
    #[inline(always)]
    pub const fn padrsv82(&self) -> &Pad {
        self.pad(82)
    }
    ///0x298..0x2a0 - no description available
    #[inline(always)]
    pub const fn padrsv83(&self) -> &Pad {
        self.pad(83)
    }
    ///0x2a0..0x2a8 - no description available
    #[inline(always)]
    pub const fn padrsv84(&self) -> &Pad {
        self.pad(84)
    }
    ///0x2a8..0x2b0 - no description available
    #[inline(always)]
    pub const fn padrsv85(&self) -> &Pad {
        self.pad(85)
    }
    ///0x2b0..0x2b8 - no description available
    #[inline(always)]
    pub const fn padrsv86(&self) -> &Pad {
        self.pad(86)
    }
    ///0x2b8..0x2c0 - no description available
    #[inline(always)]
    pub const fn padrsv87(&self) -> &Pad {
        self.pad(87)
    }
    ///0x2c0..0x2c8 - no description available
    #[inline(always)]
    pub const fn padrsv88(&self) -> &Pad {
        self.pad(88)
    }
    ///0x2c8..0x2d0 - no description available
    #[inline(always)]
    pub const fn padrsv89(&self) -> &Pad {
        self.pad(89)
    }
    ///0x2d0..0x2d8 - no description available
    #[inline(always)]
    pub const fn padrsv90(&self) -> &Pad {
        self.pad(90)
    }
    ///0x2d8..0x2e0 - no description available
    #[inline(always)]
    pub const fn padrsv91(&self) -> &Pad {
        self.pad(91)
    }
    ///0x2e0..0x2e8 - no description available
    #[inline(always)]
    pub const fn padrsv92(&self) -> &Pad {
        self.pad(92)
    }
    ///0x2e8..0x2f0 - no description available
    #[inline(always)]
    pub const fn padrsv93(&self) -> &Pad {
        self.pad(93)
    }
    ///0x2f0..0x2f8 - no description available
    #[inline(always)]
    pub const fn padrsv94(&self) -> &Pad {
        self.pad(94)
    }
    ///0x2f8..0x300 - no description available
    #[inline(always)]
    pub const fn padrsv95(&self) -> &Pad {
        self.pad(95)
    }
    ///0x300..0x308 - no description available
    #[inline(always)]
    pub const fn padrsv96(&self) -> &Pad {
        self.pad(96)
    }
    ///0x308..0x310 - no description available
    #[inline(always)]
    pub const fn padrsv97(&self) -> &Pad {
        self.pad(97)
    }
    ///0x310..0x318 - no description available
    #[inline(always)]
    pub const fn padrsv98(&self) -> &Pad {
        self.pad(98)
    }
    ///0x318..0x320 - no description available
    #[inline(always)]
    pub const fn padrsv99(&self) -> &Pad {
        self.pad(99)
    }
    ///0x320..0x328 - no description available
    #[inline(always)]
    pub const fn padrsv100(&self) -> &Pad {
        self.pad(100)
    }
    ///0x328..0x330 - no description available
    #[inline(always)]
    pub const fn padrsv101(&self) -> &Pad {
        self.pad(101)
    }
    ///0x330..0x338 - no description available
    #[inline(always)]
    pub const fn padrsv102(&self) -> &Pad {
        self.pad(102)
    }
    ///0x338..0x340 - no description available
    #[inline(always)]
    pub const fn padrsv103(&self) -> &Pad {
        self.pad(103)
    }
    ///0x340..0x348 - no description available
    #[inline(always)]
    pub const fn padrsv104(&self) -> &Pad {
        self.pad(104)
    }
    ///0x348..0x350 - no description available
    #[inline(always)]
    pub const fn padrsv105(&self) -> &Pad {
        self.pad(105)
    }
    ///0x350..0x358 - no description available
    #[inline(always)]
    pub const fn padrsv106(&self) -> &Pad {
        self.pad(106)
    }
    ///0x358..0x360 - no description available
    #[inline(always)]
    pub const fn padrsv107(&self) -> &Pad {
        self.pad(107)
    }
    ///0x360..0x368 - no description available
    #[inline(always)]
    pub const fn padrsv108(&self) -> &Pad {
        self.pad(108)
    }
    ///0x368..0x370 - no description available
    #[inline(always)]
    pub const fn padrsv109(&self) -> &Pad {
        self.pad(109)
    }
    ///0x370..0x378 - no description available
    #[inline(always)]
    pub const fn padrsv110(&self) -> &Pad {
        self.pad(110)
    }
    ///0x378..0x380 - no description available
    #[inline(always)]
    pub const fn padrsv111(&self) -> &Pad {
        self.pad(111)
    }
    ///0x380..0x388 - no description available
    #[inline(always)]
    pub const fn padrsv112(&self) -> &Pad {
        self.pad(112)
    }
    ///0x388..0x390 - no description available
    #[inline(always)]
    pub const fn padrsv113(&self) -> &Pad {
        self.pad(113)
    }
    ///0x390..0x398 - no description available
    #[inline(always)]
    pub const fn padrsv114(&self) -> &Pad {
        self.pad(114)
    }
    ///0x398..0x3a0 - no description available
    #[inline(always)]
    pub const fn padrsv115(&self) -> &Pad {
        self.pad(115)
    }
    ///0x3a0..0x3a8 - no description available
    #[inline(always)]
    pub const fn padrsv116(&self) -> &Pad {
        self.pad(116)
    }
    ///0x3a8..0x3b0 - no description available
    #[inline(always)]
    pub const fn padrsv117(&self) -> &Pad {
        self.pad(117)
    }
    ///0x3b0..0x3b8 - no description available
    #[inline(always)]
    pub const fn padrsv118(&self) -> &Pad {
        self.pad(118)
    }
    ///0x3b8..0x3c0 - no description available
    #[inline(always)]
    pub const fn padrsv119(&self) -> &Pad {
        self.pad(119)
    }
    ///0x3c0..0x3c8 - no description available
    #[inline(always)]
    pub const fn padrsv120(&self) -> &Pad {
        self.pad(120)
    }
    ///0x3c8..0x3d0 - no description available
    #[inline(always)]
    pub const fn padrsv121(&self) -> &Pad {
        self.pad(121)
    }
    ///0x3d0..0x3d8 - no description available
    #[inline(always)]
    pub const fn padrsv122(&self) -> &Pad {
        self.pad(122)
    }
    ///0x3d8..0x3e0 - no description available
    #[inline(always)]
    pub const fn padrsv123(&self) -> &Pad {
        self.pad(123)
    }
    ///0x3e0..0x3e8 - no description available
    #[inline(always)]
    pub const fn padrsv124(&self) -> &Pad {
        self.pad(124)
    }
    ///0x3e8..0x3f0 - no description available
    #[inline(always)]
    pub const fn padrsv125(&self) -> &Pad {
        self.pad(125)
    }
    ///0x3f0..0x3f8 - no description available
    #[inline(always)]
    pub const fn padrsv126(&self) -> &Pad {
        self.pad(126)
    }
    ///0x3f8..0x400 - no description available
    #[inline(always)]
    pub const fn padrsv127(&self) -> &Pad {
        self.pad(127)
    }
    ///0x400..0x408 - no description available
    #[inline(always)]
    pub const fn padrsv128(&self) -> &Pad {
        self.pad(128)
    }
    ///0x408..0x410 - no description available
    #[inline(always)]
    pub const fn padrsv129(&self) -> &Pad {
        self.pad(129)
    }
    ///0x410..0x418 - no description available
    #[inline(always)]
    pub const fn padrsv130(&self) -> &Pad {
        self.pad(130)
    }
    ///0x418..0x420 - no description available
    #[inline(always)]
    pub const fn padrsv131(&self) -> &Pad {
        self.pad(131)
    }
    ///0x420..0x428 - no description available
    #[inline(always)]
    pub const fn padrsv132(&self) -> &Pad {
        self.pad(132)
    }
    ///0x428..0x430 - no description available
    #[inline(always)]
    pub const fn padrsv133(&self) -> &Pad {
        self.pad(133)
    }
    ///0x430..0x438 - no description available
    #[inline(always)]
    pub const fn padrsv134(&self) -> &Pad {
        self.pad(134)
    }
    ///0x438..0x440 - no description available
    #[inline(always)]
    pub const fn padrsv135(&self) -> &Pad {
        self.pad(135)
    }
    ///0x440..0x448 - no description available
    #[inline(always)]
    pub const fn padrsv136(&self) -> &Pad {
        self.pad(136)
    }
    ///0x448..0x450 - no description available
    #[inline(always)]
    pub const fn padrsv137(&self) -> &Pad {
        self.pad(137)
    }
    ///0x450..0x458 - no description available
    #[inline(always)]
    pub const fn padrsv138(&self) -> &Pad {
        self.pad(138)
    }
    ///0x458..0x460 - no description available
    #[inline(always)]
    pub const fn padrsv139(&self) -> &Pad {
        self.pad(139)
    }
    ///0x460..0x468 - no description available
    #[inline(always)]
    pub const fn padrsv140(&self) -> &Pad {
        self.pad(140)
    }
    ///0x468..0x470 - no description available
    #[inline(always)]
    pub const fn padrsv141(&self) -> &Pad {
        self.pad(141)
    }
    ///0x470..0x478 - no description available
    #[inline(always)]
    pub const fn padrsv142(&self) -> &Pad {
        self.pad(142)
    }
    ///0x478..0x480 - no description available
    #[inline(always)]
    pub const fn padrsv143(&self) -> &Pad {
        self.pad(143)
    }
    ///0x480..0x488 - no description available
    #[inline(always)]
    pub const fn padrsv144(&self) -> &Pad {
        self.pad(144)
    }
    ///0x488..0x490 - no description available
    #[inline(always)]
    pub const fn padrsv145(&self) -> &Pad {
        self.pad(145)
    }
    ///0x490..0x498 - no description available
    #[inline(always)]
    pub const fn padrsv146(&self) -> &Pad {
        self.pad(146)
    }
    ///0x498..0x4a0 - no description available
    #[inline(always)]
    pub const fn padrsv147(&self) -> &Pad {
        self.pad(147)
    }
    ///0x4a0..0x4a8 - no description available
    #[inline(always)]
    pub const fn padrsv148(&self) -> &Pad {
        self.pad(148)
    }
    ///0x4a8..0x4b0 - no description available
    #[inline(always)]
    pub const fn padrsv149(&self) -> &Pad {
        self.pad(149)
    }
    ///0x4b0..0x4b8 - no description available
    #[inline(always)]
    pub const fn padrsv150(&self) -> &Pad {
        self.pad(150)
    }
    ///0x4b8..0x4c0 - no description available
    #[inline(always)]
    pub const fn padrsv151(&self) -> &Pad {
        self.pad(151)
    }
    ///0x4c0..0x4c8 - no description available
    #[inline(always)]
    pub const fn padrsv152(&self) -> &Pad {
        self.pad(152)
    }
    ///0x4c8..0x4d0 - no description available
    #[inline(always)]
    pub const fn padrsv153(&self) -> &Pad {
        self.pad(153)
    }
    ///0x4d0..0x4d8 - no description available
    #[inline(always)]
    pub const fn padrsv154(&self) -> &Pad {
        self.pad(154)
    }
    ///0x4d8..0x4e0 - no description available
    #[inline(always)]
    pub const fn padrsv155(&self) -> &Pad {
        self.pad(155)
    }
    ///0x4e0..0x4e8 - no description available
    #[inline(always)]
    pub const fn padrsv156(&self) -> &Pad {
        self.pad(156)
    }
    ///0x4e8..0x4f0 - no description available
    #[inline(always)]
    pub const fn padrsv157(&self) -> &Pad {
        self.pad(157)
    }
    ///0x4f0..0x4f8 - no description available
    #[inline(always)]
    pub const fn padrsv158(&self) -> &Pad {
        self.pad(158)
    }
    ///0x4f8..0x500 - no description available
    #[inline(always)]
    pub const fn padrsv159(&self) -> &Pad {
        self.pad(159)
    }
    ///0x500..0x508 - no description available
    #[inline(always)]
    pub const fn padrsv160(&self) -> &Pad {
        self.pad(160)
    }
    ///0x508..0x510 - no description available
    #[inline(always)]
    pub const fn padrsv161(&self) -> &Pad {
        self.pad(161)
    }
    ///0x510..0x518 - no description available
    #[inline(always)]
    pub const fn padrsv162(&self) -> &Pad {
        self.pad(162)
    }
    ///0x518..0x520 - no description available
    #[inline(always)]
    pub const fn padrsv163(&self) -> &Pad {
        self.pad(163)
    }
    ///0x520..0x528 - no description available
    #[inline(always)]
    pub const fn padrsv164(&self) -> &Pad {
        self.pad(164)
    }
    ///0x528..0x530 - no description available
    #[inline(always)]
    pub const fn padrsv165(&self) -> &Pad {
        self.pad(165)
    }
    ///0x530..0x538 - no description available
    #[inline(always)]
    pub const fn padrsv166(&self) -> &Pad {
        self.pad(166)
    }
    ///0x538..0x540 - no description available
    #[inline(always)]
    pub const fn padrsv167(&self) -> &Pad {
        self.pad(167)
    }
    ///0x540..0x548 - no description available
    #[inline(always)]
    pub const fn padrsv168(&self) -> &Pad {
        self.pad(168)
    }
    ///0x548..0x550 - no description available
    #[inline(always)]
    pub const fn padrsv169(&self) -> &Pad {
        self.pad(169)
    }
    ///0x550..0x558 - no description available
    #[inline(always)]
    pub const fn padrsv170(&self) -> &Pad {
        self.pad(170)
    }
    ///0x558..0x560 - no description available
    #[inline(always)]
    pub const fn padrsv171(&self) -> &Pad {
        self.pad(171)
    }
    ///0x560..0x568 - no description available
    #[inline(always)]
    pub const fn padrsv172(&self) -> &Pad {
        self.pad(172)
    }
    ///0x568..0x570 - no description available
    #[inline(always)]
    pub const fn padrsv173(&self) -> &Pad {
        self.pad(173)
    }
    ///0x570..0x578 - no description available
    #[inline(always)]
    pub const fn padrsv174(&self) -> &Pad {
        self.pad(174)
    }
    ///0x578..0x580 - no description available
    #[inline(always)]
    pub const fn padrsv175(&self) -> &Pad {
        self.pad(175)
    }
    ///0x580..0x588 - no description available
    #[inline(always)]
    pub const fn padrsv176(&self) -> &Pad {
        self.pad(176)
    }
    ///0x588..0x590 - no description available
    #[inline(always)]
    pub const fn padrsv177(&self) -> &Pad {
        self.pad(177)
    }
    ///0x590..0x598 - no description available
    #[inline(always)]
    pub const fn padrsv178(&self) -> &Pad {
        self.pad(178)
    }
    ///0x598..0x5a0 - no description available
    #[inline(always)]
    pub const fn padrsv179(&self) -> &Pad {
        self.pad(179)
    }
    ///0x5a0..0x5a8 - no description available
    #[inline(always)]
    pub const fn padrsv180(&self) -> &Pad {
        self.pad(180)
    }
    ///0x5a8..0x5b0 - no description available
    #[inline(always)]
    pub const fn padrsv181(&self) -> &Pad {
        self.pad(181)
    }
    ///0x5b0..0x5b8 - no description available
    #[inline(always)]
    pub const fn padrsv182(&self) -> &Pad {
        self.pad(182)
    }
    ///0x5b8..0x5c0 - no description available
    #[inline(always)]
    pub const fn padrsv183(&self) -> &Pad {
        self.pad(183)
    }
    ///0x5c0..0x5c8 - no description available
    #[inline(always)]
    pub const fn padrsv184(&self) -> &Pad {
        self.pad(184)
    }
    ///0x5c8..0x5d0 - no description available
    #[inline(always)]
    pub const fn padrsv185(&self) -> &Pad {
        self.pad(185)
    }
    ///0x5d0..0x5d8 - no description available
    #[inline(always)]
    pub const fn padrsv186(&self) -> &Pad {
        self.pad(186)
    }
    ///0x5d8..0x5e0 - no description available
    #[inline(always)]
    pub const fn padrsv187(&self) -> &Pad {
        self.pad(187)
    }
    ///0x5e0..0x5e8 - no description available
    #[inline(always)]
    pub const fn padrsv188(&self) -> &Pad {
        self.pad(188)
    }
    ///0x5e8..0x5f0 - no description available
    #[inline(always)]
    pub const fn padrsv189(&self) -> &Pad {
        self.pad(189)
    }
    ///0x5f0..0x5f8 - no description available
    #[inline(always)]
    pub const fn padrsv190(&self) -> &Pad {
        self.pad(190)
    }
    ///0x5f8..0x600 - no description available
    #[inline(always)]
    pub const fn padrsv191(&self) -> &Pad {
        self.pad(191)
    }
    ///0x600..0x608 - no description available
    #[inline(always)]
    pub const fn padrsv192(&self) -> &Pad {
        self.pad(192)
    }
    ///0x608..0x610 - no description available
    #[inline(always)]
    pub const fn padrsv193(&self) -> &Pad {
        self.pad(193)
    }
    ///0x610..0x618 - no description available
    #[inline(always)]
    pub const fn padrsv194(&self) -> &Pad {
        self.pad(194)
    }
    ///0x618..0x620 - no description available
    #[inline(always)]
    pub const fn padrsv195(&self) -> &Pad {
        self.pad(195)
    }
    ///0x620..0x628 - no description available
    #[inline(always)]
    pub const fn padrsv196(&self) -> &Pad {
        self.pad(196)
    }
    ///0x628..0x630 - no description available
    #[inline(always)]
    pub const fn padrsv197(&self) -> &Pad {
        self.pad(197)
    }
    ///0x630..0x638 - no description available
    #[inline(always)]
    pub const fn padrsv198(&self) -> &Pad {
        self.pad(198)
    }
    ///0x638..0x640 - no description available
    #[inline(always)]
    pub const fn padrsv199(&self) -> &Pad {
        self.pad(199)
    }
    ///0x640..0x648 - no description available
    #[inline(always)]
    pub const fn padrsv200(&self) -> &Pad {
        self.pad(200)
    }
    ///0x648..0x650 - no description available
    #[inline(always)]
    pub const fn padrsv201(&self) -> &Pad {
        self.pad(201)
    }
    ///0x650..0x658 - no description available
    #[inline(always)]
    pub const fn padrsv202(&self) -> &Pad {
        self.pad(202)
    }
    ///0x658..0x660 - no description available
    #[inline(always)]
    pub const fn padrsv203(&self) -> &Pad {
        self.pad(203)
    }
    ///0x660..0x668 - no description available
    #[inline(always)]
    pub const fn padrsv204(&self) -> &Pad {
        self.pad(204)
    }
    ///0x668..0x670 - no description available
    #[inline(always)]
    pub const fn padrsv205(&self) -> &Pad {
        self.pad(205)
    }
    ///0x670..0x678 - no description available
    #[inline(always)]
    pub const fn padrsv206(&self) -> &Pad {
        self.pad(206)
    }
    ///0x678..0x680 - no description available
    #[inline(always)]
    pub const fn padrsv207(&self) -> &Pad {
        self.pad(207)
    }
    ///0x680..0x688 - no description available
    #[inline(always)]
    pub const fn padrsv208(&self) -> &Pad {
        self.pad(208)
    }
    ///0x688..0x690 - no description available
    #[inline(always)]
    pub const fn padrsv209(&self) -> &Pad {
        self.pad(209)
    }
    ///0x690..0x698 - no description available
    #[inline(always)]
    pub const fn padrsv210(&self) -> &Pad {
        self.pad(210)
    }
    ///0x698..0x6a0 - no description available
    #[inline(always)]
    pub const fn padrsv211(&self) -> &Pad {
        self.pad(211)
    }
    ///0x6a0..0x6a8 - no description available
    #[inline(always)]
    pub const fn padrsv212(&self) -> &Pad {
        self.pad(212)
    }
    ///0x6a8..0x6b0 - no description available
    #[inline(always)]
    pub const fn padrsv213(&self) -> &Pad {
        self.pad(213)
    }
    ///0x6b0..0x6b8 - no description available
    #[inline(always)]
    pub const fn padrsv214(&self) -> &Pad {
        self.pad(214)
    }
    ///0x6b8..0x6c0 - no description available
    #[inline(always)]
    pub const fn padrsv215(&self) -> &Pad {
        self.pad(215)
    }
    ///0x6c0..0x6c8 - no description available
    #[inline(always)]
    pub const fn padrsv216(&self) -> &Pad {
        self.pad(216)
    }
    ///0x6c8..0x6d0 - no description available
    #[inline(always)]
    pub const fn padrsv217(&self) -> &Pad {
        self.pad(217)
    }
    ///0x6d0..0x6d8 - no description available
    #[inline(always)]
    pub const fn padrsv218(&self) -> &Pad {
        self.pad(218)
    }
    ///0x6d8..0x6e0 - no description available
    #[inline(always)]
    pub const fn padrsv219(&self) -> &Pad {
        self.pad(219)
    }
    ///0x6e0..0x6e8 - no description available
    #[inline(always)]
    pub const fn padrsv220(&self) -> &Pad {
        self.pad(220)
    }
    ///0x6e8..0x6f0 - no description available
    #[inline(always)]
    pub const fn padrsv221(&self) -> &Pad {
        self.pad(221)
    }
    ///0x6f0..0x6f8 - no description available
    #[inline(always)]
    pub const fn padrsv222(&self) -> &Pad {
        self.pad(222)
    }
    ///0x6f8..0x700 - no description available
    #[inline(always)]
    pub const fn padrsv223(&self) -> &Pad {
        self.pad(223)
    }
    ///0x700..0x708 - no description available
    #[inline(always)]
    pub const fn padrsv224(&self) -> &Pad {
        self.pad(224)
    }
    ///0x708..0x710 - no description available
    #[inline(always)]
    pub const fn padrsv225(&self) -> &Pad {
        self.pad(225)
    }
    ///0x710..0x718 - no description available
    #[inline(always)]
    pub const fn padrsv226(&self) -> &Pad {
        self.pad(226)
    }
    ///0x718..0x720 - no description available
    #[inline(always)]
    pub const fn padrsv227(&self) -> &Pad {
        self.pad(227)
    }
    ///0x720..0x728 - no description available
    #[inline(always)]
    pub const fn padrsv228(&self) -> &Pad {
        self.pad(228)
    }
    ///0x728..0x730 - no description available
    #[inline(always)]
    pub const fn padrsv229(&self) -> &Pad {
        self.pad(229)
    }
    ///0x730..0x738 - no description available
    #[inline(always)]
    pub const fn padrsv230(&self) -> &Pad {
        self.pad(230)
    }
    ///0x738..0x740 - no description available
    #[inline(always)]
    pub const fn padrsv231(&self) -> &Pad {
        self.pad(231)
    }
    ///0x740..0x748 - no description available
    #[inline(always)]
    pub const fn padrsv232(&self) -> &Pad {
        self.pad(232)
    }
    ///0x748..0x750 - no description available
    #[inline(always)]
    pub const fn padrsv233(&self) -> &Pad {
        self.pad(233)
    }
    ///0x750..0x758 - no description available
    #[inline(always)]
    pub const fn padrsv234(&self) -> &Pad {
        self.pad(234)
    }
    ///0x758..0x760 - no description available
    #[inline(always)]
    pub const fn padrsv235(&self) -> &Pad {
        self.pad(235)
    }
    ///0x760..0x768 - no description available
    #[inline(always)]
    pub const fn padrsv236(&self) -> &Pad {
        self.pad(236)
    }
    ///0x768..0x770 - no description available
    #[inline(always)]
    pub const fn padrsv237(&self) -> &Pad {
        self.pad(237)
    }
    ///0x770..0x778 - no description available
    #[inline(always)]
    pub const fn padrsv238(&self) -> &Pad {
        self.pad(238)
    }
    ///0x778..0x780 - no description available
    #[inline(always)]
    pub const fn padrsv239(&self) -> &Pad {
        self.pad(239)
    }
    ///0x780..0x788 - no description available
    #[inline(always)]
    pub const fn padrsv240(&self) -> &Pad {
        self.pad(240)
    }
    ///0x788..0x790 - no description available
    #[inline(always)]
    pub const fn padrsv241(&self) -> &Pad {
        self.pad(241)
    }
    ///0x790..0x798 - no description available
    #[inline(always)]
    pub const fn padrsv242(&self) -> &Pad {
        self.pad(242)
    }
    ///0x798..0x7a0 - no description available
    #[inline(always)]
    pub const fn padrsv243(&self) -> &Pad {
        self.pad(243)
    }
    ///0x7a0..0x7a8 - no description available
    #[inline(always)]
    pub const fn padrsv244(&self) -> &Pad {
        self.pad(244)
    }
    ///0x7a8..0x7b0 - no description available
    #[inline(always)]
    pub const fn padrsv245(&self) -> &Pad {
        self.pad(245)
    }
    ///0x7b0..0x7b8 - no description available
    #[inline(always)]
    pub const fn padrsv246(&self) -> &Pad {
        self.pad(246)
    }
    ///0x7b8..0x7c0 - no description available
    #[inline(always)]
    pub const fn padrsv247(&self) -> &Pad {
        self.pad(247)
    }
    ///0x7c0..0x7c8 - no description available
    #[inline(always)]
    pub const fn padrsv248(&self) -> &Pad {
        self.pad(248)
    }
    ///0x7c8..0x7d0 - no description available
    #[inline(always)]
    pub const fn padrsv249(&self) -> &Pad {
        self.pad(249)
    }
    ///0x7d0..0x7d8 - no description available
    #[inline(always)]
    pub const fn padrsv250(&self) -> &Pad {
        self.pad(250)
    }
    ///0x7d8..0x7e0 - no description available
    #[inline(always)]
    pub const fn padrsv251(&self) -> &Pad {
        self.pad(251)
    }
    ///0x7e0..0x7e8 - no description available
    #[inline(always)]
    pub const fn padrsv252(&self) -> &Pad {
        self.pad(252)
    }
    ///0x7e8..0x7f0 - no description available
    #[inline(always)]
    pub const fn padrsv253(&self) -> &Pad {
        self.pad(253)
    }
    ///0x7f0..0x7f8 - no description available
    #[inline(always)]
    pub const fn padrsv254(&self) -> &Pad {
        self.pad(254)
    }
    ///0x7f8..0x800 - no description available
    #[inline(always)]
    pub const fn padrsv255(&self) -> &Pad {
        self.pad(255)
    }
    ///0x800..0x808 - no description available
    #[inline(always)]
    pub const fn padrsv256(&self) -> &Pad {
        self.pad(256)
    }
    ///0x808..0x810 - no description available
    #[inline(always)]
    pub const fn padrsv257(&self) -> &Pad {
        self.pad(257)
    }
    ///0x810..0x818 - no description available
    #[inline(always)]
    pub const fn padrsv258(&self) -> &Pad {
        self.pad(258)
    }
    ///0x818..0x820 - no description available
    #[inline(always)]
    pub const fn padrsv259(&self) -> &Pad {
        self.pad(259)
    }
    ///0x820..0x828 - no description available
    #[inline(always)]
    pub const fn padrsv260(&self) -> &Pad {
        self.pad(260)
    }
    ///0x828..0x830 - no description available
    #[inline(always)]
    pub const fn padrsv261(&self) -> &Pad {
        self.pad(261)
    }
    ///0x830..0x838 - no description available
    #[inline(always)]
    pub const fn padrsv262(&self) -> &Pad {
        self.pad(262)
    }
    ///0x838..0x840 - no description available
    #[inline(always)]
    pub const fn padrsv263(&self) -> &Pad {
        self.pad(263)
    }
    ///0x840..0x848 - no description available
    #[inline(always)]
    pub const fn padrsv264(&self) -> &Pad {
        self.pad(264)
    }
    ///0x848..0x850 - no description available
    #[inline(always)]
    pub const fn padrsv265(&self) -> &Pad {
        self.pad(265)
    }
    ///0x850..0x858 - no description available
    #[inline(always)]
    pub const fn padrsv266(&self) -> &Pad {
        self.pad(266)
    }
    ///0x858..0x860 - no description available
    #[inline(always)]
    pub const fn padrsv267(&self) -> &Pad {
        self.pad(267)
    }
    ///0x860..0x868 - no description available
    #[inline(always)]
    pub const fn padrsv268(&self) -> &Pad {
        self.pad(268)
    }
    ///0x868..0x870 - no description available
    #[inline(always)]
    pub const fn padrsv269(&self) -> &Pad {
        self.pad(269)
    }
    ///0x870..0x878 - no description available
    #[inline(always)]
    pub const fn padrsv270(&self) -> &Pad {
        self.pad(270)
    }
    ///0x878..0x880 - no description available
    #[inline(always)]
    pub const fn padrsv271(&self) -> &Pad {
        self.pad(271)
    }
    ///0x880..0x888 - no description available
    #[inline(always)]
    pub const fn padrsv272(&self) -> &Pad {
        self.pad(272)
    }
    ///0x888..0x890 - no description available
    #[inline(always)]
    pub const fn padrsv273(&self) -> &Pad {
        self.pad(273)
    }
    ///0x890..0x898 - no description available
    #[inline(always)]
    pub const fn padrsv274(&self) -> &Pad {
        self.pad(274)
    }
    ///0x898..0x8a0 - no description available
    #[inline(always)]
    pub const fn padrsv275(&self) -> &Pad {
        self.pad(275)
    }
    ///0x8a0..0x8a8 - no description available
    #[inline(always)]
    pub const fn padrsv276(&self) -> &Pad {
        self.pad(276)
    }
    ///0x8a8..0x8b0 - no description available
    #[inline(always)]
    pub const fn padrsv277(&self) -> &Pad {
        self.pad(277)
    }
    ///0x8b0..0x8b8 - no description available
    #[inline(always)]
    pub const fn padrsv278(&self) -> &Pad {
        self.pad(278)
    }
    ///0x8b8..0x8c0 - no description available
    #[inline(always)]
    pub const fn padrsv279(&self) -> &Pad {
        self.pad(279)
    }
    ///0x8c0..0x8c8 - no description available
    #[inline(always)]
    pub const fn padrsv280(&self) -> &Pad {
        self.pad(280)
    }
    ///0x8c8..0x8d0 - no description available
    #[inline(always)]
    pub const fn padrsv281(&self) -> &Pad {
        self.pad(281)
    }
    ///0x8d0..0x8d8 - no description available
    #[inline(always)]
    pub const fn padrsv282(&self) -> &Pad {
        self.pad(282)
    }
    ///0x8d8..0x8e0 - no description available
    #[inline(always)]
    pub const fn padrsv283(&self) -> &Pad {
        self.pad(283)
    }
    ///0x8e0..0x8e8 - no description available
    #[inline(always)]
    pub const fn padrsv284(&self) -> &Pad {
        self.pad(284)
    }
    ///0x8e8..0x8f0 - no description available
    #[inline(always)]
    pub const fn padrsv285(&self) -> &Pad {
        self.pad(285)
    }
    ///0x8f0..0x8f8 - no description available
    #[inline(always)]
    pub const fn padrsv286(&self) -> &Pad {
        self.pad(286)
    }
    ///0x8f8..0x900 - no description available
    #[inline(always)]
    pub const fn padrsv287(&self) -> &Pad {
        self.pad(287)
    }
    ///0x900..0x908 - no description available
    #[inline(always)]
    pub const fn padrsv288(&self) -> &Pad {
        self.pad(288)
    }
    ///0x908..0x910 - no description available
    #[inline(always)]
    pub const fn padrsv289(&self) -> &Pad {
        self.pad(289)
    }
    ///0x910..0x918 - no description available
    #[inline(always)]
    pub const fn padrsv290(&self) -> &Pad {
        self.pad(290)
    }
    ///0x918..0x920 - no description available
    #[inline(always)]
    pub const fn padrsv291(&self) -> &Pad {
        self.pad(291)
    }
    ///0x920..0x928 - no description available
    #[inline(always)]
    pub const fn padrsv292(&self) -> &Pad {
        self.pad(292)
    }
    ///0x928..0x930 - no description available
    #[inline(always)]
    pub const fn padrsv293(&self) -> &Pad {
        self.pad(293)
    }
    ///0x930..0x938 - no description available
    #[inline(always)]
    pub const fn padrsv294(&self) -> &Pad {
        self.pad(294)
    }
    ///0x938..0x940 - no description available
    #[inline(always)]
    pub const fn padrsv295(&self) -> &Pad {
        self.pad(295)
    }
    ///0x940..0x948 - no description available
    #[inline(always)]
    pub const fn padrsv296(&self) -> &Pad {
        self.pad(296)
    }
    ///0x948..0x950 - no description available
    #[inline(always)]
    pub const fn padrsv297(&self) -> &Pad {
        self.pad(297)
    }
    ///0x950..0x958 - no description available
    #[inline(always)]
    pub const fn padrsv298(&self) -> &Pad {
        self.pad(298)
    }
    ///0x958..0x960 - no description available
    #[inline(always)]
    pub const fn padrsv299(&self) -> &Pad {
        self.pad(299)
    }
    ///0x960..0x968 - no description available
    #[inline(always)]
    pub const fn padrsv300(&self) -> &Pad {
        self.pad(300)
    }
    ///0x968..0x970 - no description available
    #[inline(always)]
    pub const fn padrsv301(&self) -> &Pad {
        self.pad(301)
    }
    ///0x970..0x978 - no description available
    #[inline(always)]
    pub const fn padrsv302(&self) -> &Pad {
        self.pad(302)
    }
    ///0x978..0x980 - no description available
    #[inline(always)]
    pub const fn padrsv303(&self) -> &Pad {
        self.pad(303)
    }
    ///0x980..0x988 - no description available
    #[inline(always)]
    pub const fn padrsv304(&self) -> &Pad {
        self.pad(304)
    }
    ///0x988..0x990 - no description available
    #[inline(always)]
    pub const fn padrsv305(&self) -> &Pad {
        self.pad(305)
    }
    ///0x990..0x998 - no description available
    #[inline(always)]
    pub const fn padrsv306(&self) -> &Pad {
        self.pad(306)
    }
    ///0x998..0x9a0 - no description available
    #[inline(always)]
    pub const fn padrsv307(&self) -> &Pad {
        self.pad(307)
    }
    ///0x9a0..0x9a8 - no description available
    #[inline(always)]
    pub const fn padrsv308(&self) -> &Pad {
        self.pad(308)
    }
    ///0x9a8..0x9b0 - no description available
    #[inline(always)]
    pub const fn padrsv309(&self) -> &Pad {
        self.pad(309)
    }
    ///0x9b0..0x9b8 - no description available
    #[inline(always)]
    pub const fn padrsv310(&self) -> &Pad {
        self.pad(310)
    }
    ///0x9b8..0x9c0 - no description available
    #[inline(always)]
    pub const fn padrsv311(&self) -> &Pad {
        self.pad(311)
    }
    ///0x9c0..0x9c8 - no description available
    #[inline(always)]
    pub const fn padrsv312(&self) -> &Pad {
        self.pad(312)
    }
    ///0x9c8..0x9d0 - no description available
    #[inline(always)]
    pub const fn padrsv313(&self) -> &Pad {
        self.pad(313)
    }
    ///0x9d0..0x9d8 - no description available
    #[inline(always)]
    pub const fn padrsv314(&self) -> &Pad {
        self.pad(314)
    }
    ///0x9d8..0x9e0 - no description available
    #[inline(always)]
    pub const fn padrsv315(&self) -> &Pad {
        self.pad(315)
    }
    ///0x9e0..0x9e8 - no description available
    #[inline(always)]
    pub const fn padrsv316(&self) -> &Pad {
        self.pad(316)
    }
    ///0x9e8..0x9f0 - no description available
    #[inline(always)]
    pub const fn padrsv317(&self) -> &Pad {
        self.pad(317)
    }
    ///0x9f0..0x9f8 - no description available
    #[inline(always)]
    pub const fn padrsv318(&self) -> &Pad {
        self.pad(318)
    }
    ///0x9f8..0xa00 - no description available
    #[inline(always)]
    pub const fn padrsv319(&self) -> &Pad {
        self.pad(319)
    }
    ///0xa00..0xa08 - no description available
    #[inline(always)]
    pub const fn padrsv320(&self) -> &Pad {
        self.pad(320)
    }
    ///0xa08..0xa10 - no description available
    #[inline(always)]
    pub const fn padrsv321(&self) -> &Pad {
        self.pad(321)
    }
    ///0xa10..0xa18 - no description available
    #[inline(always)]
    pub const fn padrsv322(&self) -> &Pad {
        self.pad(322)
    }
    ///0xa18..0xa20 - no description available
    #[inline(always)]
    pub const fn padrsv323(&self) -> &Pad {
        self.pad(323)
    }
    ///0xa20..0xa28 - no description available
    #[inline(always)]
    pub const fn padrsv324(&self) -> &Pad {
        self.pad(324)
    }
    ///0xa28..0xa30 - no description available
    #[inline(always)]
    pub const fn padrsv325(&self) -> &Pad {
        self.pad(325)
    }
    ///0xa30..0xa38 - no description available
    #[inline(always)]
    pub const fn padrsv326(&self) -> &Pad {
        self.pad(326)
    }
    ///0xa38..0xa40 - no description available
    #[inline(always)]
    pub const fn padrsv327(&self) -> &Pad {
        self.pad(327)
    }
    ///0xa40..0xa48 - no description available
    #[inline(always)]
    pub const fn padrsv328(&self) -> &Pad {
        self.pad(328)
    }
    ///0xa48..0xa50 - no description available
    #[inline(always)]
    pub const fn padrsv329(&self) -> &Pad {
        self.pad(329)
    }
    ///0xa50..0xa58 - no description available
    #[inline(always)]
    pub const fn padrsv330(&self) -> &Pad {
        self.pad(330)
    }
    ///0xa58..0xa60 - no description available
    #[inline(always)]
    pub const fn padrsv331(&self) -> &Pad {
        self.pad(331)
    }
    ///0xa60..0xa68 - no description available
    #[inline(always)]
    pub const fn padrsv332(&self) -> &Pad {
        self.pad(332)
    }
    ///0xa68..0xa70 - no description available
    #[inline(always)]
    pub const fn padrsv333(&self) -> &Pad {
        self.pad(333)
    }
    ///0xa70..0xa78 - no description available
    #[inline(always)]
    pub const fn padrsv334(&self) -> &Pad {
        self.pad(334)
    }
    ///0xa78..0xa80 - no description available
    #[inline(always)]
    pub const fn padrsv335(&self) -> &Pad {
        self.pad(335)
    }
    ///0xa80..0xa88 - no description available
    #[inline(always)]
    pub const fn padrsv336(&self) -> &Pad {
        self.pad(336)
    }
    ///0xa88..0xa90 - no description available
    #[inline(always)]
    pub const fn padrsv337(&self) -> &Pad {
        self.pad(337)
    }
    ///0xa90..0xa98 - no description available
    #[inline(always)]
    pub const fn padrsv338(&self) -> &Pad {
        self.pad(338)
    }
    ///0xa98..0xaa0 - no description available
    #[inline(always)]
    pub const fn padrsv339(&self) -> &Pad {
        self.pad(339)
    }
    ///0xaa0..0xaa8 - no description available
    #[inline(always)]
    pub const fn padrsv340(&self) -> &Pad {
        self.pad(340)
    }
    ///0xaa8..0xab0 - no description available
    #[inline(always)]
    pub const fn padrsv341(&self) -> &Pad {
        self.pad(341)
    }
    ///0xab0..0xab8 - no description available
    #[inline(always)]
    pub const fn padrsv342(&self) -> &Pad {
        self.pad(342)
    }
    ///0xab8..0xac0 - no description available
    #[inline(always)]
    pub const fn padrsv343(&self) -> &Pad {
        self.pad(343)
    }
    ///0xac0..0xac8 - no description available
    #[inline(always)]
    pub const fn padrsv344(&self) -> &Pad {
        self.pad(344)
    }
    ///0xac8..0xad0 - no description available
    #[inline(always)]
    pub const fn padrsv345(&self) -> &Pad {
        self.pad(345)
    }
    ///0xad0..0xad8 - no description available
    #[inline(always)]
    pub const fn padrsv346(&self) -> &Pad {
        self.pad(346)
    }
    ///0xad8..0xae0 - no description available
    #[inline(always)]
    pub const fn padrsv347(&self) -> &Pad {
        self.pad(347)
    }
    ///0xae0..0xae8 - no description available
    #[inline(always)]
    pub const fn padrsv348(&self) -> &Pad {
        self.pad(348)
    }
    ///0xae8..0xaf0 - no description available
    #[inline(always)]
    pub const fn padrsv349(&self) -> &Pad {
        self.pad(349)
    }
    ///0xaf0..0xaf8 - no description available
    #[inline(always)]
    pub const fn padrsv350(&self) -> &Pad {
        self.pad(350)
    }
    ///0xaf8..0xb00 - no description available
    #[inline(always)]
    pub const fn padrsv351(&self) -> &Pad {
        self.pad(351)
    }
    ///0xb00..0xb08 - no description available
    #[inline(always)]
    pub const fn padrsv352(&self) -> &Pad {
        self.pad(352)
    }
    ///0xb08..0xb10 - no description available
    #[inline(always)]
    pub const fn padrsv353(&self) -> &Pad {
        self.pad(353)
    }
    ///0xb10..0xb18 - no description available
    #[inline(always)]
    pub const fn padrsv354(&self) -> &Pad {
        self.pad(354)
    }
    ///0xb18..0xb20 - no description available
    #[inline(always)]
    pub const fn padrsv355(&self) -> &Pad {
        self.pad(355)
    }
    ///0xb20..0xb28 - no description available
    #[inline(always)]
    pub const fn padrsv356(&self) -> &Pad {
        self.pad(356)
    }
    ///0xb28..0xb30 - no description available
    #[inline(always)]
    pub const fn padrsv357(&self) -> &Pad {
        self.pad(357)
    }
    ///0xb30..0xb38 - no description available
    #[inline(always)]
    pub const fn padrsv358(&self) -> &Pad {
        self.pad(358)
    }
    ///0xb38..0xb40 - no description available
    #[inline(always)]
    pub const fn padrsv359(&self) -> &Pad {
        self.pad(359)
    }
    ///0xb40..0xb48 - no description available
    #[inline(always)]
    pub const fn padrsv360(&self) -> &Pad {
        self.pad(360)
    }
    ///0xb48..0xb50 - no description available
    #[inline(always)]
    pub const fn padrsv361(&self) -> &Pad {
        self.pad(361)
    }
    ///0xb50..0xb58 - no description available
    #[inline(always)]
    pub const fn padrsv362(&self) -> &Pad {
        self.pad(362)
    }
    ///0xb58..0xb60 - no description available
    #[inline(always)]
    pub const fn padrsv363(&self) -> &Pad {
        self.pad(363)
    }
    ///0xb60..0xb68 - no description available
    #[inline(always)]
    pub const fn padrsv364(&self) -> &Pad {
        self.pad(364)
    }
    ///0xb68..0xb70 - no description available
    #[inline(always)]
    pub const fn padrsv365(&self) -> &Pad {
        self.pad(365)
    }
    ///0xb70..0xb78 - no description available
    #[inline(always)]
    pub const fn padrsv366(&self) -> &Pad {
        self.pad(366)
    }
    ///0xb78..0xb80 - no description available
    #[inline(always)]
    pub const fn padrsv367(&self) -> &Pad {
        self.pad(367)
    }
    ///0xb80..0xb88 - no description available
    #[inline(always)]
    pub const fn padrsv368(&self) -> &Pad {
        self.pad(368)
    }
    ///0xb88..0xb90 - no description available
    #[inline(always)]
    pub const fn padrsv369(&self) -> &Pad {
        self.pad(369)
    }
    ///0xb90..0xb98 - no description available
    #[inline(always)]
    pub const fn padrsv370(&self) -> &Pad {
        self.pad(370)
    }
    ///0xb98..0xba0 - no description available
    #[inline(always)]
    pub const fn padrsv371(&self) -> &Pad {
        self.pad(371)
    }
    ///0xba0..0xba8 - no description available
    #[inline(always)]
    pub const fn padrsv372(&self) -> &Pad {
        self.pad(372)
    }
    ///0xba8..0xbb0 - no description available
    #[inline(always)]
    pub const fn padrsv373(&self) -> &Pad {
        self.pad(373)
    }
    ///0xbb0..0xbb8 - no description available
    #[inline(always)]
    pub const fn padrsv374(&self) -> &Pad {
        self.pad(374)
    }
    ///0xbb8..0xbc0 - no description available
    #[inline(always)]
    pub const fn padrsv375(&self) -> &Pad {
        self.pad(375)
    }
    ///0xbc0..0xbc8 - no description available
    #[inline(always)]
    pub const fn padrsv376(&self) -> &Pad {
        self.pad(376)
    }
    ///0xbc8..0xbd0 - no description available
    #[inline(always)]
    pub const fn padrsv377(&self) -> &Pad {
        self.pad(377)
    }
    ///0xbd0..0xbd8 - no description available
    #[inline(always)]
    pub const fn padrsv378(&self) -> &Pad {
        self.pad(378)
    }
    ///0xbd8..0xbe0 - no description available
    #[inline(always)]
    pub const fn padrsv379(&self) -> &Pad {
        self.pad(379)
    }
    ///0xbe0..0xbe8 - no description available
    #[inline(always)]
    pub const fn padrsv380(&self) -> &Pad {
        self.pad(380)
    }
    ///0xbe8..0xbf0 - no description available
    #[inline(always)]
    pub const fn padrsv381(&self) -> &Pad {
        self.pad(381)
    }
    ///0xbf0..0xbf8 - no description available
    #[inline(always)]
    pub const fn padrsv382(&self) -> &Pad {
        self.pad(382)
    }
    ///0xbf8..0xc00 - no description available
    #[inline(always)]
    pub const fn padrsv383(&self) -> &Pad {
        self.pad(383)
    }
    ///0xc00..0xc08 - no description available
    #[inline(always)]
    pub const fn padrsv384(&self) -> &Pad {
        self.pad(384)
    }
    ///0xc08..0xc10 - no description available
    #[inline(always)]
    pub const fn padrsv385(&self) -> &Pad {
        self.pad(385)
    }
    ///0xc10..0xc18 - no description available
    #[inline(always)]
    pub const fn padrsv386(&self) -> &Pad {
        self.pad(386)
    }
    ///0xc18..0xc20 - no description available
    #[inline(always)]
    pub const fn padrsv387(&self) -> &Pad {
        self.pad(387)
    }
    ///0xc20..0xc28 - no description available
    #[inline(always)]
    pub const fn padrsv388(&self) -> &Pad {
        self.pad(388)
    }
    ///0xc28..0xc30 - no description available
    #[inline(always)]
    pub const fn padrsv389(&self) -> &Pad {
        self.pad(389)
    }
    ///0xc30..0xc38 - no description available
    #[inline(always)]
    pub const fn padrsv390(&self) -> &Pad {
        self.pad(390)
    }
    ///0xc38..0xc40 - no description available
    #[inline(always)]
    pub const fn padrsv391(&self) -> &Pad {
        self.pad(391)
    }
    ///0xc40..0xc48 - no description available
    #[inline(always)]
    pub const fn padrsv392(&self) -> &Pad {
        self.pad(392)
    }
    ///0xc48..0xc50 - no description available
    #[inline(always)]
    pub const fn padrsv393(&self) -> &Pad {
        self.pad(393)
    }
    ///0xc50..0xc58 - no description available
    #[inline(always)]
    pub const fn padrsv394(&self) -> &Pad {
        self.pad(394)
    }
    ///0xc58..0xc60 - no description available
    #[inline(always)]
    pub const fn padrsv395(&self) -> &Pad {
        self.pad(395)
    }
    ///0xc60..0xc68 - no description available
    #[inline(always)]
    pub const fn padrsv396(&self) -> &Pad {
        self.pad(396)
    }
    ///0xc68..0xc70 - no description available
    #[inline(always)]
    pub const fn padrsv397(&self) -> &Pad {
        self.pad(397)
    }
    ///0xc70..0xc78 - no description available
    #[inline(always)]
    pub const fn padrsv398(&self) -> &Pad {
        self.pad(398)
    }
    ///0xc78..0xc80 - no description available
    #[inline(always)]
    pub const fn padrsv399(&self) -> &Pad {
        self.pad(399)
    }
    ///0xc80..0xc88 - no description available
    #[inline(always)]
    pub const fn padrsv400(&self) -> &Pad {
        self.pad(400)
    }
    ///0xc88..0xc90 - no description available
    #[inline(always)]
    pub const fn padrsv401(&self) -> &Pad {
        self.pad(401)
    }
    ///0xc90..0xc98 - no description available
    #[inline(always)]
    pub const fn padrsv402(&self) -> &Pad {
        self.pad(402)
    }
    ///0xc98..0xca0 - no description available
    #[inline(always)]
    pub const fn padrsv403(&self) -> &Pad {
        self.pad(403)
    }
    ///0xca0..0xca8 - no description available
    #[inline(always)]
    pub const fn padrsv404(&self) -> &Pad {
        self.pad(404)
    }
    ///0xca8..0xcb0 - no description available
    #[inline(always)]
    pub const fn padrsv405(&self) -> &Pad {
        self.pad(405)
    }
    ///0xcb0..0xcb8 - no description available
    #[inline(always)]
    pub const fn padrsv406(&self) -> &Pad {
        self.pad(406)
    }
    ///0xcb8..0xcc0 - no description available
    #[inline(always)]
    pub const fn padrsv407(&self) -> &Pad {
        self.pad(407)
    }
    ///0xcc0..0xcc8 - no description available
    #[inline(always)]
    pub const fn padrsv408(&self) -> &Pad {
        self.pad(408)
    }
    ///0xcc8..0xcd0 - no description available
    #[inline(always)]
    pub const fn padrsv409(&self) -> &Pad {
        self.pad(409)
    }
    ///0xcd0..0xcd8 - no description available
    #[inline(always)]
    pub const fn padrsv410(&self) -> &Pad {
        self.pad(410)
    }
    ///0xcd8..0xce0 - no description available
    #[inline(always)]
    pub const fn padrsv411(&self) -> &Pad {
        self.pad(411)
    }
    ///0xce0..0xce8 - no description available
    #[inline(always)]
    pub const fn padrsv412(&self) -> &Pad {
        self.pad(412)
    }
    ///0xce8..0xcf0 - no description available
    #[inline(always)]
    pub const fn padrsv413(&self) -> &Pad {
        self.pad(413)
    }
    ///0xcf0..0xcf8 - no description available
    #[inline(always)]
    pub const fn padrsv414(&self) -> &Pad {
        self.pad(414)
    }
    ///0xcf8..0xd00 - no description available
    #[inline(always)]
    pub const fn padrsv415(&self) -> &Pad {
        self.pad(415)
    }
    ///0xd00..0xd08 - no description available
    #[inline(always)]
    pub const fn padpx00(&self) -> &Pad {
        self.pad(416)
    }
    ///0xd08..0xd10 - no description available
    #[inline(always)]
    pub const fn padpx01(&self) -> &Pad {
        self.pad(417)
    }
    ///0xd10..0xd18 - no description available
    #[inline(always)]
    pub const fn padpx02(&self) -> &Pad {
        self.pad(418)
    }
    ///0xd18..0xd20 - no description available
    #[inline(always)]
    pub const fn padpx03(&self) -> &Pad {
        self.pad(419)
    }
    ///0xd20..0xd28 - no description available
    #[inline(always)]
    pub const fn padpx04(&self) -> &Pad {
        self.pad(420)
    }
    ///0xd28..0xd30 - no description available
    #[inline(always)]
    pub const fn padpx05(&self) -> &Pad {
        self.pad(421)
    }
    ///0xd30..0xd38 - no description available
    #[inline(always)]
    pub const fn padpx06(&self) -> &Pad {
        self.pad(422)
    }
    ///0xd38..0xd40 - no description available
    #[inline(always)]
    pub const fn padpx07(&self) -> &Pad {
        self.pad(423)
    }
    ///0xd40..0xd48 - no description available
    #[inline(always)]
    pub const fn padrsv424(&self) -> &Pad {
        self.pad(424)
    }
    ///0xd48..0xd50 - no description available
    #[inline(always)]
    pub const fn padrsv425(&self) -> &Pad {
        self.pad(425)
    }
    ///0xd50..0xd58 - no description available
    #[inline(always)]
    pub const fn padrsv426(&self) -> &Pad {
        self.pad(426)
    }
    ///0xd58..0xd60 - no description available
    #[inline(always)]
    pub const fn padrsv427(&self) -> &Pad {
        self.pad(427)
    }
    ///0xd60..0xd68 - no description available
    #[inline(always)]
    pub const fn padrsv428(&self) -> &Pad {
        self.pad(428)
    }
    ///0xd68..0xd70 - no description available
    #[inline(always)]
    pub const fn padrsv429(&self) -> &Pad {
        self.pad(429)
    }
    ///0xd70..0xd78 - no description available
    #[inline(always)]
    pub const fn padrsv430(&self) -> &Pad {
        self.pad(430)
    }
    ///0xd78..0xd80 - no description available
    #[inline(always)]
    pub const fn padrsv431(&self) -> &Pad {
        self.pad(431)
    }
    ///0xd80..0xd88 - no description available
    #[inline(always)]
    pub const fn padrsv432(&self) -> &Pad {
        self.pad(432)
    }
    ///0xd88..0xd90 - no description available
    #[inline(always)]
    pub const fn padrsv433(&self) -> &Pad {
        self.pad(433)
    }
    ///0xd90..0xd98 - no description available
    #[inline(always)]
    pub const fn padrsv434(&self) -> &Pad {
        self.pad(434)
    }
    ///0xd98..0xda0 - no description available
    #[inline(always)]
    pub const fn padrsv435(&self) -> &Pad {
        self.pad(435)
    }
    ///0xda0..0xda8 - no description available
    #[inline(always)]
    pub const fn padrsv436(&self) -> &Pad {
        self.pad(436)
    }
    ///0xda8..0xdb0 - no description available
    #[inline(always)]
    pub const fn padrsv437(&self) -> &Pad {
        self.pad(437)
    }
    ///0xdb0..0xdb8 - no description available
    #[inline(always)]
    pub const fn padrsv438(&self) -> &Pad {
        self.pad(438)
    }
    ///0xdb8..0xdc0 - no description available
    #[inline(always)]
    pub const fn padrsv439(&self) -> &Pad {
        self.pad(439)
    }
    ///0xdc0..0xdc8 - no description available
    #[inline(always)]
    pub const fn padrsv440(&self) -> &Pad {
        self.pad(440)
    }
    ///0xdc8..0xdd0 - no description available
    #[inline(always)]
    pub const fn padrsv441(&self) -> &Pad {
        self.pad(441)
    }
    ///0xdd0..0xdd8 - no description available
    #[inline(always)]
    pub const fn padrsv442(&self) -> &Pad {
        self.pad(442)
    }
    ///0xdd8..0xde0 - no description available
    #[inline(always)]
    pub const fn padrsv443(&self) -> &Pad {
        self.pad(443)
    }
    ///0xde0..0xde8 - no description available
    #[inline(always)]
    pub const fn padrsv444(&self) -> &Pad {
        self.pad(444)
    }
    ///0xde8..0xdf0 - no description available
    #[inline(always)]
    pub const fn padrsv445(&self) -> &Pad {
        self.pad(445)
    }
    ///0xdf0..0xdf8 - no description available
    #[inline(always)]
    pub const fn padrsv446(&self) -> &Pad {
        self.pad(446)
    }
    ///0xdf8..0xe00 - no description available
    #[inline(always)]
    pub const fn padrsv447(&self) -> &Pad {
        self.pad(447)
    }
    ///0xe00..0xe08 - no description available
    #[inline(always)]
    pub const fn padpy00(&self) -> &Pad {
        self.pad(448)
    }
    ///0xe08..0xe10 - no description available
    #[inline(always)]
    pub const fn padpy01(&self) -> &Pad {
        self.pad(449)
    }
    ///0xe10..0xe18 - no description available
    #[inline(always)]
    pub const fn padpy02(&self) -> &Pad {
        self.pad(450)
    }
    ///0xe18..0xe20 - no description available
    #[inline(always)]
    pub const fn padpy03(&self) -> &Pad {
        self.pad(451)
    }
    ///0xe20..0xe28 - no description available
    #[inline(always)]
    pub const fn padpy04(&self) -> &Pad {
        self.pad(452)
    }
    ///0xe28..0xe30 - no description available
    #[inline(always)]
    pub const fn padpy05(&self) -> &Pad {
        self.pad(453)
    }
    ///0xe30..0xe38 - no description available
    #[inline(always)]
    pub const fn padpy06(&self) -> &Pad {
        self.pad(454)
    }
    ///0xe38..0xe40 - no description available
    #[inline(always)]
    pub const fn padpy07(&self) -> &Pad {
        self.pad(455)
    }
}
///no description available
pub use self::pad::Pad;
///Cluster
///no description available
pub mod pad;
