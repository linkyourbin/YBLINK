///Register `THRESHOLD` reader
pub type R = crate::R<ThresholdSpec>;
///Register `THRESHOLD` writer
pub type W = crate::W<ThresholdSpec>;
///Field `THRESHOLD` reader - Interrupt priority threshold.
pub type ThresholdR = crate::FieldReader<u32>;
///Field `THRESHOLD` writer - Interrupt priority threshold.
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Interrupt priority threshold.
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Interrupt priority threshold.
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, ThresholdSpec> {
        ThresholdW::new(self, 0)
    }
}
/**Target0 priority threshold

You can [`read`](crate::Reg::read) this register and get [`threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ThresholdSpec;
impl crate::RegisterSpec for ThresholdSpec {
    type Ux = u32;
}
///`read()` method returns [`threshold::R`](R) reader structure
impl crate::Readable for ThresholdSpec {}
///`write(|w| ..)` method takes [`threshold::W`](W) writer structure
impl crate::Writable for ThresholdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets THRESHOLD to value 0
impl crate::Resettable for ThresholdSpec {}
