///Register `OT_INT_VAL` reader
pub type R = crate::R<OtIntValSpec>;
///Register `OT_INT_VAL` writer
pub type W = crate::W<OtIntValSpec>;
///Field `OT_INT_VAL` reader - WDT timeout interrupt value
pub type OtIntValR = crate::FieldReader<u16>;
///Field `OT_INT_VAL` writer - WDT timeout interrupt value
pub type OtIntValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WDT timeout interrupt value
    #[inline(always)]
    pub fn ot_int_val(&self) -> OtIntValR {
        OtIntValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - WDT timeout interrupt value
    #[inline(always)]
    pub fn ot_int_val(&mut self) -> OtIntValW<'_, OtIntValSpec> {
        OtIntValW::new(self, 0)
    }
}
/**wdog timeout interrupt counter value

You can [`read`](crate::Reg::read) this register and get [`ot_int_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ot_int_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OtIntValSpec;
impl crate::RegisterSpec for OtIntValSpec {
    type Ux = u32;
}
///`read()` method returns [`ot_int_val::R`](R) reader structure
impl crate::Readable for OtIntValSpec {}
///`write(|w| ..)` method takes [`ot_int_val::W`](W) writer structure
impl crate::Writable for OtIntValSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OT_INT_VAL to value 0
impl crate::Resettable for OtIntValSpec {}
