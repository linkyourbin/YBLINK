#[repr(C)]
///Register block
pub struct RegisterBlock {
    resource: [Resource; 311],
    _reserved1: [u8; 0x0324],
    group0: [Group0; 2],
    _reserved2: [u8; 0xe0],
    affiliate: [Affiliate; 1],
    _reserved3: [u8; 0x10],
    retention: [Retention; 1],
    _reserved4: [u8; 0x06d0],
    power: [Power; 1],
    _reserved5: [u8; 0x03ec],
    reset: [Reset; 2],
    _reserved6: [u8; 0x03e0],
    clock_cpu: [ClockCpu; 1],
    clock: [Clock; 36],
    _reserved8: [u8; 0x036c],
    adcclk: [Adcclk; 2],
    dacclk: [Dacclk; 2],
    _reserved10: [u8; 0x03f0],
    global00: Global00,
    _reserved11: [u8; 0x03fc],
    monitor: (),
    _reserved12: [u8; 0x0400],
    cpu: [Cpu; 1],
}
impl RegisterBlock {
    ///0x00..0x4dc - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `RESOURCEcpu0` register.</div>
    #[inline(always)]
    pub const fn resource(&self, n: usize) -> &Resource {
        &self.resource[n]
    }
    ///Iterator for array of:
    ///0x00..0x4dc - no description available
    #[inline(always)]
    pub fn resource_iter(&self) -> impl Iterator<Item = &Resource> {
        self.resource.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn resourcecpu0(&self) -> &Resource {
        self.resource(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn resourcecpx0(&self) -> &Resource {
        self.resource(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn resourcersv2(&self) -> &Resource {
        self.resource(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn resourcersv3(&self) -> &Resource {
        self.resource(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn resourcersv4(&self) -> &Resource {
        self.resource(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn resourcersv5(&self) -> &Resource {
        self.resource(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn resourcersv6(&self) -> &Resource {
        self.resource(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn resourcersv7(&self) -> &Resource {
        self.resource(7)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn resourcersv8(&self) -> &Resource {
        self.resource(8)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn resourcersv9(&self) -> &Resource {
        self.resource(9)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn resourcersv10(&self) -> &Resource {
        self.resource(10)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn resourcersv11(&self) -> &Resource {
        self.resource(11)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn resourcersv12(&self) -> &Resource {
        self.resource(12)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn resourcersv13(&self) -> &Resource {
        self.resource(13)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn resourcersv14(&self) -> &Resource {
        self.resource(14)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn resourcersv15(&self) -> &Resource {
        self.resource(15)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn resourcersv16(&self) -> &Resource {
        self.resource(16)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn resourcersv17(&self) -> &Resource {
        self.resource(17)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn resourcersv18(&self) -> &Resource {
        self.resource(18)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn resourcersv19(&self) -> &Resource {
        self.resource(19)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn resourcersv20(&self) -> &Resource {
        self.resource(20)
    }
    ///0x54 - no description available
    #[inline(always)]
    pub const fn resourcepow_cpu0(&self) -> &Resource {
        self.resource(21)
    }
    ///0x58 - no description available
    #[inline(always)]
    pub const fn resourcerst_soc(&self) -> &Resource {
        self.resource(22)
    }
    ///0x5c - no description available
    #[inline(always)]
    pub const fn resourcerst_cpu0(&self) -> &Resource {
        self.resource(23)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn resourcersv24(&self) -> &Resource {
        self.resource(24)
    }
    ///0x64 - no description available
    #[inline(always)]
    pub const fn resourcersv25(&self) -> &Resource {
        self.resource(25)
    }
    ///0x68 - no description available
    #[inline(always)]
    pub const fn resourcersv26(&self) -> &Resource {
        self.resource(26)
    }
    ///0x6c - no description available
    #[inline(always)]
    pub const fn resourcersv27(&self) -> &Resource {
        self.resource(27)
    }
    ///0x70 - no description available
    #[inline(always)]
    pub const fn resourcersv28(&self) -> &Resource {
        self.resource(28)
    }
    ///0x74 - no description available
    #[inline(always)]
    pub const fn resourcersv29(&self) -> &Resource {
        self.resource(29)
    }
    ///0x78 - no description available
    #[inline(always)]
    pub const fn resourcersv30(&self) -> &Resource {
        self.resource(30)
    }
    ///0x7c - no description available
    #[inline(always)]
    pub const fn resourcersv31(&self) -> &Resource {
        self.resource(31)
    }
    ///0x80 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_xtal(&self) -> &Resource {
        self.resource(32)
    }
    ///0x84 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_pll0(&self) -> &Resource {
        self.resource(33)
    }
    ///0x88 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk0_pll0(&self) -> &Resource {
        self.resource(34)
    }
    ///0x8c - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk1_pll0(&self) -> &Resource {
        self.resource(35)
    }
    ///0x90 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk2_pll0(&self) -> &Resource {
        self.resource(36)
    }
    ///0x94 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_pll1(&self) -> &Resource {
        self.resource(37)
    }
    ///0x98 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk0_pll1(&self) -> &Resource {
        self.resource(38)
    }
    ///0x9c - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk1_pll1(&self) -> &Resource {
        self.resource(39)
    }
    ///0xa0 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk2_pll1(&self) -> &Resource {
        self.resource(40)
    }
    ///0xa4 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_clk3_pll1(&self) -> &Resource {
        self.resource(41)
    }
    ///0xa8 - no description available
    #[inline(always)]
    pub const fn resourceclk_src_pll0_ref(&self) -> &Resource {
        self.resource(42)
    }
    ///0xac - no description available
    #[inline(always)]
    pub const fn resourceclk_src_pll1_ref(&self) -> &Resource {
        self.resource(43)
    }
    ///0xb0 - no description available
    #[inline(always)]
    pub const fn resourcersv44(&self) -> &Resource {
        self.resource(44)
    }
    ///0xb4 - no description available
    #[inline(always)]
    pub const fn resourcersv45(&self) -> &Resource {
        self.resource(45)
    }
    ///0xb8 - no description available
    #[inline(always)]
    pub const fn resourcersv46(&self) -> &Resource {
        self.resource(46)
    }
    ///0xbc - no description available
    #[inline(always)]
    pub const fn resourcersv47(&self) -> &Resource {
        self.resource(47)
    }
    ///0xc0 - no description available
    #[inline(always)]
    pub const fn resourcersv48(&self) -> &Resource {
        self.resource(48)
    }
    ///0xc4 - no description available
    #[inline(always)]
    pub const fn resourcersv49(&self) -> &Resource {
        self.resource(49)
    }
    ///0xc8 - no description available
    #[inline(always)]
    pub const fn resourcersv50(&self) -> &Resource {
        self.resource(50)
    }
    ///0xcc - no description available
    #[inline(always)]
    pub const fn resourcersv51(&self) -> &Resource {
        self.resource(51)
    }
    ///0xd0 - no description available
    #[inline(always)]
    pub const fn resourcersv52(&self) -> &Resource {
        self.resource(52)
    }
    ///0xd4 - no description available
    #[inline(always)]
    pub const fn resourcersv53(&self) -> &Resource {
        self.resource(53)
    }
    ///0xd8 - no description available
    #[inline(always)]
    pub const fn resourcersv54(&self) -> &Resource {
        self.resource(54)
    }
    ///0xdc - no description available
    #[inline(always)]
    pub const fn resourcersv55(&self) -> &Resource {
        self.resource(55)
    }
    ///0xe0 - no description available
    #[inline(always)]
    pub const fn resourcersv56(&self) -> &Resource {
        self.resource(56)
    }
    ///0xe4 - no description available
    #[inline(always)]
    pub const fn resourcersv57(&self) -> &Resource {
        self.resource(57)
    }
    ///0xe8 - no description available
    #[inline(always)]
    pub const fn resourcersv58(&self) -> &Resource {
        self.resource(58)
    }
    ///0xec - no description available
    #[inline(always)]
    pub const fn resourcersv59(&self) -> &Resource {
        self.resource(59)
    }
    ///0xf0 - no description available
    #[inline(always)]
    pub const fn resourcersv60(&self) -> &Resource {
        self.resource(60)
    }
    ///0xf4 - no description available
    #[inline(always)]
    pub const fn resourcersv61(&self) -> &Resource {
        self.resource(61)
    }
    ///0xf8 - no description available
    #[inline(always)]
    pub const fn resourcersv62(&self) -> &Resource {
        self.resource(62)
    }
    ///0xfc - no description available
    #[inline(always)]
    pub const fn resourcersv63(&self) -> &Resource {
        self.resource(63)
    }
    ///0x100 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_cpu0(&self) -> &Resource {
        self.resource(64)
    }
    ///0x104 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_mct0(&self) -> &Resource {
        self.resource(65)
    }
    ///0x108 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_can0(&self) -> &Resource {
        self.resource(66)
    }
    ///0x10c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_can1(&self) -> &Resource {
        self.resource(67)
    }
    ///0x110 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_can2(&self) -> &Resource {
        self.resource(68)
    }
    ///0x114 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_can3(&self) -> &Resource {
        self.resource(69)
    }
    ///0x118 - no description available
    #[inline(always)]
    pub const fn resourcersv70(&self) -> &Resource {
        self.resource(70)
    }
    ///0x11c - no description available
    #[inline(always)]
    pub const fn resourcersv71(&self) -> &Resource {
        self.resource(71)
    }
    ///0x120 - no description available
    #[inline(always)]
    pub const fn resourcersv72(&self) -> &Resource {
        self.resource(72)
    }
    ///0x124 - no description available
    #[inline(always)]
    pub const fn resourcersv73(&self) -> &Resource {
        self.resource(73)
    }
    ///0x128 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_tmr0(&self) -> &Resource {
        self.resource(74)
    }
    ///0x12c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_tmr1(&self) -> &Resource {
        self.resource(75)
    }
    ///0x130 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_tmr2(&self) -> &Resource {
        self.resource(76)
    }
    ///0x134 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_tmr3(&self) -> &Resource {
        self.resource(77)
    }
    ///0x138 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_i2c0(&self) -> &Resource {
        self.resource(78)
    }
    ///0x13c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_i2c1(&self) -> &Resource {
        self.resource(79)
    }
    ///0x140 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_i2c2(&self) -> &Resource {
        self.resource(80)
    }
    ///0x144 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_i2c3(&self) -> &Resource {
        self.resource(81)
    }
    ///0x148 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_spi0(&self) -> &Resource {
        self.resource(82)
    }
    ///0x14c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_spi1(&self) -> &Resource {
        self.resource(83)
    }
    ///0x150 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_spi2(&self) -> &Resource {
        self.resource(84)
    }
    ///0x154 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_spi3(&self) -> &Resource {
        self.resource(85)
    }
    ///0x158 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt0(&self) -> &Resource {
        self.resource(86)
    }
    ///0x15c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt1(&self) -> &Resource {
        self.resource(87)
    }
    ///0x160 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt2(&self) -> &Resource {
        self.resource(88)
    }
    ///0x164 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt3(&self) -> &Resource {
        self.resource(89)
    }
    ///0x168 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt4(&self) -> &Resource {
        self.resource(90)
    }
    ///0x16c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt5(&self) -> &Resource {
        self.resource(91)
    }
    ///0x170 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt6(&self) -> &Resource {
        self.resource(92)
    }
    ///0x174 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_urt7(&self) -> &Resource {
        self.resource(93)
    }
    ///0x178 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_xpi0(&self) -> &Resource {
        self.resource(94)
    }
    ///0x17c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_ana0(&self) -> &Resource {
        self.resource(95)
    }
    ///0x180 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_ana1(&self) -> &Resource {
        self.resource(96)
    }
    ///0x184 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_ana2(&self) -> &Resource {
        self.resource(97)
    }
    ///0x188 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_ana3(&self) -> &Resource {
        self.resource(98)
    }
    ///0x18c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_ref0(&self) -> &Resource {
        self.resource(99)
    }
    ///0x190 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_ref1(&self) -> &Resource {
        self.resource(100)
    }
    ///0x194 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_adc0(&self) -> &Resource {
        self.resource(101)
    }
    ///0x198 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_adc1(&self) -> &Resource {
        self.resource(102)
    }
    ///0x19c - no description available
    #[inline(always)]
    pub const fn resourceclk_top_dac0(&self) -> &Resource {
        self.resource(103)
    }
    ///0x1a0 - no description available
    #[inline(always)]
    pub const fn resourceclk_top_dac1(&self) -> &Resource {
        self.resource(104)
    }
    ///0x1a4 - no description available
    #[inline(always)]
    pub const fn resourcersv105(&self) -> &Resource {
        self.resource(105)
    }
    ///0x1a8 - no description available
    #[inline(always)]
    pub const fn resourcersv106(&self) -> &Resource {
        self.resource(106)
    }
    ///0x1ac - no description available
    #[inline(always)]
    pub const fn resourcersv107(&self) -> &Resource {
        self.resource(107)
    }
    ///0x1b0 - no description available
    #[inline(always)]
    pub const fn resourcersv108(&self) -> &Resource {
        self.resource(108)
    }
    ///0x1b4 - no description available
    #[inline(always)]
    pub const fn resourcersv109(&self) -> &Resource {
        self.resource(109)
    }
    ///0x1b8 - no description available
    #[inline(always)]
    pub const fn resourcersv110(&self) -> &Resource {
        self.resource(110)
    }
    ///0x1bc - no description available
    #[inline(always)]
    pub const fn resourcersv111(&self) -> &Resource {
        self.resource(111)
    }
    ///0x1c0 - no description available
    #[inline(always)]
    pub const fn resourcersv112(&self) -> &Resource {
        self.resource(112)
    }
    ///0x1c4 - no description available
    #[inline(always)]
    pub const fn resourcersv113(&self) -> &Resource {
        self.resource(113)
    }
    ///0x1c8 - no description available
    #[inline(always)]
    pub const fn resourcersv114(&self) -> &Resource {
        self.resource(114)
    }
    ///0x1cc - no description available
    #[inline(always)]
    pub const fn resourcersv115(&self) -> &Resource {
        self.resource(115)
    }
    ///0x1d0 - no description available
    #[inline(always)]
    pub const fn resourcersv116(&self) -> &Resource {
        self.resource(116)
    }
    ///0x1d4 - no description available
    #[inline(always)]
    pub const fn resourcersv117(&self) -> &Resource {
        self.resource(117)
    }
    ///0x1d8 - no description available
    #[inline(always)]
    pub const fn resourcersv118(&self) -> &Resource {
        self.resource(118)
    }
    ///0x1dc - no description available
    #[inline(always)]
    pub const fn resourcersv119(&self) -> &Resource {
        self.resource(119)
    }
    ///0x1e0 - no description available
    #[inline(always)]
    pub const fn resourcersv120(&self) -> &Resource {
        self.resource(120)
    }
    ///0x1e4 - no description available
    #[inline(always)]
    pub const fn resourcersv121(&self) -> &Resource {
        self.resource(121)
    }
    ///0x1e8 - no description available
    #[inline(always)]
    pub const fn resourcersv122(&self) -> &Resource {
        self.resource(122)
    }
    ///0x1ec - no description available
    #[inline(always)]
    pub const fn resourcersv123(&self) -> &Resource {
        self.resource(123)
    }
    ///0x1f0 - no description available
    #[inline(always)]
    pub const fn resourcersv124(&self) -> &Resource {
        self.resource(124)
    }
    ///0x1f4 - no description available
    #[inline(always)]
    pub const fn resourcersv125(&self) -> &Resource {
        self.resource(125)
    }
    ///0x1f8 - no description available
    #[inline(always)]
    pub const fn resourcersv126(&self) -> &Resource {
        self.resource(126)
    }
    ///0x1fc - no description available
    #[inline(always)]
    pub const fn resourcersv127(&self) -> &Resource {
        self.resource(127)
    }
    ///0x200 - no description available
    #[inline(always)]
    pub const fn resourcersv128(&self) -> &Resource {
        self.resource(128)
    }
    ///0x204 - no description available
    #[inline(always)]
    pub const fn resourcersv129(&self) -> &Resource {
        self.resource(129)
    }
    ///0x208 - no description available
    #[inline(always)]
    pub const fn resourcersv130(&self) -> &Resource {
        self.resource(130)
    }
    ///0x20c - no description available
    #[inline(always)]
    pub const fn resourcersv131(&self) -> &Resource {
        self.resource(131)
    }
    ///0x210 - no description available
    #[inline(always)]
    pub const fn resourcersv132(&self) -> &Resource {
        self.resource(132)
    }
    ///0x214 - no description available
    #[inline(always)]
    pub const fn resourcersv133(&self) -> &Resource {
        self.resource(133)
    }
    ///0x218 - no description available
    #[inline(always)]
    pub const fn resourcersv134(&self) -> &Resource {
        self.resource(134)
    }
    ///0x21c - no description available
    #[inline(always)]
    pub const fn resourcersv135(&self) -> &Resource {
        self.resource(135)
    }
    ///0x220 - no description available
    #[inline(always)]
    pub const fn resourcersv136(&self) -> &Resource {
        self.resource(136)
    }
    ///0x224 - no description available
    #[inline(always)]
    pub const fn resourcersv137(&self) -> &Resource {
        self.resource(137)
    }
    ///0x228 - no description available
    #[inline(always)]
    pub const fn resourcersv138(&self) -> &Resource {
        self.resource(138)
    }
    ///0x22c - no description available
    #[inline(always)]
    pub const fn resourcersv139(&self) -> &Resource {
        self.resource(139)
    }
    ///0x230 - no description available
    #[inline(always)]
    pub const fn resourcersv140(&self) -> &Resource {
        self.resource(140)
    }
    ///0x234 - no description available
    #[inline(always)]
    pub const fn resourcersv141(&self) -> &Resource {
        self.resource(141)
    }
    ///0x238 - no description available
    #[inline(always)]
    pub const fn resourcersv142(&self) -> &Resource {
        self.resource(142)
    }
    ///0x23c - no description available
    #[inline(always)]
    pub const fn resourcersv143(&self) -> &Resource {
        self.resource(143)
    }
    ///0x240 - no description available
    #[inline(always)]
    pub const fn resourcersv144(&self) -> &Resource {
        self.resource(144)
    }
    ///0x244 - no description available
    #[inline(always)]
    pub const fn resourcersv145(&self) -> &Resource {
        self.resource(145)
    }
    ///0x248 - no description available
    #[inline(always)]
    pub const fn resourcersv146(&self) -> &Resource {
        self.resource(146)
    }
    ///0x24c - no description available
    #[inline(always)]
    pub const fn resourcersv147(&self) -> &Resource {
        self.resource(147)
    }
    ///0x250 - no description available
    #[inline(always)]
    pub const fn resourcersv148(&self) -> &Resource {
        self.resource(148)
    }
    ///0x254 - no description available
    #[inline(always)]
    pub const fn resourcersv149(&self) -> &Resource {
        self.resource(149)
    }
    ///0x258 - no description available
    #[inline(always)]
    pub const fn resourcersv150(&self) -> &Resource {
        self.resource(150)
    }
    ///0x25c - no description available
    #[inline(always)]
    pub const fn resourcersv151(&self) -> &Resource {
        self.resource(151)
    }
    ///0x260 - no description available
    #[inline(always)]
    pub const fn resourcersv152(&self) -> &Resource {
        self.resource(152)
    }
    ///0x264 - no description available
    #[inline(always)]
    pub const fn resourcersv153(&self) -> &Resource {
        self.resource(153)
    }
    ///0x268 - no description available
    #[inline(always)]
    pub const fn resourcersv154(&self) -> &Resource {
        self.resource(154)
    }
    ///0x26c - no description available
    #[inline(always)]
    pub const fn resourcersv155(&self) -> &Resource {
        self.resource(155)
    }
    ///0x270 - no description available
    #[inline(always)]
    pub const fn resourcersv156(&self) -> &Resource {
        self.resource(156)
    }
    ///0x274 - no description available
    #[inline(always)]
    pub const fn resourcersv157(&self) -> &Resource {
        self.resource(157)
    }
    ///0x278 - no description available
    #[inline(always)]
    pub const fn resourcersv158(&self) -> &Resource {
        self.resource(158)
    }
    ///0x27c - no description available
    #[inline(always)]
    pub const fn resourcersv159(&self) -> &Resource {
        self.resource(159)
    }
    ///0x280 - no description available
    #[inline(always)]
    pub const fn resourcersv160(&self) -> &Resource {
        self.resource(160)
    }
    ///0x284 - no description available
    #[inline(always)]
    pub const fn resourcersv161(&self) -> &Resource {
        self.resource(161)
    }
    ///0x288 - no description available
    #[inline(always)]
    pub const fn resourcersv162(&self) -> &Resource {
        self.resource(162)
    }
    ///0x28c - no description available
    #[inline(always)]
    pub const fn resourcersv163(&self) -> &Resource {
        self.resource(163)
    }
    ///0x290 - no description available
    #[inline(always)]
    pub const fn resourcersv164(&self) -> &Resource {
        self.resource(164)
    }
    ///0x294 - no description available
    #[inline(always)]
    pub const fn resourcersv165(&self) -> &Resource {
        self.resource(165)
    }
    ///0x298 - no description available
    #[inline(always)]
    pub const fn resourcersv166(&self) -> &Resource {
        self.resource(166)
    }
    ///0x29c - no description available
    #[inline(always)]
    pub const fn resourcersv167(&self) -> &Resource {
        self.resource(167)
    }
    ///0x2a0 - no description available
    #[inline(always)]
    pub const fn resourcersv168(&self) -> &Resource {
        self.resource(168)
    }
    ///0x2a4 - no description available
    #[inline(always)]
    pub const fn resourcersv169(&self) -> &Resource {
        self.resource(169)
    }
    ///0x2a8 - no description available
    #[inline(always)]
    pub const fn resourcersv170(&self) -> &Resource {
        self.resource(170)
    }
    ///0x2ac - no description available
    #[inline(always)]
    pub const fn resourcersv171(&self) -> &Resource {
        self.resource(171)
    }
    ///0x2b0 - no description available
    #[inline(always)]
    pub const fn resourcersv172(&self) -> &Resource {
        self.resource(172)
    }
    ///0x2b4 - no description available
    #[inline(always)]
    pub const fn resourcersv173(&self) -> &Resource {
        self.resource(173)
    }
    ///0x2b8 - no description available
    #[inline(always)]
    pub const fn resourcersv174(&self) -> &Resource {
        self.resource(174)
    }
    ///0x2bc - no description available
    #[inline(always)]
    pub const fn resourcersv175(&self) -> &Resource {
        self.resource(175)
    }
    ///0x2c0 - no description available
    #[inline(always)]
    pub const fn resourcersv176(&self) -> &Resource {
        self.resource(176)
    }
    ///0x2c4 - no description available
    #[inline(always)]
    pub const fn resourcersv177(&self) -> &Resource {
        self.resource(177)
    }
    ///0x2c8 - no description available
    #[inline(always)]
    pub const fn resourcersv178(&self) -> &Resource {
        self.resource(178)
    }
    ///0x2cc - no description available
    #[inline(always)]
    pub const fn resourcersv179(&self) -> &Resource {
        self.resource(179)
    }
    ///0x2d0 - no description available
    #[inline(always)]
    pub const fn resourcersv180(&self) -> &Resource {
        self.resource(180)
    }
    ///0x2d4 - no description available
    #[inline(always)]
    pub const fn resourcersv181(&self) -> &Resource {
        self.resource(181)
    }
    ///0x2d8 - no description available
    #[inline(always)]
    pub const fn resourcersv182(&self) -> &Resource {
        self.resource(182)
    }
    ///0x2dc - no description available
    #[inline(always)]
    pub const fn resourcersv183(&self) -> &Resource {
        self.resource(183)
    }
    ///0x2e0 - no description available
    #[inline(always)]
    pub const fn resourcersv184(&self) -> &Resource {
        self.resource(184)
    }
    ///0x2e4 - no description available
    #[inline(always)]
    pub const fn resourcersv185(&self) -> &Resource {
        self.resource(185)
    }
    ///0x2e8 - no description available
    #[inline(always)]
    pub const fn resourcersv186(&self) -> &Resource {
        self.resource(186)
    }
    ///0x2ec - no description available
    #[inline(always)]
    pub const fn resourcersv187(&self) -> &Resource {
        self.resource(187)
    }
    ///0x2f0 - no description available
    #[inline(always)]
    pub const fn resourcersv188(&self) -> &Resource {
        self.resource(188)
    }
    ///0x2f4 - no description available
    #[inline(always)]
    pub const fn resourcersv189(&self) -> &Resource {
        self.resource(189)
    }
    ///0x2f8 - no description available
    #[inline(always)]
    pub const fn resourcersv190(&self) -> &Resource {
        self.resource(190)
    }
    ///0x2fc - no description available
    #[inline(always)]
    pub const fn resourcersv191(&self) -> &Resource {
        self.resource(191)
    }
    ///0x300 - no description available
    #[inline(always)]
    pub const fn resourcersv192(&self) -> &Resource {
        self.resource(192)
    }
    ///0x304 - no description available
    #[inline(always)]
    pub const fn resourcersv193(&self) -> &Resource {
        self.resource(193)
    }
    ///0x308 - no description available
    #[inline(always)]
    pub const fn resourcersv194(&self) -> &Resource {
        self.resource(194)
    }
    ///0x30c - no description available
    #[inline(always)]
    pub const fn resourcersv195(&self) -> &Resource {
        self.resource(195)
    }
    ///0x310 - no description available
    #[inline(always)]
    pub const fn resourcersv196(&self) -> &Resource {
        self.resource(196)
    }
    ///0x314 - no description available
    #[inline(always)]
    pub const fn resourcersv197(&self) -> &Resource {
        self.resource(197)
    }
    ///0x318 - no description available
    #[inline(always)]
    pub const fn resourcersv198(&self) -> &Resource {
        self.resource(198)
    }
    ///0x31c - no description available
    #[inline(always)]
    pub const fn resourcersv199(&self) -> &Resource {
        self.resource(199)
    }
    ///0x320 - no description available
    #[inline(always)]
    pub const fn resourcersv200(&self) -> &Resource {
        self.resource(200)
    }
    ///0x324 - no description available
    #[inline(always)]
    pub const fn resourcersv201(&self) -> &Resource {
        self.resource(201)
    }
    ///0x328 - no description available
    #[inline(always)]
    pub const fn resourcersv202(&self) -> &Resource {
        self.resource(202)
    }
    ///0x32c - no description available
    #[inline(always)]
    pub const fn resourcersv203(&self) -> &Resource {
        self.resource(203)
    }
    ///0x330 - no description available
    #[inline(always)]
    pub const fn resourcersv204(&self) -> &Resource {
        self.resource(204)
    }
    ///0x334 - no description available
    #[inline(always)]
    pub const fn resourcersv205(&self) -> &Resource {
        self.resource(205)
    }
    ///0x338 - no description available
    #[inline(always)]
    pub const fn resourcersv206(&self) -> &Resource {
        self.resource(206)
    }
    ///0x33c - no description available
    #[inline(always)]
    pub const fn resourcersv207(&self) -> &Resource {
        self.resource(207)
    }
    ///0x340 - no description available
    #[inline(always)]
    pub const fn resourcersv208(&self) -> &Resource {
        self.resource(208)
    }
    ///0x344 - no description available
    #[inline(always)]
    pub const fn resourcersv209(&self) -> &Resource {
        self.resource(209)
    }
    ///0x348 - no description available
    #[inline(always)]
    pub const fn resourcersv210(&self) -> &Resource {
        self.resource(210)
    }
    ///0x34c - no description available
    #[inline(always)]
    pub const fn resourcersv211(&self) -> &Resource {
        self.resource(211)
    }
    ///0x350 - no description available
    #[inline(always)]
    pub const fn resourcersv212(&self) -> &Resource {
        self.resource(212)
    }
    ///0x354 - no description available
    #[inline(always)]
    pub const fn resourcersv213(&self) -> &Resource {
        self.resource(213)
    }
    ///0x358 - no description available
    #[inline(always)]
    pub const fn resourcersv214(&self) -> &Resource {
        self.resource(214)
    }
    ///0x35c - no description available
    #[inline(always)]
    pub const fn resourcersv215(&self) -> &Resource {
        self.resource(215)
    }
    ///0x360 - no description available
    #[inline(always)]
    pub const fn resourcersv216(&self) -> &Resource {
        self.resource(216)
    }
    ///0x364 - no description available
    #[inline(always)]
    pub const fn resourcersv217(&self) -> &Resource {
        self.resource(217)
    }
    ///0x368 - no description available
    #[inline(always)]
    pub const fn resourcersv218(&self) -> &Resource {
        self.resource(218)
    }
    ///0x36c - no description available
    #[inline(always)]
    pub const fn resourcersv219(&self) -> &Resource {
        self.resource(219)
    }
    ///0x370 - no description available
    #[inline(always)]
    pub const fn resourcersv220(&self) -> &Resource {
        self.resource(220)
    }
    ///0x374 - no description available
    #[inline(always)]
    pub const fn resourcersv221(&self) -> &Resource {
        self.resource(221)
    }
    ///0x378 - no description available
    #[inline(always)]
    pub const fn resourcersv222(&self) -> &Resource {
        self.resource(222)
    }
    ///0x37c - no description available
    #[inline(always)]
    pub const fn resourcersv223(&self) -> &Resource {
        self.resource(223)
    }
    ///0x380 - no description available
    #[inline(always)]
    pub const fn resourcersv224(&self) -> &Resource {
        self.resource(224)
    }
    ///0x384 - no description available
    #[inline(always)]
    pub const fn resourcersv225(&self) -> &Resource {
        self.resource(225)
    }
    ///0x388 - no description available
    #[inline(always)]
    pub const fn resourcersv226(&self) -> &Resource {
        self.resource(226)
    }
    ///0x38c - no description available
    #[inline(always)]
    pub const fn resourcersv227(&self) -> &Resource {
        self.resource(227)
    }
    ///0x390 - no description available
    #[inline(always)]
    pub const fn resourcersv228(&self) -> &Resource {
        self.resource(228)
    }
    ///0x394 - no description available
    #[inline(always)]
    pub const fn resourcersv229(&self) -> &Resource {
        self.resource(229)
    }
    ///0x398 - no description available
    #[inline(always)]
    pub const fn resourcersv230(&self) -> &Resource {
        self.resource(230)
    }
    ///0x39c - no description available
    #[inline(always)]
    pub const fn resourcersv231(&self) -> &Resource {
        self.resource(231)
    }
    ///0x3a0 - no description available
    #[inline(always)]
    pub const fn resourcersv232(&self) -> &Resource {
        self.resource(232)
    }
    ///0x3a4 - no description available
    #[inline(always)]
    pub const fn resourcersv233(&self) -> &Resource {
        self.resource(233)
    }
    ///0x3a8 - no description available
    #[inline(always)]
    pub const fn resourcersv234(&self) -> &Resource {
        self.resource(234)
    }
    ///0x3ac - no description available
    #[inline(always)]
    pub const fn resourcersv235(&self) -> &Resource {
        self.resource(235)
    }
    ///0x3b0 - no description available
    #[inline(always)]
    pub const fn resourcersv236(&self) -> &Resource {
        self.resource(236)
    }
    ///0x3b4 - no description available
    #[inline(always)]
    pub const fn resourcersv237(&self) -> &Resource {
        self.resource(237)
    }
    ///0x3b8 - no description available
    #[inline(always)]
    pub const fn resourcersv238(&self) -> &Resource {
        self.resource(238)
    }
    ///0x3bc - no description available
    #[inline(always)]
    pub const fn resourcersv239(&self) -> &Resource {
        self.resource(239)
    }
    ///0x3c0 - no description available
    #[inline(always)]
    pub const fn resourcersv240(&self) -> &Resource {
        self.resource(240)
    }
    ///0x3c4 - no description available
    #[inline(always)]
    pub const fn resourcersv241(&self) -> &Resource {
        self.resource(241)
    }
    ///0x3c8 - no description available
    #[inline(always)]
    pub const fn resourcersv242(&self) -> &Resource {
        self.resource(242)
    }
    ///0x3cc - no description available
    #[inline(always)]
    pub const fn resourcersv243(&self) -> &Resource {
        self.resource(243)
    }
    ///0x3d0 - no description available
    #[inline(always)]
    pub const fn resourcersv244(&self) -> &Resource {
        self.resource(244)
    }
    ///0x3d4 - no description available
    #[inline(always)]
    pub const fn resourcersv245(&self) -> &Resource {
        self.resource(245)
    }
    ///0x3d8 - no description available
    #[inline(always)]
    pub const fn resourcersv246(&self) -> &Resource {
        self.resource(246)
    }
    ///0x3dc - no description available
    #[inline(always)]
    pub const fn resourcersv247(&self) -> &Resource {
        self.resource(247)
    }
    ///0x3e0 - no description available
    #[inline(always)]
    pub const fn resourcersv248(&self) -> &Resource {
        self.resource(248)
    }
    ///0x3e4 - no description available
    #[inline(always)]
    pub const fn resourcersv249(&self) -> &Resource {
        self.resource(249)
    }
    ///0x3e8 - no description available
    #[inline(always)]
    pub const fn resourcersv250(&self) -> &Resource {
        self.resource(250)
    }
    ///0x3ec - no description available
    #[inline(always)]
    pub const fn resourcersv251(&self) -> &Resource {
        self.resource(251)
    }
    ///0x3f0 - no description available
    #[inline(always)]
    pub const fn resourcersv252(&self) -> &Resource {
        self.resource(252)
    }
    ///0x3f4 - no description available
    #[inline(always)]
    pub const fn resourcersv253(&self) -> &Resource {
        self.resource(253)
    }
    ///0x3f8 - no description available
    #[inline(always)]
    pub const fn resourcersv254(&self) -> &Resource {
        self.resource(254)
    }
    ///0x3fc - no description available
    #[inline(always)]
    pub const fn resourcersv255(&self) -> &Resource {
        self.resource(255)
    }
    ///0x400 - no description available
    #[inline(always)]
    pub const fn resourceahb0(&self) -> &Resource {
        self.resource(256)
    }
    ///0x404 - no description available
    #[inline(always)]
    pub const fn resourcelmm0(&self) -> &Resource {
        self.resource(257)
    }
    ///0x408 - no description available
    #[inline(always)]
    pub const fn resourcemct0(&self) -> &Resource {
        self.resource(258)
    }
    ///0x40c - no description available
    #[inline(always)]
    pub const fn resourcerom0(&self) -> &Resource {
        self.resource(259)
    }
    ///0x410 - no description available
    #[inline(always)]
    pub const fn resourcecan0(&self) -> &Resource {
        self.resource(260)
    }
    ///0x414 - no description available
    #[inline(always)]
    pub const fn resourcecan1(&self) -> &Resource {
        self.resource(261)
    }
    ///0x418 - no description available
    #[inline(always)]
    pub const fn resourcecan2(&self) -> &Resource {
        self.resource(262)
    }
    ///0x41c - no description available
    #[inline(always)]
    pub const fn resourcecan3(&self) -> &Resource {
        self.resource(263)
    }
    ///0x420 - no description available
    #[inline(always)]
    pub const fn resourceptpc(&self) -> &Resource {
        self.resource(264)
    }
    ///0x424 - no description available
    #[inline(always)]
    pub const fn resourcersv265(&self) -> &Resource {
        self.resource(265)
    }
    ///0x428 - no description available
    #[inline(always)]
    pub const fn resourcersv266(&self) -> &Resource {
        self.resource(266)
    }
    ///0x42c - no description available
    #[inline(always)]
    pub const fn resourcersv267(&self) -> &Resource {
        self.resource(267)
    }
    ///0x430 - no description available
    #[inline(always)]
    pub const fn resourcersv268(&self) -> &Resource {
        self.resource(268)
    }
    ///0x434 - no description available
    #[inline(always)]
    pub const fn resourcetmr0(&self) -> &Resource {
        self.resource(269)
    }
    ///0x438 - no description available
    #[inline(always)]
    pub const fn resourcetmr1(&self) -> &Resource {
        self.resource(270)
    }
    ///0x43c - no description available
    #[inline(always)]
    pub const fn resourcetmr2(&self) -> &Resource {
        self.resource(271)
    }
    ///0x440 - no description available
    #[inline(always)]
    pub const fn resourcetmr3(&self) -> &Resource {
        self.resource(272)
    }
    ///0x444 - no description available
    #[inline(always)]
    pub const fn resourcei2c0(&self) -> &Resource {
        self.resource(273)
    }
    ///0x448 - no description available
    #[inline(always)]
    pub const fn resourcei2c1(&self) -> &Resource {
        self.resource(274)
    }
    ///0x44c - no description available
    #[inline(always)]
    pub const fn resourcei2c2(&self) -> &Resource {
        self.resource(275)
    }
    ///0x450 - no description available
    #[inline(always)]
    pub const fn resourcei2c3(&self) -> &Resource {
        self.resource(276)
    }
    ///0x454 - no description available
    #[inline(always)]
    pub const fn resourcespi0(&self) -> &Resource {
        self.resource(277)
    }
    ///0x458 - no description available
    #[inline(always)]
    pub const fn resourcespi1(&self) -> &Resource {
        self.resource(278)
    }
    ///0x45c - no description available
    #[inline(always)]
    pub const fn resourcespi2(&self) -> &Resource {
        self.resource(279)
    }
    ///0x460 - no description available
    #[inline(always)]
    pub const fn resourcespi3(&self) -> &Resource {
        self.resource(280)
    }
    ///0x464 - no description available
    #[inline(always)]
    pub const fn resourceurt0(&self) -> &Resource {
        self.resource(281)
    }
    ///0x468 - no description available
    #[inline(always)]
    pub const fn resourceurt1(&self) -> &Resource {
        self.resource(282)
    }
    ///0x46c - no description available
    #[inline(always)]
    pub const fn resourceurt2(&self) -> &Resource {
        self.resource(283)
    }
    ///0x470 - no description available
    #[inline(always)]
    pub const fn resourceurt3(&self) -> &Resource {
        self.resource(284)
    }
    ///0x474 - no description available
    #[inline(always)]
    pub const fn resourceurt4(&self) -> &Resource {
        self.resource(285)
    }
    ///0x478 - no description available
    #[inline(always)]
    pub const fn resourceurt5(&self) -> &Resource {
        self.resource(286)
    }
    ///0x47c - no description available
    #[inline(always)]
    pub const fn resourceurt6(&self) -> &Resource {
        self.resource(287)
    }
    ///0x480 - no description available
    #[inline(always)]
    pub const fn resourceurt7(&self) -> &Resource {
        self.resource(288)
    }
    ///0x484 - no description available
    #[inline(always)]
    pub const fn resourcewdg0(&self) -> &Resource {
        self.resource(289)
    }
    ///0x488 - no description available
    #[inline(always)]
    pub const fn resourcewdg1(&self) -> &Resource {
        self.resource(290)
    }
    ///0x48c - no description available
    #[inline(always)]
    pub const fn resourcembx0(&self) -> &Resource {
        self.resource(291)
    }
    ///0x490 - no description available
    #[inline(always)]
    pub const fn resourcetsns(&self) -> &Resource {
        self.resource(292)
    }
    ///0x494 - no description available
    #[inline(always)]
    pub const fn resourcecrc0(&self) -> &Resource {
        self.resource(293)
    }
    ///0x498 - no description available
    #[inline(always)]
    pub const fn resourceadc0(&self) -> &Resource {
        self.resource(294)
    }
    ///0x49c - no description available
    #[inline(always)]
    pub const fn resourceadc1(&self) -> &Resource {
        self.resource(295)
    }
    ///0x4a0 - no description available
    #[inline(always)]
    pub const fn resourcedac0(&self) -> &Resource {
        self.resource(296)
    }
    ///0x4a4 - no description available
    #[inline(always)]
    pub const fn resourcedac1(&self) -> &Resource {
        self.resource(297)
    }
    ///0x4a8 - no description available
    #[inline(always)]
    pub const fn resourceacmp(&self) -> &Resource {
        self.resource(298)
    }
    ///0x4ac - no description available
    #[inline(always)]
    pub const fn resourceopa0(&self) -> &Resource {
        self.resource(299)
    }
    ///0x4b0 - no description available
    #[inline(always)]
    pub const fn resourceopa1(&self) -> &Resource {
        self.resource(300)
    }
    ///0x4b4 - no description available
    #[inline(always)]
    pub const fn resourcemot0(&self) -> &Resource {
        self.resource(301)
    }
    ///0x4b8 - no description available
    #[inline(always)]
    pub const fn resourcerng0(&self) -> &Resource {
        self.resource(302)
    }
    ///0x4bc - no description available
    #[inline(always)]
    pub const fn resourcesdp0(&self) -> &Resource {
        self.resource(303)
    }
    ///0x4c0 - no description available
    #[inline(always)]
    pub const fn resourcekman(&self) -> &Resource {
        self.resource(304)
    }
    ///0x4c4 - no description available
    #[inline(always)]
    pub const fn resourcegpio(&self) -> &Resource {
        self.resource(305)
    }
    ///0x4c8 - no description available
    #[inline(always)]
    pub const fn resourcehdma(&self) -> &Resource {
        self.resource(306)
    }
    ///0x4cc - no description available
    #[inline(always)]
    pub const fn resourcexpi0(&self) -> &Resource {
        self.resource(307)
    }
    ///0x4d0 - no description available
    #[inline(always)]
    pub const fn resourceusb0(&self) -> &Resource {
        self.resource(308)
    }
    ///0x4d4 - no description available
    #[inline(always)]
    pub const fn resourceref0(&self) -> &Resource {
        self.resource(309)
    }
    ///0x4d8 - no description available
    #[inline(always)]
    pub const fn resourceref1(&self) -> &Resource {
        self.resource(310)
    }
    ///0x800..0x820 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `GROUP0link0` cluster.</div>
    #[inline(always)]
    pub const fn group0(&self, n: usize) -> &Group0 {
        &self.group0[n]
    }
    ///Iterator for array of:
    ///0x800..0x820 - no description available
    #[inline(always)]
    pub fn group0_iter(&self) -> impl Iterator<Item = &Group0> {
        self.group0.iter()
    }
    ///0x800..0x810 - no description available
    #[inline(always)]
    pub const fn group0link0(&self) -> &Group0 {
        self.group0(0)
    }
    ///0x810..0x820 - no description available
    #[inline(always)]
    pub const fn group0link1(&self) -> &Group0 {
        self.group0(1)
    }
    ///0x900..0x910 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `AFFILIATEcpu0` cluster.</div>
    #[inline(always)]
    pub const fn affiliate(&self, n: usize) -> &Affiliate {
        &self.affiliate[n]
    }
    ///Iterator for array of:
    ///0x900..0x910 - no description available
    #[inline(always)]
    pub fn affiliate_iter(&self) -> impl Iterator<Item = &Affiliate> {
        self.affiliate.iter()
    }
    ///0x900..0x910 - no description available
    #[inline(always)]
    pub const fn affiliatecpu0(&self) -> &Affiliate {
        self.affiliate(0)
    }
    ///0x920..0x930 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `RETENTIONcpu0` cluster.</div>
    #[inline(always)]
    pub const fn retention(&self, n: usize) -> &Retention {
        &self.retention[n]
    }
    ///Iterator for array of:
    ///0x920..0x930 - no description available
    #[inline(always)]
    pub fn retention_iter(&self) -> impl Iterator<Item = &Retention> {
        self.retention.iter()
    }
    ///0x920..0x930 - no description available
    #[inline(always)]
    pub const fn retentioncpu0(&self) -> &Retention {
        self.retention(0)
    }
    ///0x1000..0x1014 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `POWERcpu0` cluster.</div>
    #[inline(always)]
    pub const fn power(&self, n: usize) -> &Power {
        &self.power[n]
    }
    ///Iterator for array of:
    ///0x1000..0x1014 - no description available
    #[inline(always)]
    pub fn power_iter(&self) -> impl Iterator<Item = &Power> {
        self.power.iter()
    }
    ///0x1000..0x1014 - no description available
    #[inline(always)]
    pub const fn powercpu0(&self) -> &Power {
        self.power(0)
    }
    ///0x1400..0x1420 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `RESETsoc` cluster.</div>
    #[inline(always)]
    pub const fn reset(&self, n: usize) -> &Reset {
        &self.reset[n]
    }
    ///Iterator for array of:
    ///0x1400..0x1420 - no description available
    #[inline(always)]
    pub fn reset_iter(&self) -> impl Iterator<Item = &Reset> {
        self.reset.iter()
    }
    ///0x1400..0x1410 - no description available
    #[inline(always)]
    pub const fn resetsoc(&self) -> &Reset {
        self.reset(0)
    }
    ///0x1410..0x1420 - no description available
    #[inline(always)]
    pub const fn resetcpu0(&self) -> &Reset {
        self.reset(1)
    }
    ///0x1800 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CLOCK_CPUclk_top_cpu0` register.</div>
    #[inline(always)]
    pub const fn clock_cpu(&self, n: usize) -> &ClockCpu {
        &self.clock_cpu[n]
    }
    ///Iterator for array of:
    ///0x1800 - no description available
    #[inline(always)]
    pub fn clock_cpu_iter(&self) -> impl Iterator<Item = &ClockCpu> {
        self.clock_cpu.iter()
    }
    ///0x1800 - no description available
    #[inline(always)]
    pub const fn clock_cpuclk_top_cpu0(&self) -> &ClockCpu {
        self.clock_cpu(0)
    }
    ///0x1804..0x1894 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CLOCKclk_top_mct0` register.</div>
    #[inline(always)]
    pub const fn clock(&self, n: usize) -> &Clock {
        &self.clock[n]
    }
    ///Iterator for array of:
    ///0x1804..0x1894 - no description available
    #[inline(always)]
    pub fn clock_iter(&self) -> impl Iterator<Item = &Clock> {
        self.clock.iter()
    }
    ///0x1804 - no description available
    #[inline(always)]
    pub const fn clockclk_top_mct0(&self) -> &Clock {
        self.clock(0)
    }
    ///0x1808 - no description available
    #[inline(always)]
    pub const fn clockclk_top_can0(&self) -> &Clock {
        self.clock(1)
    }
    ///0x180c - no description available
    #[inline(always)]
    pub const fn clockclk_top_can1(&self) -> &Clock {
        self.clock(2)
    }
    ///0x1810 - no description available
    #[inline(always)]
    pub const fn clockclk_top_can2(&self) -> &Clock {
        self.clock(3)
    }
    ///0x1814 - no description available
    #[inline(always)]
    pub const fn clockclk_top_can3(&self) -> &Clock {
        self.clock(4)
    }
    ///0x1818 - no description available
    #[inline(always)]
    pub const fn clockrsv5(&self) -> &Clock {
        self.clock(5)
    }
    ///0x181c - no description available
    #[inline(always)]
    pub const fn clockrsv6(&self) -> &Clock {
        self.clock(6)
    }
    ///0x1820 - no description available
    #[inline(always)]
    pub const fn clockrsv7(&self) -> &Clock {
        self.clock(7)
    }
    ///0x1824 - no description available
    #[inline(always)]
    pub const fn clockrsv8(&self) -> &Clock {
        self.clock(8)
    }
    ///0x1828 - no description available
    #[inline(always)]
    pub const fn clockclk_top_tmr0(&self) -> &Clock {
        self.clock(9)
    }
    ///0x182c - no description available
    #[inline(always)]
    pub const fn clockclk_top_tmr1(&self) -> &Clock {
        self.clock(10)
    }
    ///0x1830 - no description available
    #[inline(always)]
    pub const fn clockclk_top_tmr2(&self) -> &Clock {
        self.clock(11)
    }
    ///0x1834 - no description available
    #[inline(always)]
    pub const fn clockclk_top_tmr3(&self) -> &Clock {
        self.clock(12)
    }
    ///0x1838 - no description available
    #[inline(always)]
    pub const fn clockclk_top_i2c0(&self) -> &Clock {
        self.clock(13)
    }
    ///0x183c - no description available
    #[inline(always)]
    pub const fn clockclk_top_i2c1(&self) -> &Clock {
        self.clock(14)
    }
    ///0x1840 - no description available
    #[inline(always)]
    pub const fn clockclk_top_i2c2(&self) -> &Clock {
        self.clock(15)
    }
    ///0x1844 - no description available
    #[inline(always)]
    pub const fn clockclk_top_i2c3(&self) -> &Clock {
        self.clock(16)
    }
    ///0x1848 - no description available
    #[inline(always)]
    pub const fn clockclk_top_spi0(&self) -> &Clock {
        self.clock(17)
    }
    ///0x184c - no description available
    #[inline(always)]
    pub const fn clockclk_top_spi1(&self) -> &Clock {
        self.clock(18)
    }
    ///0x1850 - no description available
    #[inline(always)]
    pub const fn clockclk_top_spi2(&self) -> &Clock {
        self.clock(19)
    }
    ///0x1854 - no description available
    #[inline(always)]
    pub const fn clockclk_top_spi3(&self) -> &Clock {
        self.clock(20)
    }
    ///0x1858 - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt0(&self) -> &Clock {
        self.clock(21)
    }
    ///0x185c - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt1(&self) -> &Clock {
        self.clock(22)
    }
    ///0x1860 - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt2(&self) -> &Clock {
        self.clock(23)
    }
    ///0x1864 - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt3(&self) -> &Clock {
        self.clock(24)
    }
    ///0x1868 - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt4(&self) -> &Clock {
        self.clock(25)
    }
    ///0x186c - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt5(&self) -> &Clock {
        self.clock(26)
    }
    ///0x1870 - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt6(&self) -> &Clock {
        self.clock(27)
    }
    ///0x1874 - no description available
    #[inline(always)]
    pub const fn clockclk_top_urt7(&self) -> &Clock {
        self.clock(28)
    }
    ///0x1878 - no description available
    #[inline(always)]
    pub const fn clockclk_top_xpi0(&self) -> &Clock {
        self.clock(29)
    }
    ///0x187c - no description available
    #[inline(always)]
    pub const fn clockclk_top_ana0(&self) -> &Clock {
        self.clock(30)
    }
    ///0x1880 - no description available
    #[inline(always)]
    pub const fn clockclk_top_ana1(&self) -> &Clock {
        self.clock(31)
    }
    ///0x1884 - no description available
    #[inline(always)]
    pub const fn clockclk_top_ana2(&self) -> &Clock {
        self.clock(32)
    }
    ///0x1888 - no description available
    #[inline(always)]
    pub const fn clockclk_top_ana3(&self) -> &Clock {
        self.clock(33)
    }
    ///0x188c - no description available
    #[inline(always)]
    pub const fn clockclk_top_ref0(&self) -> &Clock {
        self.clock(34)
    }
    ///0x1890 - no description available
    #[inline(always)]
    pub const fn clockclk_top_ref1(&self) -> &Clock {
        self.clock(35)
    }
    ///0x1c00..0x1c08 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ADCCLKclk_top_adc0` register.</div>
    #[inline(always)]
    pub const fn adcclk(&self, n: usize) -> &Adcclk {
        &self.adcclk[n]
    }
    ///Iterator for array of:
    ///0x1c00..0x1c08 - no description available
    #[inline(always)]
    pub fn adcclk_iter(&self) -> impl Iterator<Item = &Adcclk> {
        self.adcclk.iter()
    }
    ///0x1c00 - no description available
    #[inline(always)]
    pub const fn adcclkclk_top_adc0(&self) -> &Adcclk {
        self.adcclk(0)
    }
    ///0x1c04 - no description available
    #[inline(always)]
    pub const fn adcclkclk_top_adc1(&self) -> &Adcclk {
        self.adcclk(1)
    }
    ///0x1c08..0x1c10 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DACCLKclk_top_dac0` register.</div>
    #[inline(always)]
    pub const fn dacclk(&self, n: usize) -> &Dacclk {
        &self.dacclk[n]
    }
    ///Iterator for array of:
    ///0x1c08..0x1c10 - no description available
    #[inline(always)]
    pub fn dacclk_iter(&self) -> impl Iterator<Item = &Dacclk> {
        self.dacclk.iter()
    }
    ///0x1c08 - no description available
    #[inline(always)]
    pub const fn dacclkclk_top_dac0(&self) -> &Dacclk {
        self.dacclk(0)
    }
    ///0x1c0c - no description available
    #[inline(always)]
    pub const fn dacclkclk_top_dac1(&self) -> &Dacclk {
        self.dacclk(1)
    }
    ///0x2000 - Clock senario
    #[inline(always)]
    pub const fn global00(&self) -> &Global00 {
        &self.global00
    }
    ///0x2400..0x2440 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `MONITORslice0` cluster.</div>
    #[inline(always)]
    pub const fn monitor(&self, n: usize) -> &Monitor {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(9216)
                .add(32 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x2400..0x2440 - no description available
    #[inline(always)]
    pub fn monitor_iter(&self) -> impl Iterator<Item = &Monitor> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(9216)
                .add(32 * n)
                .cast()
        })
    }
    ///0x2400..0x2410 - no description available
    #[inline(always)]
    pub const fn monitorslice0(&self) -> &Monitor {
        self.monitor(0)
    }
    ///0x2420..0x2430 - no description available
    #[inline(always)]
    pub const fn monitorslice1(&self) -> &Monitor {
        self.monitor(1)
    }
    ///0x2440..0x2450 - no description available
    #[inline(always)]
    pub const fn monitorslice2(&self) -> &Monitor {
        self.monitor(2)
    }
    ///0x2460..0x2470 - no description available
    #[inline(always)]
    pub const fn monitorslice3(&self) -> &Monitor {
        self.monitor(3)
    }
    ///0x2800..0x2890 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CPUcpu0` cluster.</div>
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &Cpu {
        &self.cpu[n]
    }
    ///Iterator for array of:
    ///0x2800..0x2890 - no description available
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &Cpu> {
        self.cpu.iter()
    }
    ///0x2800..0x2890 - no description available
    #[inline(always)]
    pub const fn cpucpu0(&self) -> &Cpu {
        self.cpu(0)
    }
}
/**RESOURCE (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`resource::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resource::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@resource`] module*/
#[doc(alias = "RESOURCE")]
pub type Resource = crate::Reg<resource::ResourceSpec>;
///no description available
pub mod resource;
///no description available
pub use self::group0::Group0;
///Cluster
///no description available
pub mod group0;
///no description available
pub use self::affiliate::Affiliate;
///Cluster
///no description available
pub mod affiliate;
///no description available
pub use self::retention::Retention;
///Cluster
///no description available
pub mod retention;
///no description available
pub use self::power::Power;
///Cluster
///no description available
pub mod power;
///no description available
pub use self::reset::Reset;
///Cluster
///no description available
pub mod reset;
/**CLOCK_CPU (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`clock_cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_cpu`] module*/
#[doc(alias = "CLOCK_CPU")]
pub type ClockCpu = crate::Reg<clock_cpu::ClockCpuSpec>;
///no description available
pub mod clock_cpu;
/**CLOCK (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
///no description available
pub mod clock;
/**ADCCLK (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`adcclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcclk`] module*/
#[doc(alias = "ADCCLK")]
pub type Adcclk = crate::Reg<adcclk::AdcclkSpec>;
///no description available
pub mod adcclk;
/**DACCLK (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`dacclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dacclk`] module*/
#[doc(alias = "DACCLK")]
pub type Dacclk = crate::Reg<dacclk::DacclkSpec>;
///no description available
pub mod dacclk;
/**global00 (rw) register accessor: Clock senario

You can [`read`](crate::Reg::read) this register and get [`global00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`global00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@global00`] module*/
#[doc(alias = "global00")]
pub type Global00 = crate::Reg<global00::Global00Spec>;
///Clock senario
pub mod global00;
///no description available
pub use self::monitor::Monitor;
///Cluster
///no description available
pub mod monitor;
///no description available
pub use self::cpu::Cpu;
///Cluster
///no description available
pub mod cpu;
