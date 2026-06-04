///Register `adc_cfg0` reader
pub type R = crate::R<AdcCfg0Spec>;
///Register `adc_cfg0` writer
pub type W = crate::W<AdcCfg0Spec>;
///Field `PORT3_REALTIME` reader - set to enable trg queue stop other queues
pub type Port3RealtimeR = crate::BitReader;
///Field `PORT3_REALTIME` writer - set to enable trg queue stop other queues
pub type Port3RealtimeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_AHB_EN` reader - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;
pub type AdcAhbEnR = crate::BitReader;
///Field `ADC_AHB_EN` writer - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;
pub type AdcAhbEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_SYNC_AHB` reader - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode
pub type SelSyncAhbR = crate::BitReader;
///Field `SEL_SYNC_AHB` writer - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode
pub type SelSyncAhbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - set to enable trg queue stop other queues
    #[inline(always)]
    pub fn port3_realtime(&self) -> Port3RealtimeR {
        Port3RealtimeR::new((self.bits & 1) != 0)
    }
    ///Bit 29 - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;
    #[inline(always)]
    pub fn adc_ahb_en(&self) -> AdcAhbEnR {
        AdcAhbEnR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode
    #[inline(always)]
    pub fn sel_sync_ahb(&self) -> SelSyncAhbR {
        SelSyncAhbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - set to enable trg queue stop other queues
    #[inline(always)]
    pub fn port3_realtime(&mut self) -> Port3RealtimeW<'_, AdcCfg0Spec> {
        Port3RealtimeW::new(self, 0)
    }
    ///Bit 29 - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;
    #[inline(always)]
    pub fn adc_ahb_en(&mut self) -> AdcAhbEnW<'_, AdcCfg0Spec> {
        AdcAhbEnW::new(self, 29)
    }
    ///Bit 31 - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode
    #[inline(always)]
    pub fn sel_sync_ahb(&mut self) -> SelSyncAhbW<'_, AdcCfg0Spec> {
        SelSyncAhbW::new(self, 31)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`adc_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcCfg0Spec;
impl crate::RegisterSpec for AdcCfg0Spec {
    type Ux = u32;
}
///`read()` method returns [`adc_cfg0::R`](R) reader structure
impl crate::Readable for AdcCfg0Spec {}
///`write(|w| ..)` method takes [`adc_cfg0::W`](W) writer structure
impl crate::Writable for AdcCfg0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets adc_cfg0 to value 0
impl crate::Resettable for AdcCfg0Spec {}
