///Register `SlvDataCnt` reader
pub type R = crate::R<SlvDataCntSpec>;
///Register `SlvDataCnt` writer
pub type W = crate::W<SlvDataCntSpec>;
///Field `RCNT` reader - Slave received data count
pub type RcntR = crate::FieldReader<u16>;
///Field `WCNT` reader - Slave transmitted data count
pub type WcntR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Slave received data count
    #[inline(always)]
    pub fn rcnt(&self) -> RcntR {
        RcntR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Slave transmitted data count
    #[inline(always)]
    pub fn wcnt(&self) -> WcntR {
        WcntR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
/**Slave Data Count Register

You can [`read`](crate::Reg::read) this register and get [`slv_data_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_data_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SlvDataCntSpec;
impl crate::RegisterSpec for SlvDataCntSpec {
    type Ux = u32;
}
///`read()` method returns [`slv_data_cnt::R`](R) reader structure
impl crate::Readable for SlvDataCntSpec {}
///`write(|w| ..)` method takes [`slv_data_cnt::W`](W) writer structure
impl crate::Writable for SlvDataCntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SlvDataCnt to value 0
impl crate::Resettable for SlvDataCntSpec {}
