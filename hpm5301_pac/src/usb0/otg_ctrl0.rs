///Register `OTG_CTRL0` reader
pub type R = crate::R<OtgCtrl0Spec>;
///Register `OTG_CTRL0` writer
pub type W = crate::W<OtgCtrl0Spec>;
///Field `SER_MODE_SUSPEND_EN` reader - for naneng usbphy, only switch to serial mode when suspend
pub type SerModeSuspendEnR = crate::BitReader;
///Field `SER_MODE_SUSPEND_EN` writer - for naneng usbphy, only switch to serial mode when suspend
pub type SerModeSuspendEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_OVER_CUR_DIS` reader - No description available
pub type OtgOverCurDisR = crate::BitReader;
///Field `OTG_OVER_CUR_DIS` writer - No description available
pub type OtgOverCurDisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_OVER_CUR_POL` reader - No description available
pub type OtgOverCurPolR = crate::BitReader;
///Field `OTG_OVER_CUR_POL` writer - No description available
pub type OtgOverCurPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_POWER_MASK` reader - No description available
pub type OtgPowerMaskR = crate::BitReader;
///Field `OTG_POWER_MASK` writer - No description available
pub type OtgPowerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_WAKEUP_INT_ENABLE` reader - No description available
pub type OtgWakeupIntEnableR = crate::BitReader;
///Field `OTG_WAKEUP_INT_ENABLE` writer - No description available
pub type OtgWakeupIntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_UTMI_RESET_SW` reader - default 1 for naneng usbphy
pub type OtgUtmiResetSwR = crate::BitReader;
///Field `OTG_UTMI_RESET_SW` writer - default 1 for naneng usbphy
pub type OtgUtmiResetSwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_UTMI_SUSPENDM_SW` reader - default 0 for naneng usbphy
pub type OtgUtmiSuspendmSwR = crate::BitReader;
///Field `OTG_UTMI_SUSPENDM_SW` writer - default 0 for naneng usbphy
pub type OtgUtmiSuspendmSwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_VBUS_SOURCE_SEL` reader - No description available
pub type OtgVbusSourceSelR = crate::BitReader;
///Field `OTG_VBUS_SOURCE_SEL` writer - No description available
pub type OtgVbusSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_ID_WAKEUP_EN` reader - No description available
pub type OtgIdWakeupEnR = crate::BitReader;
///Field `OTG_ID_WAKEUP_EN` writer - No description available
pub type OtgIdWakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_VBUS_WAKEUP_EN` reader - No description available
pub type OtgVbusWakeupEnR = crate::BitReader;
///Field `OTG_VBUS_WAKEUP_EN` writer - No description available
pub type OtgVbusWakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTORESUME_EN` reader - No description available
pub type AutoresumeEnR = crate::BitReader;
///Field `AUTORESUME_EN` writer - No description available
pub type AutoresumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG_WKDPDMCHG_EN` reader - No description available
pub type OtgWkdpdmchgEnR = crate::BitReader;
///Field `OTG_WKDPDMCHG_EN` writer - No description available
pub type OtgWkdpdmchgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - for naneng usbphy, only switch to serial mode when suspend
    #[inline(always)]
    pub fn ser_mode_suspend_en(&self) -> SerModeSuspendEnR {
        SerModeSuspendEnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - No description available
    #[inline(always)]
    pub fn otg_over_cur_dis(&self) -> OtgOverCurDisR {
        OtgOverCurDisR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - No description available
    #[inline(always)]
    pub fn otg_over_cur_pol(&self) -> OtgOverCurPolR {
        OtgOverCurPolR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - No description available
    #[inline(always)]
    pub fn otg_power_mask(&self) -> OtgPowerMaskR {
        OtgPowerMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - No description available
    #[inline(always)]
    pub fn otg_wakeup_int_enable(&self) -> OtgWakeupIntEnableR {
        OtgWakeupIntEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - default 1 for naneng usbphy
    #[inline(always)]
    pub fn otg_utmi_reset_sw(&self) -> OtgUtmiResetSwR {
        OtgUtmiResetSwR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - default 0 for naneng usbphy
    #[inline(always)]
    pub fn otg_utmi_suspendm_sw(&self) -> OtgUtmiSuspendmSwR {
        OtgUtmiSuspendmSwR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - No description available
    #[inline(always)]
    pub fn otg_vbus_source_sel(&self) -> OtgVbusSourceSelR {
        OtgVbusSourceSelR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - No description available
    #[inline(always)]
    pub fn otg_id_wakeup_en(&self) -> OtgIdWakeupEnR {
        OtgIdWakeupEnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - No description available
    #[inline(always)]
    pub fn otg_vbus_wakeup_en(&self) -> OtgVbusWakeupEnR {
        OtgVbusWakeupEnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - No description available
    #[inline(always)]
    pub fn autoresume_en(&self) -> AutoresumeEnR {
        AutoresumeEnR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 25 - No description available
    #[inline(always)]
    pub fn otg_wkdpdmchg_en(&self) -> OtgWkdpdmchgEnR {
        OtgWkdpdmchgEnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - for naneng usbphy, only switch to serial mode when suspend
    #[inline(always)]
    pub fn ser_mode_suspend_en(&mut self) -> SerModeSuspendEnW<'_, OtgCtrl0Spec> {
        SerModeSuspendEnW::new(self, 4)
    }
    ///Bit 7 - No description available
    #[inline(always)]
    pub fn otg_over_cur_dis(&mut self) -> OtgOverCurDisW<'_, OtgCtrl0Spec> {
        OtgOverCurDisW::new(self, 7)
    }
    ///Bit 8 - No description available
    #[inline(always)]
    pub fn otg_over_cur_pol(&mut self) -> OtgOverCurPolW<'_, OtgCtrl0Spec> {
        OtgOverCurPolW::new(self, 8)
    }
    ///Bit 9 - No description available
    #[inline(always)]
    pub fn otg_power_mask(&mut self) -> OtgPowerMaskW<'_, OtgCtrl0Spec> {
        OtgPowerMaskW::new(self, 9)
    }
    ///Bit 10 - No description available
    #[inline(always)]
    pub fn otg_wakeup_int_enable(&mut self) -> OtgWakeupIntEnableW<'_, OtgCtrl0Spec> {
        OtgWakeupIntEnableW::new(self, 10)
    }
    ///Bit 11 - default 1 for naneng usbphy
    #[inline(always)]
    pub fn otg_utmi_reset_sw(&mut self) -> OtgUtmiResetSwW<'_, OtgCtrl0Spec> {
        OtgUtmiResetSwW::new(self, 11)
    }
    ///Bit 12 - default 0 for naneng usbphy
    #[inline(always)]
    pub fn otg_utmi_suspendm_sw(&mut self) -> OtgUtmiSuspendmSwW<'_, OtgCtrl0Spec> {
        OtgUtmiSuspendmSwW::new(self, 12)
    }
    ///Bit 13 - No description available
    #[inline(always)]
    pub fn otg_vbus_source_sel(&mut self) -> OtgVbusSourceSelW<'_, OtgCtrl0Spec> {
        OtgVbusSourceSelW::new(self, 13)
    }
    ///Bit 16 - No description available
    #[inline(always)]
    pub fn otg_id_wakeup_en(&mut self) -> OtgIdWakeupEnW<'_, OtgCtrl0Spec> {
        OtgIdWakeupEnW::new(self, 16)
    }
    ///Bit 17 - No description available
    #[inline(always)]
    pub fn otg_vbus_wakeup_en(&mut self) -> OtgVbusWakeupEnW<'_, OtgCtrl0Spec> {
        OtgVbusWakeupEnW::new(self, 17)
    }
    ///Bit 19 - No description available
    #[inline(always)]
    pub fn autoresume_en(&mut self) -> AutoresumeEnW<'_, OtgCtrl0Spec> {
        AutoresumeEnW::new(self, 19)
    }
    ///Bit 25 - No description available
    #[inline(always)]
    pub fn otg_wkdpdmchg_en(&mut self) -> OtgWkdpdmchgEnW<'_, OtgCtrl0Spec> {
        OtgWkdpdmchgEnW::new(self, 25)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`otg_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OtgCtrl0Spec;
impl crate::RegisterSpec for OtgCtrl0Spec {
    type Ux = u32;
}
///`read()` method returns [`otg_ctrl0::R`](R) reader structure
impl crate::Readable for OtgCtrl0Spec {}
///`write(|w| ..)` method takes [`otg_ctrl0::W`](W) writer structure
impl crate::Writable for OtgCtrl0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_CTRL0 to value 0
impl crate::Resettable for OtgCtrl0Spec {}
