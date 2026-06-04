///Register `WAKEUP_STATUS[%s]` reader
pub type R = crate::R<WakeupStatusSpec>;
///Register `WAKEUP_STATUS[%s]` writer
pub type W = crate::W<WakeupStatusSpec>;
///Field `STATUS` reader - IRQ values
pub type StatusR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - IRQ values
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`wakeup_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WakeupStatusSpec;
impl crate::RegisterSpec for WakeupStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`wakeup_status::R`](R) reader structure
impl crate::Readable for WakeupStatusSpec {}
///`write(|w| ..)` method takes [`wakeup_status::W`](W) writer structure
impl crate::Writable for WakeupStatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WAKEUP_STATUS[%s] to value 0
impl crate::Resettable for WakeupStatusSpec {}
