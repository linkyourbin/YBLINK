///Register `SlvDataRCnt` reader
pub type R = crate::R<SlvDataRcntSpec>;
///Register `SlvDataRCnt` writer
pub type W = crate::W<SlvDataRcntSpec>;
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
/**RCnt

You can [`read`](crate::Reg::read) this register and get [`slv_data_rcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_data_rcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SlvDataRcntSpec;
impl crate::RegisterSpec for SlvDataRcntSpec {
    type Ux = u32;
}
///`read()` method returns [`slv_data_rcnt::R`](R) reader structure
impl crate::Readable for SlvDataRcntSpec {}
///`write(|w| ..)` method takes [`slv_data_rcnt::W`](W) writer structure
impl crate::Writable for SlvDataRcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SlvDataRCnt to value 0
impl crate::Resettable for SlvDataRcntSpec {}
