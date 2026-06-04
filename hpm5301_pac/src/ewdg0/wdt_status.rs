///Register `WDT_STATUS` reader
pub type R = crate::R<WdtStatusSpec>;
///Register `WDT_STATUS` writer
pub type W = crate::W<WdtStatusSpec>;
///Field `REF_VIO` writer - Refresh fail Write one to clear the bit
pub type RefVioW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REF_UNL_FAIL` writer - Refresh unlock fail Write one to clear the bit
pub type RefUnlFailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTL_VIO` writer - Violate register update protection mechanism Write one to clear the bit
pub type CtlVioW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTL_UNL_FAIL` writer - Unlock ctrl reg update protection fail Write one to clear the bit
pub type CtlUnlFailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OT_INT` writer - Timeout happens, a interrupt will happen once enable bit set This bit can be cleared only by refreshing wdt or reset
pub type OtIntW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OT_RST` writer - Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset
pub type OtRstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARITY_ERROR` writer - parity error Write one to clear the bit
pub type ParityErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - Refresh fail Write one to clear the bit
    #[inline(always)]
    pub fn ref_vio(&mut self) -> RefVioW<'_, WdtStatusSpec> {
        RefVioW::new(self, 0)
    }
    ///Bit 1 - Refresh unlock fail Write one to clear the bit
    #[inline(always)]
    pub fn ref_unl_fail(&mut self) -> RefUnlFailW<'_, WdtStatusSpec> {
        RefUnlFailW::new(self, 1)
    }
    ///Bit 2 - Violate register update protection mechanism Write one to clear the bit
    #[inline(always)]
    pub fn ctl_vio(&mut self) -> CtlVioW<'_, WdtStatusSpec> {
        CtlVioW::new(self, 2)
    }
    ///Bit 3 - Unlock ctrl reg update protection fail Write one to clear the bit
    #[inline(always)]
    pub fn ctl_unl_fail(&mut self) -> CtlUnlFailW<'_, WdtStatusSpec> {
        CtlUnlFailW::new(self, 3)
    }
    ///Bit 4 - Timeout happens, a interrupt will happen once enable bit set This bit can be cleared only by refreshing wdt or reset
    #[inline(always)]
    pub fn ot_int(&mut self) -> OtIntW<'_, WdtStatusSpec> {
        OtIntW::new(self, 4)
    }
    ///Bit 5 - Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset
    #[inline(always)]
    pub fn ot_rst(&mut self) -> OtRstW<'_, WdtStatusSpec> {
        OtRstW::new(self, 5)
    }
    ///Bit 6 - parity error Write one to clear the bit
    #[inline(always)]
    pub fn parity_error(&mut self) -> ParityErrorW<'_, WdtStatusSpec> {
        ParityErrorW::new(self, 6)
    }
}
/**wdog status register

You can [`read`](crate::Reg::read) this register and get [`wdt_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdtStatusSpec;
impl crate::RegisterSpec for WdtStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`wdt_status::R`](R) reader structure
impl crate::Readable for WdtStatusSpec {}
///`write(|w| ..)` method takes [`wdt_status::W`](W) writer structure
impl crate::Writable for WdtStatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDT_STATUS to value 0
impl crate::Resettable for WdtStatusSpec {}
