///Register `result` reader
pub type R = crate::R<ResultSpec>;
///Register `result` writer
pub type W = crate::W<ResultSpec>;
///Field `RESULT` reader - crc result
pub type ResultR = crate::FieldReader<u32>;
///Field `RESULT` writer - crc result
pub type ResultW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - crc result
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - crc result
    #[inline(always)]
    pub fn result(&mut self) -> ResultW<'_, ResultSpec> {
        ResultW::new(self, 0)
    }
}
/**chn&index0 result

You can [`read`](crate::Reg::read) this register and get [`result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
///`read()` method returns [`result::R`](R) reader structure
impl crate::Readable for ResultSpec {}
///`write(|w| ..)` method takes [`result::W`](W) writer structure
impl crate::Writable for ResultSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets result to value 0
impl crate::Resettable for ResultSpec {}
