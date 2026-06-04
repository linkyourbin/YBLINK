///Register `CMP[%s]` reader
pub type R = crate::R<CmpSpec>;
///Register `CMP[%s]` writer
pub type W = crate::W<CmpSpec>;
///Field `CMP` reader - comparator value, the output will assert when counter count to this value
pub type CmpR = crate::FieldReader<u32>;
///Field `CMP` writer - comparator value, the output will assert when counter count to this value
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - comparator value, the output will assert when counter count to this value
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - comparator value, the output will assert when counter count to this value
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, CmpSpec> {
        CmpW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CmpSpec;
impl crate::RegisterSpec for CmpSpec {
    type Ux = u32;
}
///`read()` method returns [`cmp::R`](R) reader structure
impl crate::Readable for CmpSpec {}
///`write(|w| ..)` method takes [`cmp::W`](W) writer structure
impl crate::Writable for CmpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMP[%s] to value 0
impl crate::Resettable for CmpSpec {}
