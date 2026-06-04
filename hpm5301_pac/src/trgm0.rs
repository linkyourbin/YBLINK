#[repr(C)]
///Register block
pub struct RegisterBlock {
    filtcfg: [Filtcfg; 28],
    _reserved1: [u8; 0x90],
    trgocfg: [Trgocfg; 137],
    _reserved2: [u8; 0xdc],
    dmacfg: [Dmacfg; 8],
    _reserved3: [u8; 0xe0],
    gcr: Gcr,
    _reserved4: [u8; 0x0c],
    adc_matrix_sel: AdcMatrixSel,
    dac_matrix_sel: DacMatrixSel,
    pos_matrix_sel0: PosMatrixSel0,
    pos_matrix_sel1: PosMatrixSel1,
    _reserved8: [u8; 0xe0],
    trgm_in: [TrgmIn; 4],
    _reserved9: [u8; 0x10],
    trgm_out: [TrgmOut; 5],
}
impl RegisterBlock {
    ///0x00..0x70 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `FILTCFGPWM0_IN0` register.</div>
    #[inline(always)]
    pub const fn filtcfg(&self, n: usize) -> &Filtcfg {
        &self.filtcfg[n]
    }
    ///Iterator for array of:
    ///0x00..0x70 - no description available
    #[inline(always)]
    pub fn filtcfg_iter(&self) -> impl Iterator<Item = &Filtcfg> {
        self.filtcfg.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in0(&self) -> &Filtcfg {
        self.filtcfg(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in1(&self) -> &Filtcfg {
        self.filtcfg(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in2(&self) -> &Filtcfg {
        self.filtcfg(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in3(&self) -> &Filtcfg {
        self.filtcfg(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in4(&self) -> &Filtcfg {
        self.filtcfg(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in5(&self) -> &Filtcfg {
        self.filtcfg(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in6(&self) -> &Filtcfg {
        self.filtcfg(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_in7(&self) -> &Filtcfg {
        self.filtcfg(7)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in0(&self) -> &Filtcfg {
        self.filtcfg(8)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in1(&self) -> &Filtcfg {
        self.filtcfg(9)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in2(&self) -> &Filtcfg {
        self.filtcfg(10)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in3(&self) -> &Filtcfg {
        self.filtcfg(11)
    }
    ///0x30 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in4(&self) -> &Filtcfg {
        self.filtcfg(12)
    }
    ///0x34 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in5(&self) -> &Filtcfg {
        self.filtcfg(13)
    }
    ///0x38 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in6(&self) -> &Filtcfg {
        self.filtcfg(14)
    }
    ///0x3c - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_in7(&self) -> &Filtcfg {
        self.filtcfg(15)
    }
    ///0x40 - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_00(&self) -> &Filtcfg {
        self.filtcfg(16)
    }
    ///0x44 - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_01(&self) -> &Filtcfg {
        self.filtcfg(17)
    }
    ///0x48 - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_02(&self) -> &Filtcfg {
        self.filtcfg(18)
    }
    ///0x4c - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_03(&self) -> &Filtcfg {
        self.filtcfg(19)
    }
    ///0x50 - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_04(&self) -> &Filtcfg {
        self.filtcfg(20)
    }
    ///0x54 - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_05(&self) -> &Filtcfg {
        self.filtcfg(21)
    }
    ///0x58 - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_06(&self) -> &Filtcfg {
        self.filtcfg(22)
    }
    ///0x5c - no description available
    #[inline(always)]
    pub const fn filtcfgtrgm_p_07(&self) -> &Filtcfg {
        self.filtcfg(23)
    }
    ///0x60 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_fault0(&self) -> &Filtcfg {
        self.filtcfg(24)
    }
    ///0x64 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm0_fault1(&self) -> &Filtcfg {
        self.filtcfg(25)
    }
    ///0x68 - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_fault0(&self) -> &Filtcfg {
        self.filtcfg(26)
    }
    ///0x6c - no description available
    #[inline(always)]
    pub const fn filtcfgpwm1_fault1(&self) -> &Filtcfg {
        self.filtcfg(27)
    }
    ///0x100..0x324 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TRGOCFGMOT2OPAMP0_0` register.</div>
    #[inline(always)]
    pub const fn trgocfg(&self, n: usize) -> &Trgocfg {
        &self.trgocfg[n]
    }
    ///Iterator for array of:
    ///0x100..0x324 - no description available
    #[inline(always)]
    pub fn trgocfg_iter(&self) -> impl Iterator<Item = &Trgocfg> {
        self.trgocfg.iter()
    }
    ///0x100 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_0(&self) -> &Trgocfg {
        self.trgocfg(0)
    }
    ///0x104 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_1(&self) -> &Trgocfg {
        self.trgocfg(1)
    }
    ///0x108 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_2(&self) -> &Trgocfg {
        self.trgocfg(2)
    }
    ///0x10c - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_3(&self) -> &Trgocfg {
        self.trgocfg(3)
    }
    ///0x110 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_4(&self) -> &Trgocfg {
        self.trgocfg(4)
    }
    ///0x114 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_5(&self) -> &Trgocfg {
        self.trgocfg(5)
    }
    ///0x118 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_6(&self) -> &Trgocfg {
        self.trgocfg(6)
    }
    ///0x11c - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp0_7(&self) -> &Trgocfg {
        self.trgocfg(7)
    }
    ///0x120 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_0(&self) -> &Trgocfg {
        self.trgocfg(8)
    }
    ///0x124 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_1(&self) -> &Trgocfg {
        self.trgocfg(9)
    }
    ///0x128 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_2(&self) -> &Trgocfg {
        self.trgocfg(10)
    }
    ///0x12c - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_3(&self) -> &Trgocfg {
        self.trgocfg(11)
    }
    ///0x130 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_4(&self) -> &Trgocfg {
        self.trgocfg(12)
    }
    ///0x134 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_5(&self) -> &Trgocfg {
        self.trgocfg(13)
    }
    ///0x138 - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_6(&self) -> &Trgocfg {
        self.trgocfg(14)
    }
    ///0x13c - no description available
    #[inline(always)]
    pub const fn trgocfgmot2opamp1_7(&self) -> &Trgocfg {
        self.trgocfg(15)
    }
    ///0x140 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr0_in2(&self) -> &Trgocfg {
        self.trgocfg(16)
    }
    ///0x144 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr0_in3(&self) -> &Trgocfg {
        self.trgocfg(17)
    }
    ///0x148 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr0_synci(&self) -> &Trgocfg {
        self.trgocfg(18)
    }
    ///0x14c - no description available
    #[inline(always)]
    pub const fn trgocfggptmr1_in2(&self) -> &Trgocfg {
        self.trgocfg(19)
    }
    ///0x150 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr1_in3(&self) -> &Trgocfg {
        self.trgocfg(20)
    }
    ///0x154 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr1_synci(&self) -> &Trgocfg {
        self.trgocfg(21)
    }
    ///0x158 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr2_in2(&self) -> &Trgocfg {
        self.trgocfg(22)
    }
    ///0x15c - no description available
    #[inline(always)]
    pub const fn trgocfggptmr2_in3(&self) -> &Trgocfg {
        self.trgocfg(23)
    }
    ///0x160 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr2_synci(&self) -> &Trgocfg {
        self.trgocfg(24)
    }
    ///0x164 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr3_in2(&self) -> &Trgocfg {
        self.trgocfg(25)
    }
    ///0x168 - no description available
    #[inline(always)]
    pub const fn trgocfggptmr3_in3(&self) -> &Trgocfg {
        self.trgocfg(26)
    }
    ///0x16c - no description available
    #[inline(always)]
    pub const fn trgocfggptmr3_synci(&self) -> &Trgocfg {
        self.trgocfg(27)
    }
    ///0x170 - no description available
    #[inline(always)]
    pub const fn trgocfgacmp_ch0_win(&self) -> &Trgocfg {
        self.trgocfg(28)
    }
    ///0x174 - no description available
    #[inline(always)]
    pub const fn trgocfgacmp_ch1_win(&self) -> &Trgocfg {
        self.trgocfg(29)
    }
    ///0x178 - no description available
    #[inline(always)]
    pub const fn trgocfgdac0_buftrg(&self) -> &Trgocfg {
        self.trgocfg(30)
    }
    ///0x17c - no description available
    #[inline(always)]
    pub const fn trgocfgdac1_buftrg(&self) -> &Trgocfg {
        self.trgocfg(31)
    }
    ///0x180 - no description available
    #[inline(always)]
    pub const fn trgocfgadc0_strgi(&self) -> &Trgocfg {
        self.trgocfg(32)
    }
    ///0x184 - no description available
    #[inline(always)]
    pub const fn trgocfgadc1_strgi(&self) -> &Trgocfg {
        self.trgocfg(33)
    }
    ///0x188 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi0a(&self) -> &Trgocfg {
        self.trgocfg(34)
    }
    ///0x18c - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi0b(&self) -> &Trgocfg {
        self.trgocfg(35)
    }
    ///0x190 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi0c(&self) -> &Trgocfg {
        self.trgocfg(36)
    }
    ///0x194 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi1a(&self) -> &Trgocfg {
        self.trgocfg(37)
    }
    ///0x198 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi1b(&self) -> &Trgocfg {
        self.trgocfg(38)
    }
    ///0x19c - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi1c(&self) -> &Trgocfg {
        self.trgocfg(39)
    }
    ///0x1a0 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi2a(&self) -> &Trgocfg {
        self.trgocfg(40)
    }
    ///0x1a4 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi2b(&self) -> &Trgocfg {
        self.trgocfg(41)
    }
    ///0x1a8 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi2c(&self) -> &Trgocfg {
        self.trgocfg(42)
    }
    ///0x1ac - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi3a(&self) -> &Trgocfg {
        self.trgocfg(43)
    }
    ///0x1b0 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi3b(&self) -> &Trgocfg {
        self.trgocfg(44)
    }
    ///0x1b4 - no description available
    #[inline(always)]
    pub const fn trgocfgadcx_ptrgi3c(&self) -> &Trgocfg {
        self.trgocfg(45)
    }
    ///0x1b8 - no description available
    #[inline(always)]
    pub const fn trgocfgcan_ptpc0_cap(&self) -> &Trgocfg {
        self.trgocfg(46)
    }
    ///0x1bc - no description available
    #[inline(always)]
    pub const fn trgocfgcan_ptpc1_cap(&self) -> &Trgocfg {
        self.trgocfg(47)
    }
    ///0x1c0 - no description available
    #[inline(always)]
    pub const fn trgocfgqeo0_trig_in0(&self) -> &Trgocfg {
        self.trgocfg(48)
    }
    ///0x1c4 - no description available
    #[inline(always)]
    pub const fn trgocfgqeo0_trig_in1(&self) -> &Trgocfg {
        self.trgocfg(49)
    }
    ///0x1c8 - no description available
    #[inline(always)]
    pub const fn trgocfgqeo1_trig_in0(&self) -> &Trgocfg {
        self.trgocfg(50)
    }
    ///0x1cc - no description available
    #[inline(always)]
    pub const fn trgocfgqeo1_trig_in1(&self) -> &Trgocfg {
        self.trgocfg(51)
    }
    ///0x1d0 - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in0(&self) -> &Trgocfg {
        self.trgocfg(52)
    }
    ///0x1d4 - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in1(&self) -> &Trgocfg {
        self.trgocfg(53)
    }
    ///0x1d8 - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in2(&self) -> &Trgocfg {
        self.trgocfg(54)
    }
    ///0x1dc - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in3(&self) -> &Trgocfg {
        self.trgocfg(55)
    }
    ///0x1e0 - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in4(&self) -> &Trgocfg {
        self.trgocfg(56)
    }
    ///0x1e4 - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in5(&self) -> &Trgocfg {
        self.trgocfg(57)
    }
    ///0x1e8 - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in6(&self) -> &Trgocfg {
        self.trgocfg(58)
    }
    ///0x1ec - no description available
    #[inline(always)]
    pub const fn trgocfgsei_trig_in7(&self) -> &Trgocfg {
        self.trgocfg(59)
    }
    ///0x1f0 - no description available
    #[inline(always)]
    pub const fn trgocfgmmc0_trig_in0(&self) -> &Trgocfg {
        self.trgocfg(60)
    }
    ///0x1f4 - no description available
    #[inline(always)]
    pub const fn trgocfgmmc0_trig_in1(&self) -> &Trgocfg {
        self.trgocfg(61)
    }
    ///0x1f8 - no description available
    #[inline(always)]
    pub const fn trgocfgmmc1_trig_in0(&self) -> &Trgocfg {
        self.trgocfg(62)
    }
    ///0x1fc - no description available
    #[inline(always)]
    pub const fn trgocfgmmc1_trig_in1(&self) -> &Trgocfg {
        self.trgocfg(63)
    }
    ///0x200 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_00(&self) -> &Trgocfg {
        self.trgocfg(64)
    }
    ///0x204 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_01(&self) -> &Trgocfg {
        self.trgocfg(65)
    }
    ///0x208 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_02(&self) -> &Trgocfg {
        self.trgocfg(66)
    }
    ///0x20c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_03(&self) -> &Trgocfg {
        self.trgocfg(67)
    }
    ///0x210 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_04(&self) -> &Trgocfg {
        self.trgocfg(68)
    }
    ///0x214 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_05(&self) -> &Trgocfg {
        self.trgocfg(69)
    }
    ///0x218 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_06(&self) -> &Trgocfg {
        self.trgocfg(70)
    }
    ///0x21c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_07(&self) -> &Trgocfg {
        self.trgocfg(71)
    }
    ///0x220 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_08(&self) -> &Trgocfg {
        self.trgocfg(72)
    }
    ///0x224 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_09(&self) -> &Trgocfg {
        self.trgocfg(73)
    }
    ///0x228 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_10(&self) -> &Trgocfg {
        self.trgocfg(74)
    }
    ///0x22c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_11(&self) -> &Trgocfg {
        self.trgocfg(75)
    }
    ///0x230 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_12(&self) -> &Trgocfg {
        self.trgocfg(76)
    }
    ///0x234 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_13(&self) -> &Trgocfg {
        self.trgocfg(77)
    }
    ///0x238 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_14(&self) -> &Trgocfg {
        self.trgocfg(78)
    }
    ///0x23c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_15(&self) -> &Trgocfg {
        self.trgocfg(79)
    }
    ///0x240 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_16(&self) -> &Trgocfg {
        self.trgocfg(80)
    }
    ///0x244 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_17(&self) -> &Trgocfg {
        self.trgocfg(81)
    }
    ///0x248 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_18(&self) -> &Trgocfg {
        self.trgocfg(82)
    }
    ///0x24c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_19(&self) -> &Trgocfg {
        self.trgocfg(83)
    }
    ///0x250 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_20(&self) -> &Trgocfg {
        self.trgocfg(84)
    }
    ///0x254 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_21(&self) -> &Trgocfg {
        self.trgocfg(85)
    }
    ///0x258 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_22(&self) -> &Trgocfg {
        self.trgocfg(86)
    }
    ///0x25c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_23(&self) -> &Trgocfg {
        self.trgocfg(87)
    }
    ///0x260 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_24(&self) -> &Trgocfg {
        self.trgocfg(88)
    }
    ///0x264 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_25(&self) -> &Trgocfg {
        self.trgocfg(89)
    }
    ///0x268 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_26(&self) -> &Trgocfg {
        self.trgocfg(90)
    }
    ///0x26c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_27(&self) -> &Trgocfg {
        self.trgocfg(91)
    }
    ///0x270 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_28(&self) -> &Trgocfg {
        self.trgocfg(92)
    }
    ///0x274 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_29(&self) -> &Trgocfg {
        self.trgocfg(93)
    }
    ///0x278 - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_30(&self) -> &Trgocfg {
        self.trgocfg(94)
    }
    ///0x27c - no description available
    #[inline(always)]
    pub const fn trgocfgplb_in_31(&self) -> &Trgocfg {
        self.trgocfg(95)
    }
    ///0x280 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_00(&self) -> &Trgocfg {
        self.trgocfg(96)
    }
    ///0x284 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_01(&self) -> &Trgocfg {
        self.trgocfg(97)
    }
    ///0x288 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_02(&self) -> &Trgocfg {
        self.trgocfg(98)
    }
    ///0x28c - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_03(&self) -> &Trgocfg {
        self.trgocfg(99)
    }
    ///0x290 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_04(&self) -> &Trgocfg {
        self.trgocfg(100)
    }
    ///0x294 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_05(&self) -> &Trgocfg {
        self.trgocfg(101)
    }
    ///0x298 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_06(&self) -> &Trgocfg {
        self.trgocfg(102)
    }
    ///0x29c - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_p_07(&self) -> &Trgocfg {
        self.trgocfg(103)
    }
    ///0x2a0 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in8(&self) -> &Trgocfg {
        self.trgocfg(104)
    }
    ///0x2a4 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in9(&self) -> &Trgocfg {
        self.trgocfg(105)
    }
    ///0x2a8 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in10(&self) -> &Trgocfg {
        self.trgocfg(106)
    }
    ///0x2ac - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in11(&self) -> &Trgocfg {
        self.trgocfg(107)
    }
    ///0x2b0 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in12(&self) -> &Trgocfg {
        self.trgocfg(108)
    }
    ///0x2b4 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in13(&self) -> &Trgocfg {
        self.trgocfg(109)
    }
    ///0x2b8 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in14(&self) -> &Trgocfg {
        self.trgocfg(110)
    }
    ///0x2bc - no description available
    #[inline(always)]
    pub const fn trgocfgpwm_in15(&self) -> &Trgocfg {
        self.trgocfg(111)
    }
    ///0x2c0 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm0_frci(&self) -> &Trgocfg {
        self.trgocfg(112)
    }
    ///0x2c4 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm0_frcsynci(&self) -> &Trgocfg {
        self.trgocfg(113)
    }
    ///0x2c8 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm0_synci(&self) -> &Trgocfg {
        self.trgocfg(114)
    }
    ///0x2cc - no description available
    #[inline(always)]
    pub const fn trgocfgpwm0_shrldsynci(&self) -> &Trgocfg {
        self.trgocfg(115)
    }
    ///0x2d0 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm0_faulti0(&self) -> &Trgocfg {
        self.trgocfg(116)
    }
    ///0x2d4 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm0_faulti1(&self) -> &Trgocfg {
        self.trgocfg(117)
    }
    ///0x2d8 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm1_frci(&self) -> &Trgocfg {
        self.trgocfg(118)
    }
    ///0x2dc - no description available
    #[inline(always)]
    pub const fn trgocfgpwm1_frcsynci(&self) -> &Trgocfg {
        self.trgocfg(119)
    }
    ///0x2e0 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm1_synci(&self) -> &Trgocfg {
        self.trgocfg(120)
    }
    ///0x2e4 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm1_shrldsynci(&self) -> &Trgocfg {
        self.trgocfg(121)
    }
    ///0x2e8 - no description available
    #[inline(always)]
    pub const fn trgocfgpwm1_faulti0(&self) -> &Trgocfg {
        self.trgocfg(122)
    }
    ///0x2ec - no description available
    #[inline(always)]
    pub const fn trgocfgpwm1_faulti1(&self) -> &Trgocfg {
        self.trgocfg(123)
    }
    ///0x2f0 - no description available
    #[inline(always)]
    pub const fn trgocfgrdc_trig_in0(&self) -> &Trgocfg {
        self.trgocfg(124)
    }
    ///0x2f4 - no description available
    #[inline(always)]
    pub const fn trgocfgrdc_trig_in1(&self) -> &Trgocfg {
        self.trgocfg(125)
    }
    ///0x2f8 - no description available
    #[inline(always)]
    pub const fn trgocfgsynctimer_trig(&self) -> &Trgocfg {
        self.trgocfg(126)
    }
    ///0x2fc - no description available
    #[inline(always)]
    pub const fn trgocfgqei0_trig_in(&self) -> &Trgocfg {
        self.trgocfg(127)
    }
    ///0x300 - no description available
    #[inline(always)]
    pub const fn trgocfgqei1_trig_in(&self) -> &Trgocfg {
        self.trgocfg(128)
    }
    ///0x304 - no description available
    #[inline(always)]
    pub const fn trgocfgqei0_pause(&self) -> &Trgocfg {
        self.trgocfg(129)
    }
    ///0x308 - no description available
    #[inline(always)]
    pub const fn trgocfgqei1_pause(&self) -> &Trgocfg {
        self.trgocfg(130)
    }
    ///0x30c - no description available
    #[inline(always)]
    pub const fn trgocfguart_trig0(&self) -> &Trgocfg {
        self.trgocfg(131)
    }
    ///0x310 - no description available
    #[inline(always)]
    pub const fn trgocfguart_trig1(&self) -> &Trgocfg {
        self.trgocfg(132)
    }
    ///0x314 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_irq0(&self) -> &Trgocfg {
        self.trgocfg(133)
    }
    ///0x318 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_irq1(&self) -> &Trgocfg {
        self.trgocfg(134)
    }
    ///0x31c - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_dma0(&self) -> &Trgocfg {
        self.trgocfg(135)
    }
    ///0x320 - no description available
    #[inline(always)]
    pub const fn trgocfgtrgm_dma1(&self) -> &Trgocfg {
        self.trgocfg(136)
    }
    ///0x400..0x420 - no description available
    #[inline(always)]
    pub const fn dmacfg(&self, n: usize) -> &Dmacfg {
        &self.dmacfg[n]
    }
    ///Iterator for array of:
    ///0x400..0x420 - no description available
    #[inline(always)]
    pub fn dmacfg_iter(&self) -> impl Iterator<Item = &Dmacfg> {
        self.dmacfg.iter()
    }
    ///0x500 - General Control Register
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    ///0x510 - adc matrix select register
    #[inline(always)]
    pub const fn adc_matrix_sel(&self) -> &AdcMatrixSel {
        &self.adc_matrix_sel
    }
    ///0x514 - dac matrix select register
    #[inline(always)]
    pub const fn dac_matrix_sel(&self) -> &DacMatrixSel {
        &self.dac_matrix_sel
    }
    ///0x518 - position matrix select register0
    #[inline(always)]
    pub const fn pos_matrix_sel0(&self) -> &PosMatrixSel0 {
        &self.pos_matrix_sel0
    }
    ///0x51c - position matrix select register1
    #[inline(always)]
    pub const fn pos_matrix_sel1(&self) -> &PosMatrixSel1 {
        &self.pos_matrix_sel1
    }
    ///0x600..0x610 - no description available
    #[inline(always)]
    pub const fn trgm_in(&self, n: usize) -> &TrgmIn {
        &self.trgm_in[n]
    }
    ///Iterator for array of:
    ///0x600..0x610 - no description available
    #[inline(always)]
    pub fn trgm_in_iter(&self) -> impl Iterator<Item = &TrgmIn> {
        self.trgm_in.iter()
    }
    ///0x620..0x634 - no description available
    #[inline(always)]
    pub const fn trgm_out(&self, n: usize) -> &TrgmOut {
        &self.trgm_out[n]
    }
    ///Iterator for array of:
    ///0x620..0x634 - no description available
    #[inline(always)]
    pub fn trgm_out_iter(&self) -> impl Iterator<Item = &TrgmOut> {
        self.trgm_out.iter()
    }
}
/**FILTCFG (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`filtcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filtcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@filtcfg`] module*/
#[doc(alias = "FILTCFG")]
pub type Filtcfg = crate::Reg<filtcfg::FiltcfgSpec>;
///no description available
pub mod filtcfg;
/**TRGOCFG (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`trgocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trgocfg`] module*/
#[doc(alias = "TRGOCFG")]
pub type Trgocfg = crate::Reg<trgocfg::TrgocfgSpec>;
///no description available
pub mod trgocfg;
/**DMACFG (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacfg`] module*/
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
///no description available
pub mod dmacfg;
/**GCR (rw) register accessor: General Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gcr`] module*/
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
///General Control Register
pub mod gcr;
/**ADC_MATRIX_SEL (rw) register accessor: adc matrix select register

You can [`read`](crate::Reg::read) this register and get [`adc_matrix_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_matrix_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc_matrix_sel`] module*/
#[doc(alias = "ADC_MATRIX_SEL")]
pub type AdcMatrixSel = crate::Reg<adc_matrix_sel::AdcMatrixSelSpec>;
///adc matrix select register
pub mod adc_matrix_sel;
/**DAC_MATRIX_SEL (rw) register accessor: dac matrix select register

You can [`read`](crate::Reg::read) this register and get [`dac_matrix_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_matrix_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dac_matrix_sel`] module*/
#[doc(alias = "DAC_MATRIX_SEL")]
pub type DacMatrixSel = crate::Reg<dac_matrix_sel::DacMatrixSelSpec>;
///dac matrix select register
pub mod dac_matrix_sel;
/**POS_MATRIX_SEL0 (rw) register accessor: position matrix select register0

You can [`read`](crate::Reg::read) this register and get [`pos_matrix_sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pos_matrix_sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pos_matrix_sel0`] module*/
#[doc(alias = "POS_MATRIX_SEL0")]
pub type PosMatrixSel0 = crate::Reg<pos_matrix_sel0::PosMatrixSel0Spec>;
///position matrix select register0
pub mod pos_matrix_sel0;
/**POS_MATRIX_SEL1 (rw) register accessor: position matrix select register1

You can [`read`](crate::Reg::read) this register and get [`pos_matrix_sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pos_matrix_sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pos_matrix_sel1`] module*/
#[doc(alias = "POS_MATRIX_SEL1")]
pub type PosMatrixSel1 = crate::Reg<pos_matrix_sel1::PosMatrixSel1Spec>;
///position matrix select register1
pub mod pos_matrix_sel1;
/**TRGM_IN (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`trgm_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgm_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trgm_in`] module*/
#[doc(alias = "TRGM_IN")]
pub type TrgmIn = crate::Reg<trgm_in::TrgmInSpec>;
///no description available
pub mod trgm_in;
/**TRGM_OUT (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`trgm_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgm_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trgm_out`] module*/
#[doc(alias = "TRGM_OUT")]
pub type TrgmOut = crate::Reg<trgm_out::TrgmOutSpec>;
///no description available
pub mod trgm_out;
