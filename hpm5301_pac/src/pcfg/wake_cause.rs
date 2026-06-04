///Register `WAKE_CAUSE` reader
pub type R = crate::R<WakeCauseSpec>;
///Register `WAKE_CAUSE` writer
pub type W = crate::W<WakeCauseSpec>;
///Field `CAUSE` reader - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
pub type CauseR = crate::FieldReader<u32>;
///Field `CAUSE` writer - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
pub type CauseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
    #[inline(always)]
    pub fn cause(&self) -> CauseR {
        CauseR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
    #[inline(always)]
    pub fn cause(&mut self) -> CauseW<'_, WakeCauseSpec> {
        CauseW::new(self, 0)
    }
}
/**Wake up source

You can [`read`](crate::Reg::read) this register and get [`wake_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wake_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WakeCauseSpec;
impl crate::RegisterSpec for WakeCauseSpec {
    type Ux = u32;
}
///`read()` method returns [`wake_cause::R`](R) reader structure
impl crate::Readable for WakeCauseSpec {}
///`write(|w| ..)` method takes [`wake_cause::W`](W) writer structure
impl crate::Writable for WakeCauseSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WAKE_CAUSE to value 0
impl crate::Resettable for WakeCauseSpec {}
