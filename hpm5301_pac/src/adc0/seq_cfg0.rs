///Register `seq_cfg0` reader
pub type R = crate::R<SeqCfg0Spec>;
///Register `seq_cfg0` writer
pub type W = crate::W<SeqCfg0Spec>;
///Field `HW_TRIG_EN` reader - set to enable external HW trigger, only trigger on posedge
pub type HwTrigEnR = crate::BitReader;
///Field `HW_TRIG_EN` writer - set to enable external HW trigger, only trigger on posedge
pub type HwTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW_TRIG_EN` reader - set to enable SW trigger
pub type SwTrigEnR = crate::BitReader;
///Field `SW_TRIG_EN` writer - set to enable SW trigger
pub type SwTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW_TRIG` writer - SW trigger, pulse signal, cleared by HW one cycle later
pub type SwTrigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT_EN` reader - if set, HW will continue process the queue till end(seq_len) after trigger once
pub type ContEnR = crate::BitReader;
///Field `CONT_EN` writer - if set, HW will continue process the queue till end(seq_len) after trigger once
pub type ContEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESTART_EN` reader - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used
pub type RestartEnR = crate::BitReader;
///Field `RESTART_EN` writer - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used
pub type RestartEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_LEN` reader - sequence queue length, 0 for one, 0xF for 16
pub type SeqLenR = crate::FieldReader;
///Field `SEQ_LEN` writer - sequence queue length, 0 for one, 0xF for 16
pub type SeqLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CYCLE` reader - current dma write cycle bit
pub type CycleR = crate::BitReader;
impl R {
    ///Bit 0 - set to enable external HW trigger, only trigger on posedge
    #[inline(always)]
    pub fn hw_trig_en(&self) -> HwTrigEnR {
        HwTrigEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - set to enable SW trigger
    #[inline(always)]
    pub fn sw_trig_en(&self) -> SwTrigEnR {
        SwTrigEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - if set, HW will continue process the queue till end(seq_len) after trigger once
    #[inline(always)]
    pub fn cont_en(&self) -> ContEnR {
        ContEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used
    #[inline(always)]
    pub fn restart_en(&self) -> RestartEnR {
        RestartEnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - sequence queue length, 0 for one, 0xF for 16
    #[inline(always)]
    pub fn seq_len(&self) -> SeqLenR {
        SeqLenR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - current dma write cycle bit
    #[inline(always)]
    pub fn cycle(&self) -> CycleR {
        CycleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - set to enable external HW trigger, only trigger on posedge
    #[inline(always)]
    pub fn hw_trig_en(&mut self) -> HwTrigEnW<'_, SeqCfg0Spec> {
        HwTrigEnW::new(self, 0)
    }
    ///Bit 1 - set to enable SW trigger
    #[inline(always)]
    pub fn sw_trig_en(&mut self) -> SwTrigEnW<'_, SeqCfg0Spec> {
        SwTrigEnW::new(self, 1)
    }
    ///Bit 2 - SW trigger, pulse signal, cleared by HW one cycle later
    #[inline(always)]
    pub fn sw_trig(&mut self) -> SwTrigW<'_, SeqCfg0Spec> {
        SwTrigW::new(self, 2)
    }
    ///Bit 3 - if set, HW will continue process the queue till end(seq_len) after trigger once
    #[inline(always)]
    pub fn cont_en(&mut self) -> ContEnW<'_, SeqCfg0Spec> {
        ContEnW::new(self, 3)
    }
    ///Bit 4 - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used
    #[inline(always)]
    pub fn restart_en(&mut self) -> RestartEnW<'_, SeqCfg0Spec> {
        RestartEnW::new(self, 4)
    }
    ///Bits 8:11 - sequence queue length, 0 for one, 0xF for 16
    #[inline(always)]
    pub fn seq_len(&mut self) -> SeqLenW<'_, SeqCfg0Spec> {
        SeqLenW::new(self, 8)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`seq_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SeqCfg0Spec;
impl crate::RegisterSpec for SeqCfg0Spec {
    type Ux = u32;
}
///`read()` method returns [`seq_cfg0::R`](R) reader structure
impl crate::Readable for SeqCfg0Spec {}
///`write(|w| ..)` method takes [`seq_cfg0::W`](W) writer structure
impl crate::Writable for SeqCfg0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets seq_cfg0 to value 0
impl crate::Resettable for SeqCfg0Spec {}
