///Register `WDT_REFRESH_REG` reader
pub type R = crate::R<WdtRefreshRegSpec>;
///Register `WDT_REFRESH_REG` writer
pub type W = crate::W<WdtRefreshRegSpec>;
///Field `WDT_REFRESH_REG` writer - Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose
pub type WdtRefreshRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose
    #[inline(always)]
    pub fn wdt_refresh_reg(&mut self) -> WdtRefreshRegW<'_, WdtRefreshRegSpec> {
        WdtRefreshRegW::new(self, 0)
    }
}
/**wdog refresh register

You can [`read`](crate::Reg::read) this register and get [`wdt_refresh_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_refresh_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdtRefreshRegSpec;
impl crate::RegisterSpec for WdtRefreshRegSpec {
    type Ux = u32;
}
///`read()` method returns [`wdt_refresh_reg::R`](R) reader structure
impl crate::Readable for WdtRefreshRegSpec {}
///`write(|w| ..)` method takes [`wdt_refresh_reg::W`](W) writer structure
impl crate::Writable for WdtRefreshRegSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDT_REFRESH_REG to value 0
impl crate::Resettable for WdtRefreshRegSpec {}
