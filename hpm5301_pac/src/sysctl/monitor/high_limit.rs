///Register `high_limit` reader
pub type R = crate::R<HighLimitSpec>;
///Register `high_limit` writer
pub type W = crate::W<HighLimitSpec>;
///Field `FREQUENCY` reader - upper frequency
pub type FrequencyR = crate::FieldReader<u32>;
///Field `FREQUENCY` writer - upper frequency
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - upper frequency
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - upper frequency
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, HighLimitSpec> {
        FrequencyW::new(self, 0)
    }
}
/**Clock upper limit

You can [`read`](crate::Reg::read) this register and get [`high_limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high_limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HighLimitSpec;
impl crate::RegisterSpec for HighLimitSpec {
    type Ux = u32;
}
///`read()` method returns [`high_limit::R`](R) reader structure
impl crate::Readable for HighLimitSpec {}
///`write(|w| ..)` method takes [`high_limit::W`](W) writer structure
impl crate::Writable for HighLimitSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets high_limit to value 0
impl crate::Resettable for HighLimitSpec {}
