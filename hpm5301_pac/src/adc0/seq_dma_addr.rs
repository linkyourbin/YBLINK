///Register `seq_dma_addr` reader
pub type R = crate::R<SeqDmaAddrSpec>;
///Register `seq_dma_addr` writer
pub type W = crate::W<SeqDmaAddrSpec>;
///Field `TAR_ADDR` reader - dma target address, should be 4-byte aligned
pub type TarAddrR = crate::FieldReader<u32>;
///Field `TAR_ADDR` writer - dma target address, should be 4-byte aligned
pub type TarAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - dma target address, should be 4-byte aligned
    #[inline(always)]
    pub fn tar_addr(&self) -> TarAddrR {
        TarAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - dma target address, should be 4-byte aligned
    #[inline(always)]
    pub fn tar_addr(&mut self) -> TarAddrW<'_, SeqDmaAddrSpec> {
        TarAddrW::new(self, 2)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`seq_dma_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_dma_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SeqDmaAddrSpec;
impl crate::RegisterSpec for SeqDmaAddrSpec {
    type Ux = u32;
}
///`read()` method returns [`seq_dma_addr::R`](R) reader structure
impl crate::Readable for SeqDmaAddrSpec {}
///`write(|w| ..)` method takes [`seq_dma_addr::W`](W) writer structure
impl crate::Writable for SeqDmaAddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets seq_dma_addr to value 0
impl crate::Resettable for SeqDmaAddrSpec {}
