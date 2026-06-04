///Register `timestamp_new` reader
pub type R = crate::R<TimestampNewSpec>;
///Register `timestamp_new` writer
pub type W = crate::W<TimestampNewSpec>;
///Field `VALUE` reader - new value for timesamp , can be used as set/inc/dec
pub type ValueR = crate::FieldReader<u32>;
///Field `VALUE` writer - new value for timesamp , can be used as set/inc/dec
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - new value for timesamp , can be used as set/inc/dec
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - new value for timesamp , can be used as set/inc/dec
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, TimestampNewSpec> {
        ValueW::new(self, 0)
    }
}
/**timestamp new value register

You can [`read`](crate::Reg::read) this register and get [`timestamp_new::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_new::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TimestampNewSpec;
impl crate::RegisterSpec for TimestampNewSpec {
    type Ux = u32;
}
///`read()` method returns [`timestamp_new::R`](R) reader structure
impl crate::Readable for TimestampNewSpec {}
///`write(|w| ..)` method takes [`timestamp_new::W`](W) writer structure
impl crate::Writable for TimestampNewSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets timestamp_new to value 0
impl crate::Resettable for TimestampNewSpec {}
