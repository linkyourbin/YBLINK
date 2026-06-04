///Register `SlvDataWCnt` reader
pub type R = crate::R<SlvDataWcntSpec>;
///Register `SlvDataWCnt` writer
pub type W = crate::W<SlvDataWcntSpec>;
///Field `VAL` reader - No description available
pub type ValR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - No description available
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {}
/**WCnt

You can [`read`](crate::Reg::read) this register and get [`slv_data_wcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_data_wcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SlvDataWcntSpec;
impl crate::RegisterSpec for SlvDataWcntSpec {
    type Ux = u32;
}
///`read()` method returns [`slv_data_wcnt::R`](R) reader structure
impl crate::Readable for SlvDataWcntSpec {}
///`write(|w| ..)` method takes [`slv_data_wcnt::W`](W) writer structure
impl crate::Writable for SlvDataWcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SlvDataWCnt to value 0
impl crate::Resettable for SlvDataWcntSpec {}
