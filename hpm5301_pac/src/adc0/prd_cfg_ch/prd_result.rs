///Register `prd_result` reader
pub type R = crate::R<PrdResultSpec>;
///Register `prd_result` writer
pub type W = crate::W<PrdResultSpec>;
///Field `CHAN_RESULT` reader - adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel
pub type ChanResultR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel
    #[inline(always)]
    pub fn chan_result(&self) -> ChanResultR {
        ChanResultR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`prd_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prd_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PrdResultSpec;
impl crate::RegisterSpec for PrdResultSpec {
    type Ux = u32;
}
///`read()` method returns [`prd_result::R`](R) reader structure
impl crate::Readable for PrdResultSpec {}
///`write(|w| ..)` method takes [`prd_result::W`](W) writer structure
impl crate::Writable for PrdResultSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets prd_result to value 0
impl crate::Resettable for PrdResultSpec {}
