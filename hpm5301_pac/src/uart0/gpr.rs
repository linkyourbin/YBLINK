///Register `GPR` reader
pub type R = crate::R<GprSpec>;
///Register `GPR` writer
pub type W = crate::W<GprSpec>;
///Field `DATA` reader - A one-byte storage register
pub type DataR = crate::FieldReader;
///Field `DATA` writer - A one-byte storage register
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - A one-byte storage register
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - A one-byte storage register
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, GprSpec> {
        DataW::new(self, 0)
    }
}
/**GPR Register

You can [`read`](crate::Reg::read) this register and get [`gpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GprSpec;
impl crate::RegisterSpec for GprSpec {
    type Ux = u32;
}
///`read()` method returns [`gpr::R`](R) reader structure
impl crate::Readable for GprSpec {}
///`write(|w| ..)` method takes [`gpr::W`](W) writer structure
impl crate::Writable for GprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPR to value 0
impl crate::Resettable for GprSpec {}
