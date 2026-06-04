///Register `WAKEUP_ENABLE[%s]` reader
pub type R = crate::R<WakeupEnableSpec>;
///Register `WAKEUP_ENABLE[%s]` writer
pub type W = crate::W<WakeupEnableSpec>;
///Field `ENABLE` reader - IRQ wakeup enable
pub type EnableR = crate::FieldReader<u32>;
///Field `ENABLE` writer - IRQ wakeup enable
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IRQ wakeup enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - IRQ wakeup enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, WakeupEnableSpec> {
        EnableW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`wakeup_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WakeupEnableSpec;
impl crate::RegisterSpec for WakeupEnableSpec {
    type Ux = u32;
}
///`read()` method returns [`wakeup_enable::R`](R) reader structure
impl crate::Readable for WakeupEnableSpec {}
///`write(|w| ..)` method takes [`wakeup_enable::W`](W) writer structure
impl crate::Writable for WakeupEnableSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WAKEUP_ENABLE[%s] to value 0
impl crate::Resettable for WakeupEnableSpec {}
