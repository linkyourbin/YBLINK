#[repr(C)]
///Register block
pub struct RegisterBlock {
    config: [Config; 12],
    trg_dma_addr: TrgDmaAddr,
    trg_sw_sta: TrgSwSta,
    _reserved3: [u8; 0x04c8],
    buf_cfg0: BufCfg0,
    _reserved4: [u8; 0x02fc],
    seq_cfg0: SeqCfg0,
    seq_dma_addr: SeqDmaAddr,
    seq_wr_addr: SeqWrAddr,
    seq_dma_cfg: SeqDmaCfg,
    seq_que: [SeqQue; 16],
    seq_high_cfg: SeqHighCfg,
    _reserved10: [u8; 0x03ac],
    prd_cfg_ch: (),
    _reserved11: [u8; 0x0400],
    sample_cfg: [SampleCfg; 16],
    _reserved12: [u8; 0xc4],
    conv_cfg1: ConvCfg1,
    adc_cfg0: AdcCfg0,
    _reserved14: [u8; 0x04],
    int_sts: IntSts,
    int_en: IntEn,
    _reserved16: [u8; 0xe8],
    ana_ctrl0: AnaCtrl0,
    _reserved17: [u8; 0x0c],
    ana_status: AnaStatus,
    _reserved18: [u8; 0x01ec],
    adc16_params: [Adc16Params; 34],
    adc16_config0: Adc16Config0,
    _reserved20: [u8; 0x18],
    adc16_config1: Adc16Config1,
}
impl RegisterBlock {
    ///0x00..0x30 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CONFIGtrg0a` register.</div>
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    ///Iterator for array of:
    ///0x00..0x30 - no description available
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
    ///0x00 - no description available
    #[inline(always)]
    pub const fn configtrg0a(&self) -> &Config {
        self.config(0)
    }
    ///0x04 - no description available
    #[inline(always)]
    pub const fn configtrg0b(&self) -> &Config {
        self.config(1)
    }
    ///0x08 - no description available
    #[inline(always)]
    pub const fn configtrg0c(&self) -> &Config {
        self.config(2)
    }
    ///0x0c - no description available
    #[inline(always)]
    pub const fn configtrg1a(&self) -> &Config {
        self.config(3)
    }
    ///0x10 - no description available
    #[inline(always)]
    pub const fn configtrg1b(&self) -> &Config {
        self.config(4)
    }
    ///0x14 - no description available
    #[inline(always)]
    pub const fn configtrg1c(&self) -> &Config {
        self.config(5)
    }
    ///0x18 - no description available
    #[inline(always)]
    pub const fn configtrg2a(&self) -> &Config {
        self.config(6)
    }
    ///0x1c - no description available
    #[inline(always)]
    pub const fn configtrg2b(&self) -> &Config {
        self.config(7)
    }
    ///0x20 - no description available
    #[inline(always)]
    pub const fn configtrg2c(&self) -> &Config {
        self.config(8)
    }
    ///0x24 - no description available
    #[inline(always)]
    pub const fn configtrg3a(&self) -> &Config {
        self.config(9)
    }
    ///0x28 - no description available
    #[inline(always)]
    pub const fn configtrg3b(&self) -> &Config {
        self.config(10)
    }
    ///0x2c - no description available
    #[inline(always)]
    pub const fn configtrg3c(&self) -> &Config {
        self.config(11)
    }
    ///0x30 - No description available
    #[inline(always)]
    pub const fn trg_dma_addr(&self) -> &TrgDmaAddr {
        &self.trg_dma_addr
    }
    ///0x34 - No description available
    #[inline(always)]
    pub const fn trg_sw_sta(&self) -> &TrgSwSta {
        &self.trg_sw_sta
    }
    ///0x500 - No description available
    #[inline(always)]
    pub const fn buf_cfg0(&self) -> &BufCfg0 {
        &self.buf_cfg0
    }
    ///0x800 - No description available
    #[inline(always)]
    pub const fn seq_cfg0(&self) -> &SeqCfg0 {
        &self.seq_cfg0
    }
    ///0x804 - No description available
    #[inline(always)]
    pub const fn seq_dma_addr(&self) -> &SeqDmaAddr {
        &self.seq_dma_addr
    }
    ///0x808 - No description available
    #[inline(always)]
    pub const fn seq_wr_addr(&self) -> &SeqWrAddr {
        &self.seq_wr_addr
    }
    ///0x80c - No description available
    #[inline(always)]
    pub const fn seq_dma_cfg(&self) -> &SeqDmaCfg {
        &self.seq_dma_cfg
    }
    ///0x810..0x850 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SEQ_QUEcfg0` register.</div>
    #[inline(always)]
    pub const fn seq_que(&self, n: usize) -> &SeqQue {
        &self.seq_que[n]
    }
    ///Iterator for array of:
    ///0x810..0x850 - no description available
    #[inline(always)]
    pub fn seq_que_iter(&self) -> impl Iterator<Item = &SeqQue> {
        self.seq_que.iter()
    }
    ///0x810 - no description available
    #[inline(always)]
    pub const fn seq_quecfg0(&self) -> &SeqQue {
        self.seq_que(0)
    }
    ///0x814 - no description available
    #[inline(always)]
    pub const fn seq_quecfg1(&self) -> &SeqQue {
        self.seq_que(1)
    }
    ///0x818 - no description available
    #[inline(always)]
    pub const fn seq_quecfg2(&self) -> &SeqQue {
        self.seq_que(2)
    }
    ///0x81c - no description available
    #[inline(always)]
    pub const fn seq_quecfg3(&self) -> &SeqQue {
        self.seq_que(3)
    }
    ///0x820 - no description available
    #[inline(always)]
    pub const fn seq_quecfg4(&self) -> &SeqQue {
        self.seq_que(4)
    }
    ///0x824 - no description available
    #[inline(always)]
    pub const fn seq_quecfg5(&self) -> &SeqQue {
        self.seq_que(5)
    }
    ///0x828 - no description available
    #[inline(always)]
    pub const fn seq_quecfg6(&self) -> &SeqQue {
        self.seq_que(6)
    }
    ///0x82c - no description available
    #[inline(always)]
    pub const fn seq_quecfg7(&self) -> &SeqQue {
        self.seq_que(7)
    }
    ///0x830 - no description available
    #[inline(always)]
    pub const fn seq_quecfg8(&self) -> &SeqQue {
        self.seq_que(8)
    }
    ///0x834 - no description available
    #[inline(always)]
    pub const fn seq_quecfg9(&self) -> &SeqQue {
        self.seq_que(9)
    }
    ///0x838 - no description available
    #[inline(always)]
    pub const fn seq_quecfg10(&self) -> &SeqQue {
        self.seq_que(10)
    }
    ///0x83c - no description available
    #[inline(always)]
    pub const fn seq_quecfg11(&self) -> &SeqQue {
        self.seq_que(11)
    }
    ///0x840 - no description available
    #[inline(always)]
    pub const fn seq_quecfg12(&self) -> &SeqQue {
        self.seq_que(12)
    }
    ///0x844 - no description available
    #[inline(always)]
    pub const fn seq_quecfg13(&self) -> &SeqQue {
        self.seq_que(13)
    }
    ///0x848 - no description available
    #[inline(always)]
    pub const fn seq_quecfg14(&self) -> &SeqQue {
        self.seq_que(14)
    }
    ///0x84c - no description available
    #[inline(always)]
    pub const fn seq_quecfg15(&self) -> &SeqQue {
        self.seq_que(15)
    }
    ///0x850 - No description available
    #[inline(always)]
    pub const fn seq_high_cfg(&self) -> &SeqHighCfg {
        &self.seq_high_cfg
    }
    ///0xc00..0xcc0 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `PRD_CFG_CHchn0` cluster.</div>
    #[inline(always)]
    pub const fn prd_cfg_ch(&self, n: usize) -> &PrdCfgCh {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3072)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0xc00..0xcc0 - no description available
    #[inline(always)]
    pub fn prd_cfg_ch_iter(&self) -> impl Iterator<Item = &PrdCfgCh> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3072)
                .add(16 * n)
                .cast()
        })
    }
    ///0xc00..0xc0c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn0(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(0)
    }
    ///0xc10..0xc1c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn1(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(1)
    }
    ///0xc20..0xc2c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn2(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(2)
    }
    ///0xc30..0xc3c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn3(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(3)
    }
    ///0xc40..0xc4c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn4(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(4)
    }
    ///0xc50..0xc5c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn5(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(5)
    }
    ///0xc60..0xc6c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn6(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(6)
    }
    ///0xc70..0xc7c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn7(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(7)
    }
    ///0xc80..0xc8c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn8(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(8)
    }
    ///0xc90..0xc9c - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn9(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(9)
    }
    ///0xca0..0xcac - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn10(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(10)
    }
    ///0xcb0..0xcbc - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn11(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(11)
    }
    ///0xcc0..0xccc - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn12(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(12)
    }
    ///0xcd0..0xcdc - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn13(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(13)
    }
    ///0xce0..0xcec - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn14(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(14)
    }
    ///0xcf0..0xcfc - no description available
    #[inline(always)]
    pub const fn prd_cfg_chchn15(&self) -> &PrdCfgCh {
        self.prd_cfg_ch(15)
    }
    ///0x1000..0x1040 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SAMPLE_CFGchn0` register.</div>
    #[inline(always)]
    pub const fn sample_cfg(&self, n: usize) -> &SampleCfg {
        &self.sample_cfg[n]
    }
    ///Iterator for array of:
    ///0x1000..0x1040 - no description available
    #[inline(always)]
    pub fn sample_cfg_iter(&self) -> impl Iterator<Item = &SampleCfg> {
        self.sample_cfg.iter()
    }
    ///0x1000 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn0(&self) -> &SampleCfg {
        self.sample_cfg(0)
    }
    ///0x1004 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn1(&self) -> &SampleCfg {
        self.sample_cfg(1)
    }
    ///0x1008 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn2(&self) -> &SampleCfg {
        self.sample_cfg(2)
    }
    ///0x100c - no description available
    #[inline(always)]
    pub const fn sample_cfgchn3(&self) -> &SampleCfg {
        self.sample_cfg(3)
    }
    ///0x1010 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn4(&self) -> &SampleCfg {
        self.sample_cfg(4)
    }
    ///0x1014 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn5(&self) -> &SampleCfg {
        self.sample_cfg(5)
    }
    ///0x1018 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn6(&self) -> &SampleCfg {
        self.sample_cfg(6)
    }
    ///0x101c - no description available
    #[inline(always)]
    pub const fn sample_cfgchn7(&self) -> &SampleCfg {
        self.sample_cfg(7)
    }
    ///0x1020 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn8(&self) -> &SampleCfg {
        self.sample_cfg(8)
    }
    ///0x1024 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn9(&self) -> &SampleCfg {
        self.sample_cfg(9)
    }
    ///0x1028 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn10(&self) -> &SampleCfg {
        self.sample_cfg(10)
    }
    ///0x102c - no description available
    #[inline(always)]
    pub const fn sample_cfgchn11(&self) -> &SampleCfg {
        self.sample_cfg(11)
    }
    ///0x1030 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn12(&self) -> &SampleCfg {
        self.sample_cfg(12)
    }
    ///0x1034 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn13(&self) -> &SampleCfg {
        self.sample_cfg(13)
    }
    ///0x1038 - no description available
    #[inline(always)]
    pub const fn sample_cfgchn14(&self) -> &SampleCfg {
        self.sample_cfg(14)
    }
    ///0x103c - no description available
    #[inline(always)]
    pub const fn sample_cfgchn15(&self) -> &SampleCfg {
        self.sample_cfg(15)
    }
    ///0x1104 - No description available
    #[inline(always)]
    pub const fn conv_cfg1(&self) -> &ConvCfg1 {
        &self.conv_cfg1
    }
    ///0x1108 - No description available
    #[inline(always)]
    pub const fn adc_cfg0(&self) -> &AdcCfg0 {
        &self.adc_cfg0
    }
    ///0x1110 - No description available
    #[inline(always)]
    pub const fn int_sts(&self) -> &IntSts {
        &self.int_sts
    }
    ///0x1114 - No description available
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    ///0x1200 - No description available
    #[inline(always)]
    pub const fn ana_ctrl0(&self) -> &AnaCtrl0 {
        &self.ana_ctrl0
    }
    ///0x1210 - No description available
    #[inline(always)]
    pub const fn ana_status(&self) -> &AnaStatus {
        &self.ana_status
    }
    ///0x1400..0x1444 - no description available
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ADC16_PARAMSadc16_para00` register.</div>
    #[inline(always)]
    pub const fn adc16_params(&self, n: usize) -> &Adc16Params {
        &self.adc16_params[n]
    }
    ///Iterator for array of:
    ///0x1400..0x1444 - no description available
    #[inline(always)]
    pub fn adc16_params_iter(&self) -> impl Iterator<Item = &Adc16Params> {
        self.adc16_params.iter()
    }
    ///0x1400 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para00(&self) -> &Adc16Params {
        self.adc16_params(0)
    }
    ///0x1402 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para01(&self) -> &Adc16Params {
        self.adc16_params(1)
    }
    ///0x1404 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para02(&self) -> &Adc16Params {
        self.adc16_params(2)
    }
    ///0x1406 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para03(&self) -> &Adc16Params {
        self.adc16_params(3)
    }
    ///0x1408 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para04(&self) -> &Adc16Params {
        self.adc16_params(4)
    }
    ///0x140a - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para05(&self) -> &Adc16Params {
        self.adc16_params(5)
    }
    ///0x140c - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para06(&self) -> &Adc16Params {
        self.adc16_params(6)
    }
    ///0x140e - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para07(&self) -> &Adc16Params {
        self.adc16_params(7)
    }
    ///0x1410 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para08(&self) -> &Adc16Params {
        self.adc16_params(8)
    }
    ///0x1412 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para09(&self) -> &Adc16Params {
        self.adc16_params(9)
    }
    ///0x1414 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para10(&self) -> &Adc16Params {
        self.adc16_params(10)
    }
    ///0x1416 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para11(&self) -> &Adc16Params {
        self.adc16_params(11)
    }
    ///0x1418 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para12(&self) -> &Adc16Params {
        self.adc16_params(12)
    }
    ///0x141a - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para13(&self) -> &Adc16Params {
        self.adc16_params(13)
    }
    ///0x141c - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para14(&self) -> &Adc16Params {
        self.adc16_params(14)
    }
    ///0x141e - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para15(&self) -> &Adc16Params {
        self.adc16_params(15)
    }
    ///0x1420 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para16(&self) -> &Adc16Params {
        self.adc16_params(16)
    }
    ///0x1422 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para17(&self) -> &Adc16Params {
        self.adc16_params(17)
    }
    ///0x1424 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para18(&self) -> &Adc16Params {
        self.adc16_params(18)
    }
    ///0x1426 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para19(&self) -> &Adc16Params {
        self.adc16_params(19)
    }
    ///0x1428 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para20(&self) -> &Adc16Params {
        self.adc16_params(20)
    }
    ///0x142a - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para21(&self) -> &Adc16Params {
        self.adc16_params(21)
    }
    ///0x142c - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para22(&self) -> &Adc16Params {
        self.adc16_params(22)
    }
    ///0x142e - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para23(&self) -> &Adc16Params {
        self.adc16_params(23)
    }
    ///0x1430 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para24(&self) -> &Adc16Params {
        self.adc16_params(24)
    }
    ///0x1432 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para25(&self) -> &Adc16Params {
        self.adc16_params(25)
    }
    ///0x1434 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para26(&self) -> &Adc16Params {
        self.adc16_params(26)
    }
    ///0x1436 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para27(&self) -> &Adc16Params {
        self.adc16_params(27)
    }
    ///0x1438 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para28(&self) -> &Adc16Params {
        self.adc16_params(28)
    }
    ///0x143a - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para29(&self) -> &Adc16Params {
        self.adc16_params(29)
    }
    ///0x143c - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para30(&self) -> &Adc16Params {
        self.adc16_params(30)
    }
    ///0x143e - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para31(&self) -> &Adc16Params {
        self.adc16_params(31)
    }
    ///0x1440 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para32(&self) -> &Adc16Params {
        self.adc16_params(32)
    }
    ///0x1442 - no description available
    #[inline(always)]
    pub const fn adc16_paramsadc16_para33(&self) -> &Adc16Params {
        self.adc16_params(33)
    }
    ///0x1444 - No description available
    #[inline(always)]
    pub const fn adc16_config0(&self) -> &Adc16Config0 {
        &self.adc16_config0
    }
    ///0x1460 - No description available
    #[inline(always)]
    pub const fn adc16_config1(&self) -> &Adc16Config1 {
        &self.adc16_config1
    }
}
/**CONFIG (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
///no description available
pub mod config;
/**trg_dma_addr (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`trg_dma_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg_dma_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trg_dma_addr`] module*/
#[doc(alias = "trg_dma_addr")]
pub type TrgDmaAddr = crate::Reg<trg_dma_addr::TrgDmaAddrSpec>;
///No description available
pub mod trg_dma_addr;
/**trg_sw_sta (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`trg_sw_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg_sw_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trg_sw_sta`] module*/
#[doc(alias = "trg_sw_sta")]
pub type TrgSwSta = crate::Reg<trg_sw_sta::TrgSwStaSpec>;
///No description available
pub mod trg_sw_sta;
/**buf_cfg0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`buf_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buf_cfg0`] module*/
#[doc(alias = "buf_cfg0")]
pub type BufCfg0 = crate::Reg<buf_cfg0::BufCfg0Spec>;
///No description available
pub mod buf_cfg0;
/**seq_cfg0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`seq_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seq_cfg0`] module*/
#[doc(alias = "seq_cfg0")]
pub type SeqCfg0 = crate::Reg<seq_cfg0::SeqCfg0Spec>;
///No description available
pub mod seq_cfg0;
/**seq_dma_addr (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`seq_dma_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_dma_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seq_dma_addr`] module*/
#[doc(alias = "seq_dma_addr")]
pub type SeqDmaAddr = crate::Reg<seq_dma_addr::SeqDmaAddrSpec>;
///No description available
pub mod seq_dma_addr;
/**seq_wr_addr (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`seq_wr_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_wr_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seq_wr_addr`] module*/
#[doc(alias = "seq_wr_addr")]
pub type SeqWrAddr = crate::Reg<seq_wr_addr::SeqWrAddrSpec>;
///No description available
pub mod seq_wr_addr;
/**seq_dma_cfg (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`seq_dma_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_dma_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seq_dma_cfg`] module*/
#[doc(alias = "seq_dma_cfg")]
pub type SeqDmaCfg = crate::Reg<seq_dma_cfg::SeqDmaCfgSpec>;
///No description available
pub mod seq_dma_cfg;
/**SEQ_QUE (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`seq_que::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_que::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seq_que`] module*/
#[doc(alias = "SEQ_QUE")]
pub type SeqQue = crate::Reg<seq_que::SeqQueSpec>;
///no description available
pub mod seq_que;
/**seq_high_cfg (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`seq_high_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_high_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seq_high_cfg`] module*/
#[doc(alias = "seq_high_cfg")]
pub type SeqHighCfg = crate::Reg<seq_high_cfg::SeqHighCfgSpec>;
///No description available
pub mod seq_high_cfg;
///no description available
pub use self::prd_cfg_ch::PrdCfgCh;
///Cluster
///no description available
pub mod prd_cfg_ch;
/**SAMPLE_CFG (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`sample_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sample_cfg`] module*/
#[doc(alias = "SAMPLE_CFG")]
pub type SampleCfg = crate::Reg<sample_cfg::SampleCfgSpec>;
///no description available
pub mod sample_cfg;
/**conv_cfg1 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`conv_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conv_cfg1`] module*/
#[doc(alias = "conv_cfg1")]
pub type ConvCfg1 = crate::Reg<conv_cfg1::ConvCfg1Spec>;
///No description available
pub mod conv_cfg1;
/**adc_cfg0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`adc_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc_cfg0`] module*/
#[doc(alias = "adc_cfg0")]
pub type AdcCfg0 = crate::Reg<adc_cfg0::AdcCfg0Spec>;
///No description available
pub mod adc_cfg0;
/**int_sts (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_sts`] module*/
#[doc(alias = "int_sts")]
pub type IntSts = crate::Reg<int_sts::IntStsSpec>;
///No description available
pub mod int_sts;
/**int_en (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_en`] module*/
#[doc(alias = "int_en")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
///No description available
pub mod int_en;
/**ana_ctrl0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`ana_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_ctrl0`] module*/
#[doc(alias = "ana_ctrl0")]
pub type AnaCtrl0 = crate::Reg<ana_ctrl0::AnaCtrl0Spec>;
///No description available
pub mod ana_ctrl0;
/**ana_status (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`ana_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_status`] module*/
#[doc(alias = "ana_status")]
pub type AnaStatus = crate::Reg<ana_status::AnaStatusSpec>;
///No description available
pub mod ana_status;
/**ADC16_PARAMS (rw) register accessor: no description available

You can [`read`](crate::Reg::read) this register and get [`adc16_params::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc16_params::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc16_params`] module*/
#[doc(alias = "ADC16_PARAMS")]
pub type Adc16Params = crate::Reg<adc16_params::Adc16ParamsSpec>;
///no description available
pub mod adc16_params;
/**adc16_config0 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`adc16_config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc16_config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc16_config0`] module*/
#[doc(alias = "adc16_config0")]
pub type Adc16Config0 = crate::Reg<adc16_config0::Adc16Config0Spec>;
///No description available
pub mod adc16_config0;
/**adc16_config1 (rw) register accessor: No description available

You can [`read`](crate::Reg::read) this register and get [`adc16_config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc16_config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc16_config1`] module*/
#[doc(alias = "adc16_config1")]
pub type Adc16Config1 = crate::Reg<adc16_config1::Adc16Config1Spec>;
///No description available
pub mod adc16_config1;
