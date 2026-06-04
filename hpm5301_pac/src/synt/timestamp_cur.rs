///Register `timestamp_cur` reader
pub type R = crate::R<TimestampCurSpec>;
///Register `timestamp_cur` writer
pub type W = crate::W<TimestampCurSpec>;
///Field `VALUE` reader - current timesamp value
pub type ValueR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - current timesamp value
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {}
/**timestamp read value

You can [`read`](crate::Reg::read) this register and get [`timestamp_cur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_cur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TimestampCurSpec;
impl crate::RegisterSpec for TimestampCurSpec {
    type Ux = u32;
}
///`read()` method returns [`timestamp_cur::R`](R) reader structure
impl crate::Readable for TimestampCurSpec {}
///`write(|w| ..)` method takes [`timestamp_cur::W`](W) writer structure
impl crate::Writable for TimestampCurSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets timestamp_cur to value 0
impl crate::Resettable for TimestampCurSpec {}
