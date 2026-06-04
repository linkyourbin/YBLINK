///Register `trg_dma_addr` reader
pub type R = crate::R<TrgDmaAddrSpec>;
///Register `trg_dma_addr` writer
pub type W = crate::W<TrgDmaAddrSpec>;
///Field `TRG_DMA_ADDR` reader - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)
pub type TrgDmaAddrR = crate::FieldReader<u32>;
///Field `TRG_DMA_ADDR` writer - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)
pub type TrgDmaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)
    #[inline(always)]
    pub fn trg_dma_addr(&self) -> TrgDmaAddrR {
        TrgDmaAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)
    #[inline(always)]
    pub fn trg_dma_addr(&mut self) -> TrgDmaAddrW<'_, TrgDmaAddrSpec> {
        TrgDmaAddrW::new(self, 2)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`trg_dma_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg_dma_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrgDmaAddrSpec;
impl crate::RegisterSpec for TrgDmaAddrSpec {
    type Ux = u32;
}
///`read()` method returns [`trg_dma_addr::R`](R) reader structure
impl crate::Readable for TrgDmaAddrSpec {}
///`write(|w| ..)` method takes [`trg_dma_addr::W`](W) writer structure
impl crate::Writable for TrgDmaAddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets trg_dma_addr to value 0
impl crate::Resettable for TrgDmaAddrSpec {}
