///Register `INTERRSTS` reader
pub type R = crate::R<InterrstsSpec>;
///Register `INTERRSTS` writer
pub type W = crate::W<InterrstsSpec>;
///Field `STS` writer - The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<'_, InterrstsSpec> {
        StsW::new(self, 0)
    }
}
/**Error Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`interrsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct InterrstsSpec;
impl crate::RegisterSpec for InterrstsSpec {
    type Ux = u32;
}
///`read()` method returns [`interrsts::R`](R) reader structure
impl crate::Readable for InterrstsSpec {}
///`write(|w| ..)` method takes [`interrsts::W`](W) writer structure
impl crate::Writable for InterrstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTERRSTS to value 0
impl crate::Resettable for InterrstsSpec {}
