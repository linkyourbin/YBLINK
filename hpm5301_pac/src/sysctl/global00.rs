///Register `global00` reader
pub type R = crate::R<Global00Spec>;
///Register `global00` writer
pub type W = crate::W<Global00Spec>;
///Field `MUX` reader - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7
pub type MuxR = crate::FieldReader;
///Field `MUX` writer - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<'_, Global00Spec> {
        MuxW::new(self, 0)
    }
}
/**Clock senario

You can [`read`](crate::Reg::read) this register and get [`global00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`global00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Global00Spec;
impl crate::RegisterSpec for Global00Spec {
    type Ux = u32;
}
///`read()` method returns [`global00::R`](R) reader structure
impl crate::Readable for Global00Spec {}
///`write(|w| ..)` method takes [`global00::W`](W) writer structure
impl crate::Writable for Global00Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets global00 to value 0
impl crate::Resettable for Global00Spec {}
