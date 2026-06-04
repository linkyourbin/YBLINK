///Register `INTTCSTS` reader
pub type R = crate::R<InttcstsSpec>;
///Register `INTTCSTS` writer
pub type W = crate::W<InttcstsSpec>;
///Field `STS` writer - The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<'_, InttcstsSpec> {
        StsW::new(self, 0)
    }
}
/**Trans Complete Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`inttcsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttcsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct InttcstsSpec;
impl crate::RegisterSpec for InttcstsSpec {
    type Ux = u32;
}
///`read()` method returns [`inttcsts::R`](R) reader structure
impl crate::Readable for InttcstsSpec {}
///`write(|w| ..)` method takes [`inttcsts::W`](W) writer structure
impl crate::Writable for InttcstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTTCSTS to value 0
impl crate::Resettable for InttcstsSpec {}
