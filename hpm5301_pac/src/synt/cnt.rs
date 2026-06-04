///Register `cnt` reader
pub type R = crate::R<CntSpec>;
///Register `cnt` writer
pub type W = crate::W<CntSpec>;
///Field `CNT` reader - counter
pub type CntR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - counter
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl W {}
/**Counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CntSpec {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets cnt to value 0
impl crate::Resettable for CntSpec {}
