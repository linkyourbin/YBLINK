///Register `DMACtrl` reader
pub type R = crate::R<DmactrlSpec>;
///Register `DMACtrl` writer
pub type W = crate::W<DmactrlSpec>;
///Field `RESET` writer - Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction.
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction.
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, DmactrlSpec> {
        ResetW::new(self, 0)
    }
}
/**DMAC Control Register

You can [`read`](crate::Reg::read) this register and get [`dmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
///`read()` method returns [`dmactrl::R`](R) reader structure
impl crate::Readable for DmactrlSpec {}
///`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACtrl to value 0
impl crate::Resettable for DmactrlSpec {}
