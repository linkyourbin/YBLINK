///Register `SOFTWARE_RESET` reader
pub type R = crate::R<SoftwareResetSpec>;
///Register `SOFTWARE_RESET` writer
pub type W = crate::W<SoftwareResetSpec>;
///Field `COUNTER` reader - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset
pub type CounterR = crate::FieldReader<u32>;
///Field `COUNTER` writer - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset
    #[inline(always)]
    pub fn counter(&mut self) -> CounterW<'_, SoftwareResetSpec> {
        CounterW::new(self, 0)
    }
}
/**Software reset counter

You can [`read`](crate::Reg::read) this register and get [`software_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`software_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SoftwareResetSpec;
impl crate::RegisterSpec for SoftwareResetSpec {
    type Ux = u32;
}
///`read()` method returns [`software_reset::R`](R) reader structure
impl crate::Readable for SoftwareResetSpec {}
///`write(|w| ..)` method takes [`software_reset::W`](W) writer structure
impl crate::Writable for SoftwareResetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFTWARE_RESET to value 0
impl crate::Resettable for SoftwareResetSpec {}
