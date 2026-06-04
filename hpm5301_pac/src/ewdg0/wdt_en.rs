///Register `WDT_EN` reader
pub type R = crate::R<WdtEnSpec>;
///Register `WDT_EN` writer
pub type W = crate::W<WdtEnSpec>;
///Field `WDOG_EN` reader - Wdog is enabled, the re-written of this register is impacted by enable lock function
pub type WdogEnR = crate::BitReader;
///Field `WDOG_EN` writer - Wdog is enabled, the re-written of this register is impacted by enable lock function
pub type WdogEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wdog is enabled, the re-written of this register is impacted by enable lock function
    #[inline(always)]
    pub fn wdog_en(&self) -> WdogEnR {
        WdogEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Wdog is enabled, the re-written of this register is impacted by enable lock function
    #[inline(always)]
    pub fn wdog_en(&mut self) -> WdogEnW<'_, WdtEnSpec> {
        WdogEnW::new(self, 0)
    }
}
/**Wdog enable

You can [`read`](crate::Reg::read) this register and get [`wdt_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdtEnSpec;
impl crate::RegisterSpec for WdtEnSpec {
    type Ux = u32;
}
///`read()` method returns [`wdt_en::R`](R) reader structure
impl crate::Readable for WdtEnSpec {}
///`write(|w| ..)` method takes [`wdt_en::W`](W) writer structure
impl crate::Writable for WdtEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDT_EN to value 0
impl crate::Resettable for WdtEnSpec {}
