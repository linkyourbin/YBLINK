///Register `current` reader
pub type R = crate::R<CurrentSpec>;
///Register `current` writer
pub type W = crate::W<CurrentSpec>;
///Field `FREQUENCY` reader - self updating measure result
pub type FrequencyR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - self updating measure result
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new(self.bits)
    }
}
impl W {}
/**Clock measure result

You can [`read`](crate::Reg::read) this register and get [`current::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CurrentSpec;
impl crate::RegisterSpec for CurrentSpec {
    type Ux = u32;
}
///`read()` method returns [`current::R`](R) reader structure
impl crate::Readable for CurrentSpec {}
///`write(|w| ..)` method takes [`current::W`](W) writer structure
impl crate::Writable for CurrentSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets current to value 0
impl crate::Resettable for CurrentSpec {}
