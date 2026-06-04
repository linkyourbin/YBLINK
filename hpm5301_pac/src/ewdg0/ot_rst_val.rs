///Register `OT_RST_VAL` reader
pub type R = crate::R<OtRstValSpec>;
///Register `OT_RST_VAL` writer
pub type W = crate::W<OtRstValSpec>;
///Field `OT_RST_VAL` reader - WDT timeout reset value
pub type OtRstValR = crate::FieldReader<u16>;
///Field `OT_RST_VAL` writer - WDT timeout reset value
pub type OtRstValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WDT timeout reset value
    #[inline(always)]
    pub fn ot_rst_val(&self) -> OtRstValR {
        OtRstValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - WDT timeout reset value
    #[inline(always)]
    pub fn ot_rst_val(&mut self) -> OtRstValW<'_, OtRstValSpec> {
        OtRstValW::new(self, 0)
    }
}
/**wdog timeout reset counter value

You can [`read`](crate::Reg::read) this register and get [`ot_rst_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ot_rst_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OtRstValSpec;
impl crate::RegisterSpec for OtRstValSpec {
    type Ux = u32;
}
///`read()` method returns [`ot_rst_val::R`](R) reader structure
impl crate::Readable for OtRstValSpec {}
///`write(|w| ..)` method takes [`ot_rst_val::W`](W) writer structure
impl crate::Writable for OtRstValSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OT_RST_VAL to value 0
impl crate::Resettable for OtRstValSpec {}
