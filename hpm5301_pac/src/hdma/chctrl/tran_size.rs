///Register `TranSize` reader
pub type R = crate::R<TranSizeSpec>;
///Register `TranSize` writer
pub type W = crate::W<TranSizeSpec>;
///Field `TRANSIZE` reader - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated.
pub type TransizeR = crate::FieldReader<u32>;
///Field `TRANSIZE` writer - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated.
pub type TransizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated.
    #[inline(always)]
    pub fn transize(&self) -> TransizeR {
        TransizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    ///Bits 0:27 - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated.
    #[inline(always)]
    pub fn transize(&mut self) -> TransizeW<'_, TranSizeSpec> {
        TransizeW::new(self, 0)
    }
}
/**Channel &index0Transfer Size Register

You can [`read`](crate::Reg::read) this register and get [`tran_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tran_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TranSizeSpec;
impl crate::RegisterSpec for TranSizeSpec {
    type Ux = u32;
}
///`read()` method returns [`tran_size::R`](R) reader structure
impl crate::Readable for TranSizeSpec {}
///`write(|w| ..)` method takes [`tran_size::W`](W) writer structure
impl crate::Writable for TranSizeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TranSize to value 0
impl crate::Resettable for TranSizeSpec {}
