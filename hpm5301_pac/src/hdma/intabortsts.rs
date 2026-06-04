///Register `INTABORTSTS` reader
pub type R = crate::R<IntabortstsSpec>;
///Register `INTABORTSTS` writer
pub type W = crate::W<IntabortstsSpec>;
///Field `STS` writer - The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<'_, IntabortstsSpec> {
        StsW::new(self, 0)
    }
}
/**Abort Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`intabortsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intabortsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntabortstsSpec;
impl crate::RegisterSpec for IntabortstsSpec {
    type Ux = u32;
}
///`read()` method returns [`intabortsts::R`](R) reader structure
impl crate::Readable for IntabortstsSpec {}
///`write(|w| ..)` method takes [`intabortsts::W`](W) writer structure
impl crate::Writable for IntabortstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTABORTSTS to value 0
impl crate::Resettable for IntabortstsSpec {}
