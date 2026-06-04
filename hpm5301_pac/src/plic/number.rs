///Register `NUMBER` reader
pub type R = crate::R<NumberSpec>;
///Register `NUMBER` writer
pub type W = crate::W<NumberSpec>;
///Field `NUM_INTERRUPT` reader - The number of supported interrupt sources
pub type NumInterruptR = crate::FieldReader<u16>;
///Field `NUM_TARGET` reader - The number of supported targets
pub type NumTargetR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - The number of supported interrupt sources
    #[inline(always)]
    pub fn num_interrupt(&self) -> NumInterruptR {
        NumInterruptR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - The number of supported targets
    #[inline(always)]
    pub fn num_target(&self) -> NumTargetR {
        NumTargetR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
/**Number of supported interrupt sources and targets

You can [`read`](crate::Reg::read) this register and get [`number::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`number::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NumberSpec;
impl crate::RegisterSpec for NumberSpec {
    type Ux = u32;
}
///`read()` method returns [`number::R`](R) reader structure
impl crate::Readable for NumberSpec {}
///`write(|w| ..)` method takes [`number::W`](W) writer structure
impl crate::Writable for NumberSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NUMBER to value 0
impl crate::Resettable for NumberSpec {}
