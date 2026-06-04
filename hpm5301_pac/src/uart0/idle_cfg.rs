///Register `IDLE_CFG` reader
pub type R = crate::R<IdleCfgSpec>;
///Register `IDLE_CFG` writer
pub type W = crate::W<IdleCfgSpec>;
///Field `RX_IDLE_THR` reader - Threshold for UART Receive Idle detection (in terms of bits)
pub type RxIdleThrR = crate::FieldReader;
///Field `RX_IDLE_THR` writer - Threshold for UART Receive Idle detection (in terms of bits)
pub type RxIdleThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RX_IDLE_EN` reader - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature
pub type RxIdleEnR = crate::BitReader;
///Field `RX_IDLE_EN` writer - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature
pub type RxIdleEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IDLE_COND` reader - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle
pub type RxIdleCondR = crate::BitReader;
///Field `RX_IDLE_COND` writer - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle
pub type RxIdleCondW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXEN` reader - UART receive enable. 0 - hold RX input to high, avoide wrong data input when config pinmux 1 - bypass RX input from PIN software should set it after config pinmux
pub type RxenR = crate::BitReader;
///Field `RXEN` writer - UART receive enable. 0 - hold RX input to high, avoide wrong data input when config pinmux 1 - bypass RX input from PIN software should set it after config pinmux
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_IDLE_THR` reader - Threshold for UART transmit Idle detection (in terms of bits)
pub type TxIdleThrR = crate::FieldReader;
///Field `TX_IDLE_THR` writer - Threshold for UART transmit Idle detection (in terms of bits)
pub type TxIdleThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TX_IDLE_EN` reader - UART TX Idle Detect Enable 0 - Disable 1 - Enable
pub type TxIdleEnR = crate::BitReader;
///Field `TX_IDLE_EN` writer - UART TX Idle Detect Enable 0 - Disable 1 - Enable
pub type TxIdleEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_IDLE_COND` reader - IDLE Detection Condition 0 - Treat as idle if TX pin is logic one 1 - Treat as idle if UART state machine state is idle
pub type TxIdleCondR = crate::BitReader;
///Field `TX_IDLE_COND` writer - IDLE Detection Condition 0 - Treat as idle if TX pin is logic one 1 - Treat as idle if UART state machine state is idle
pub type TxIdleCondW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Threshold for UART Receive Idle detection (in terms of bits)
    #[inline(always)]
    pub fn rx_idle_thr(&self) -> RxIdleThrR {
        RxIdleThrR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature
    #[inline(always)]
    pub fn rx_idle_en(&self) -> RxIdleEnR {
        RxIdleEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle
    #[inline(always)]
    pub fn rx_idle_cond(&self) -> RxIdleCondR {
        RxIdleCondR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - UART receive enable. 0 - hold RX input to high, avoide wrong data input when config pinmux 1 - bypass RX input from PIN software should set it after config pinmux
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:23 - Threshold for UART transmit Idle detection (in terms of bits)
    #[inline(always)]
    pub fn tx_idle_thr(&self) -> TxIdleThrR {
        TxIdleThrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - UART TX Idle Detect Enable 0 - Disable 1 - Enable
    #[inline(always)]
    pub fn tx_idle_en(&self) -> TxIdleEnR {
        TxIdleEnR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - IDLE Detection Condition 0 - Treat as idle if TX pin is logic one 1 - Treat as idle if UART state machine state is idle
    #[inline(always)]
    pub fn tx_idle_cond(&self) -> TxIdleCondR {
        TxIdleCondR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Threshold for UART Receive Idle detection (in terms of bits)
    #[inline(always)]
    pub fn rx_idle_thr(&mut self) -> RxIdleThrW<'_, IdleCfgSpec> {
        RxIdleThrW::new(self, 0)
    }
    ///Bit 8 - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature
    #[inline(always)]
    pub fn rx_idle_en(&mut self) -> RxIdleEnW<'_, IdleCfgSpec> {
        RxIdleEnW::new(self, 8)
    }
    ///Bit 9 - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle
    #[inline(always)]
    pub fn rx_idle_cond(&mut self) -> RxIdleCondW<'_, IdleCfgSpec> {
        RxIdleCondW::new(self, 9)
    }
    ///Bit 11 - UART receive enable. 0 - hold RX input to high, avoide wrong data input when config pinmux 1 - bypass RX input from PIN software should set it after config pinmux
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, IdleCfgSpec> {
        RxenW::new(self, 11)
    }
    ///Bits 16:23 - Threshold for UART transmit Idle detection (in terms of bits)
    #[inline(always)]
    pub fn tx_idle_thr(&mut self) -> TxIdleThrW<'_, IdleCfgSpec> {
        TxIdleThrW::new(self, 16)
    }
    ///Bit 24 - UART TX Idle Detect Enable 0 - Disable 1 - Enable
    #[inline(always)]
    pub fn tx_idle_en(&mut self) -> TxIdleEnW<'_, IdleCfgSpec> {
        TxIdleEnW::new(self, 24)
    }
    ///Bit 25 - IDLE Detection Condition 0 - Treat as idle if TX pin is logic one 1 - Treat as idle if UART state machine state is idle
    #[inline(always)]
    pub fn tx_idle_cond(&mut self) -> TxIdleCondW<'_, IdleCfgSpec> {
        TxIdleCondW::new(self, 25)
    }
}
/**Idle Configuration Register

You can [`read`](crate::Reg::read) this register and get [`idle_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IdleCfgSpec;
impl crate::RegisterSpec for IdleCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`idle_cfg::R`](R) reader structure
impl crate::Readable for IdleCfgSpec {}
///`write(|w| ..)` method takes [`idle_cfg::W`](W) writer structure
impl crate::Writable for IdleCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDLE_CFG to value 0
impl crate::Resettable for IdleCfgSpec {}
