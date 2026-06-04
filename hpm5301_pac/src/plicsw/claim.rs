///Register `CLAIM` reader
pub type R = crate::R<ClaimSpec>;
///Register `CLAIM` writer
pub type W = crate::W<ClaimSpec>;
///Field `INTERRUPT_ID` reader - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed).
pub type InterruptIdR = crate::BitReader;
///Field `INTERRUPT_ID` writer - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed).
pub type InterruptIdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed).
    #[inline(always)]
    pub fn interrupt_id(&self) -> InterruptIdR {
        InterruptIdR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed).
    #[inline(always)]
    pub fn interrupt_id(&mut self) -> InterruptIdW<'_, ClaimSpec> {
        InterruptIdW::new(self, 0)
    }
}
/**Claim and complete.

You can [`read`](crate::Reg::read) this register and get [`claim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ClaimSpec;
impl crate::RegisterSpec for ClaimSpec {
    type Ux = u32;
}
///`read()` method returns [`claim::R`](R) reader structure
impl crate::Readable for ClaimSpec {}
///`write(|w| ..)` method takes [`claim::W`](W) writer structure
impl crate::Writable for ClaimSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLAIM to value 0
impl crate::Resettable for ClaimSpec {}
