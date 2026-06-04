///Register `ChAbort` reader
pub type R = crate::R<ChAbortSpec>;
///Register `ChAbort` writer
pub type W = crate::W<ChAbortSpec>;
///Field `CHABORT` writer - Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)
pub type ChabortW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)
    #[inline(always)]
    pub fn chabort(&mut self) -> ChabortW<'_, ChAbortSpec> {
        ChabortW::new(self, 0)
    }
}
/**Channel Abort Register

You can [`read`](crate::Reg::read) this register and get [`ch_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ChAbortSpec;
impl crate::RegisterSpec for ChAbortSpec {
    type Ux = u32;
}
///`read()` method returns [`ch_abort::R`](R) reader structure
impl crate::Readable for ChAbortSpec {}
///`write(|w| ..)` method takes [`ch_abort::W`](W) writer structure
impl crate::Writable for ChAbortSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ChAbort to value 0
impl crate::Resettable for ChAbortSpec {}
