///Register `low_limit` reader
pub type R = crate::R<LowLimitSpec>;
///Register `low_limit` writer
pub type W = crate::W<LowLimitSpec>;
///Field `FREQUENCY` reader - lower frequency
pub type FrequencyR = crate::FieldReader<u32>;
///Field `FREQUENCY` writer - lower frequency
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - lower frequency
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - lower frequency
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, LowLimitSpec> {
        FrequencyW::new(self, 0)
    }
}
/**Clock lower limit

You can [`read`](crate::Reg::read) this register and get [`low_limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LowLimitSpec;
impl crate::RegisterSpec for LowLimitSpec {
    type Ux = u32;
}
///`read()` method returns [`low_limit::R`](R) reader structure
impl crate::Readable for LowLimitSpec {}
///`write(|w| ..)` method takes [`low_limit::W`](W) writer structure
impl crate::Writable for LowLimitSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets low_limit to value 0xffff_ffff
impl crate::Resettable for LowLimitSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
