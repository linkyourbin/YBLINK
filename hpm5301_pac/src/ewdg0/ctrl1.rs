///Register `CTRL1` reader
pub type R = crate::R<Ctrl1Spec>;
///Register `CTRL1` writer
pub type W = crate::W<Ctrl1Spec>;
///Field `PARITY_FAIL_INT_EN` reader - Parity error will trigger a interrupt
pub type ParityFailIntEnR = crate::BitReader;
///Field `PARITY_FAIL_INT_EN` writer - Parity error will trigger a interrupt
pub type ParityFailIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARITY_FAIL_RST_EN` reader - Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits
pub type ParityFailRstEnR = crate::BitReader;
///Field `PARITY_FAIL_RST_EN` writer - Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits
pub type ParityFailRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNL_CTL_FAIL_INT_EN` reader - Unlock register update failure will trigger a interrupt
pub type UnlCtlFailIntEnR = crate::BitReader;
///Field `UNL_CTL_FAIL_INT_EN` writer - Unlock register update failure will trigger a interrupt
pub type UnlCtlFailIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNL_CTL_FAIL_RST_EN` reader - Unlock register update failure will trigger a reset
pub type UnlCtlFailRstEnR = crate::BitReader;
///Field `UNL_CTL_FAIL_RST_EN` writer - Unlock register update failure will trigger a reset
pub type UnlCtlFailRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTL_VIO_INT_EN` reader - Ctrl update violation will trigger a interrupt
pub type CtlVioIntEnR = crate::BitReader;
///Field `CTL_VIO_INT_EN` writer - Ctrl update violation will trigger a interrupt
pub type CtlVioIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTL_VIO_RST_EN` reader - Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them
pub type CtlVioRstEnR = crate::BitReader;
///Field `CTL_VIO_RST_EN` writer - Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them
pub type CtlVioRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OT_INT_EN` reader - WDT can generate an interrupt warning before timeout
pub type OtIntEnR = crate::BitReader;
///Field `OT_INT_EN` writer - WDT can generate an interrupt warning before timeout
pub type OtIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OT_RST_EN` reader - WDT overtime will generate a reset
pub type OtRstEnR = crate::BitReader;
///Field `OT_RST_EN` writer - WDT overtime will generate a reset
pub type OtRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNL_REF_FAIL_INT_EN` reader - Refresh unlock fail will trigger a interrupt
pub type UnlRefFailIntEnR = crate::BitReader;
///Field `UNL_REF_FAIL_INT_EN` writer - Refresh unlock fail will trigger a interrupt
pub type UnlRefFailIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNL_REF_FAIL_RST_EN` reader - Refresh unlock fail will trigger a reset
pub type UnlRefFailRstEnR = crate::BitReader;
///Field `UNL_REF_FAIL_RST_EN` writer - Refresh unlock fail will trigger a reset
pub type UnlRefFailRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REF_FAIL_INT_EN` reader - Refresh violation will trigger an interrupt
pub type RefFailIntEnR = crate::BitReader;
///Field `REF_FAIL_INT_EN` writer - Refresh violation will trigger an interrupt
pub type RefFailIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REF_FAIL_RST_EN` reader - Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt.
pub type RefFailRstEnR = crate::BitReader;
///Field `REF_FAIL_RST_EN` writer - Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt.
pub type RefFailRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Parity error will trigger a interrupt
    #[inline(always)]
    pub fn parity_fail_int_en(&self) -> ParityFailIntEnR {
        ParityFailIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits
    #[inline(always)]
    pub fn parity_fail_rst_en(&self) -> ParityFailRstEnR {
        ParityFailRstEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Unlock register update failure will trigger a interrupt
    #[inline(always)]
    pub fn unl_ctl_fail_int_en(&self) -> UnlCtlFailIntEnR {
        UnlCtlFailIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Unlock register update failure will trigger a reset
    #[inline(always)]
    pub fn unl_ctl_fail_rst_en(&self) -> UnlCtlFailRstEnR {
        UnlCtlFailRstEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Ctrl update violation will trigger a interrupt
    #[inline(always)]
    pub fn ctl_vio_int_en(&self) -> CtlVioIntEnR {
        CtlVioIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them
    #[inline(always)]
    pub fn ctl_vio_rst_en(&self) -> CtlVioRstEnR {
        CtlVioRstEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - WDT can generate an interrupt warning before timeout
    #[inline(always)]
    pub fn ot_int_en(&self) -> OtIntEnR {
        OtIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - WDT overtime will generate a reset
    #[inline(always)]
    pub fn ot_rst_en(&self) -> OtRstEnR {
        OtRstEnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Refresh unlock fail will trigger a interrupt
    #[inline(always)]
    pub fn unl_ref_fail_int_en(&self) -> UnlRefFailIntEnR {
        UnlRefFailIntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Refresh unlock fail will trigger a reset
    #[inline(always)]
    pub fn unl_ref_fail_rst_en(&self) -> UnlRefFailRstEnR {
        UnlRefFailRstEnR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Refresh violation will trigger an interrupt
    #[inline(always)]
    pub fn ref_fail_int_en(&self) -> RefFailIntEnR {
        RefFailIntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt.
    #[inline(always)]
    pub fn ref_fail_rst_en(&self) -> RefFailRstEnR {
        RefFailRstEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Parity error will trigger a interrupt
    #[inline(always)]
    pub fn parity_fail_int_en(&mut self) -> ParityFailIntEnW<'_, Ctrl1Spec> {
        ParityFailIntEnW::new(self, 2)
    }
    ///Bit 3 - Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits
    #[inline(always)]
    pub fn parity_fail_rst_en(&mut self) -> ParityFailRstEnW<'_, Ctrl1Spec> {
        ParityFailRstEnW::new(self, 3)
    }
    ///Bit 4 - Unlock register update failure will trigger a interrupt
    #[inline(always)]
    pub fn unl_ctl_fail_int_en(&mut self) -> UnlCtlFailIntEnW<'_, Ctrl1Spec> {
        UnlCtlFailIntEnW::new(self, 4)
    }
    ///Bit 5 - Unlock register update failure will trigger a reset
    #[inline(always)]
    pub fn unl_ctl_fail_rst_en(&mut self) -> UnlCtlFailRstEnW<'_, Ctrl1Spec> {
        UnlCtlFailRstEnW::new(self, 5)
    }
    ///Bit 6 - Ctrl update violation will trigger a interrupt
    #[inline(always)]
    pub fn ctl_vio_int_en(&mut self) -> CtlVioIntEnW<'_, Ctrl1Spec> {
        CtlVioIntEnW::new(self, 6)
    }
    ///Bit 7 - Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them
    #[inline(always)]
    pub fn ctl_vio_rst_en(&mut self) -> CtlVioRstEnW<'_, Ctrl1Spec> {
        CtlVioRstEnW::new(self, 7)
    }
    ///Bit 16 - WDT can generate an interrupt warning before timeout
    #[inline(always)]
    pub fn ot_int_en(&mut self) -> OtIntEnW<'_, Ctrl1Spec> {
        OtIntEnW::new(self, 16)
    }
    ///Bit 17 - WDT overtime will generate a reset
    #[inline(always)]
    pub fn ot_rst_en(&mut self) -> OtRstEnW<'_, Ctrl1Spec> {
        OtRstEnW::new(self, 17)
    }
    ///Bit 20 - Refresh unlock fail will trigger a interrupt
    #[inline(always)]
    pub fn unl_ref_fail_int_en(&mut self) -> UnlRefFailIntEnW<'_, Ctrl1Spec> {
        UnlRefFailIntEnW::new(self, 20)
    }
    ///Bit 21 - Refresh unlock fail will trigger a reset
    #[inline(always)]
    pub fn unl_ref_fail_rst_en(&mut self) -> UnlRefFailRstEnW<'_, Ctrl1Spec> {
        UnlRefFailRstEnW::new(self, 21)
    }
    ///Bit 22 - Refresh violation will trigger an interrupt
    #[inline(always)]
    pub fn ref_fail_int_en(&mut self) -> RefFailIntEnW<'_, Ctrl1Spec> {
        RefFailIntEnW::new(self, 22)
    }
    ///Bit 23 - Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt.
    #[inline(always)]
    pub fn ref_fail_rst_en(&mut self) -> RefFailRstEnW<'_, Ctrl1Spec> {
        RefFailRstEnW::new(self, 23)
    }
}
/**wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits

You can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
///`read()` method returns [`ctrl1::R`](R) reader structure
impl crate::Readable for Ctrl1Spec {}
///`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTRL1 to value 0
impl crate::Resettable for Ctrl1Spec {}
