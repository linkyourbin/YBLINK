///Register `DGO_CTR3` reader
pub type R = crate::R<DgoCtr3Spec>;
///Register `DGO_CTR3` writer
pub type W = crate::W<DgoCtr3Spec>;
///Field `WAKEUP_COUNTER` reader - software wakeup counter
pub type WakeupCounterR = crate::FieldReader<u32>;
///Field `WAKEUP_COUNTER` writer - software wakeup counter
pub type WakeupCounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - software wakeup counter
    #[inline(always)]
    pub fn wakeup_counter(&self) -> WakeupCounterR {
        WakeupCounterR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - software wakeup counter
    #[inline(always)]
    pub fn wakeup_counter(&mut self) -> WakeupCounterW<'_, DgoCtr3Spec> {
        WakeupCounterW::new(self, 0)
    }
}
/**control register 3

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoCtr3Spec;
impl crate::RegisterSpec for DgoCtr3Spec {
    type Ux = u32;
}
///`read()` method returns [`dgo_ctr3::R`](R) reader structure
impl crate::Readable for DgoCtr3Spec {}
///`write(|w| ..)` method takes [`dgo_ctr3::W`](W) writer structure
impl crate::Writable for DgoCtr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_CTR3 to value 0
impl crate::Resettable for DgoCtr3Spec {}
