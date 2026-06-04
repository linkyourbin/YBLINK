///Register `MUXCFG[%s]` reader
pub type R = crate::R<MuxcfgSpec>;
///Register `MUXCFG[%s]` writer
pub type W = crate::W<MuxcfgSpec>;
///Field `SOURCE` writer - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the "DMA MUX Mapping"
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `ENABLE` writer - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bits 0:6 - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the "DMA MUX Mapping"
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, MuxcfgSpec> {
        SourceW::new(self, 0)
    }
    ///Bit 31 - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, MuxcfgSpec> {
        EnableW::new(self, 31)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`muxcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MuxcfgSpec;
impl crate::RegisterSpec for MuxcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`muxcfg::R`](R) reader structure
impl crate::Readable for MuxcfgSpec {}
///`write(|w| ..)` method takes [`muxcfg::W`](W) writer structure
impl crate::Writable for MuxcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MUXCFG[%s] to value 0
impl crate::Resettable for MuxcfgSpec {}
