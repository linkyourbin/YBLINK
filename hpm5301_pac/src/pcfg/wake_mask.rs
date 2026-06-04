///Register `WAKE_MASK` reader
pub type R = crate::R<WakeMaskSpec>;
///Register `WAKE_MASK` writer
pub type W = crate::W<WakeMaskSpec>;
///Field `MASK` reader - mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
pub type MaskR = crate::FieldReader<u32>;
///Field `MASK` writer - mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, WakeMaskSpec> {
        MaskW::new(self, 0)
    }
}
/**Wake up mask

You can [`read`](crate::Reg::read) this register and get [`wake_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wake_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WakeMaskSpec;
impl crate::RegisterSpec for WakeMaskSpec {
    type Ux = u32;
}
///`read()` method returns [`wake_mask::R`](R) reader structure
impl crate::Readable for WakeMaskSpec {}
///`write(|w| ..)` method takes [`wake_mask::W`](W) writer structure
impl crate::Writable for WakeMaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WAKE_MASK to value 0
impl crate::Resettable for WakeMaskSpec {}
