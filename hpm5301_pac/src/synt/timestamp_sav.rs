///Register `timestamp_sav` reader
pub type R = crate::R<TimestampSavSpec>;
///Register `timestamp_sav` writer
pub type W = crate::W<TimestampSavSpec>;
///Field `VALUE` reader - use the trigger to save timesamp here
pub type ValueR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - use the trigger to save timesamp here
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {}
/**timestamp trig save value

You can [`read`](crate::Reg::read) this register and get [`timestamp_sav::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_sav::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TimestampSavSpec;
impl crate::RegisterSpec for TimestampSavSpec {
    type Ux = u32;
}
///`read()` method returns [`timestamp_sav::R`](R) reader structure
impl crate::Readable for TimestampSavSpec {}
///`write(|w| ..)` method takes [`timestamp_sav::W`](W) writer structure
impl crate::Writable for TimestampSavSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets timestamp_sav to value 0
impl crate::Resettable for TimestampSavSpec {}
