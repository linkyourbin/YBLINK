///Register `LIFECYCLE` reader
pub type R = crate::R<LifecycleSpec>;
///Register `LIFECYCLE` writer
pub type W = crate::W<LifecycleSpec>;
///Field `LIFECYCLE` reader - lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow
pub type LifecycleR = crate::FieldReader;
impl R {
    ///Bits 0:7 - lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow
    #[inline(always)]
    pub fn lifecycle(&self) -> LifecycleR {
        LifecycleR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
/**Lifecycle

You can [`read`](crate::Reg::read) this register and get [`lifecycle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifecycle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LifecycleSpec;
impl crate::RegisterSpec for LifecycleSpec {
    type Ux = u32;
}
///`read()` method returns [`lifecycle::R`](R) reader structure
impl crate::Readable for LifecycleSpec {}
///`write(|w| ..)` method takes [`lifecycle::W`](W) writer structure
impl crate::Writable for LifecycleSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LIFECYCLE to value 0
impl crate::Resettable for LifecycleSpec {}
