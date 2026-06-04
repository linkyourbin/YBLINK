///Register `PHY_CTRL0` reader
pub type R = crate::R<PhyCtrl0Spec>;
///Register `PHY_CTRL0` writer
pub type W = crate::W<PhyCtrl0Spec>;
///Field `VBUS_VALID_OVERRIDE_EN` reader - No description available
pub type VbusValidOverrideEnR = crate::BitReader;
///Field `VBUS_VALID_OVERRIDE_EN` writer - No description available
pub type VbusValidOverrideEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SESS_VALID_OVERRIDE_EN` reader - No description available
pub type SessValidOverrideEnR = crate::BitReader;
///Field `SESS_VALID_OVERRIDE_EN` writer - No description available
pub type SessValidOverrideEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ID_DIG_OVERRIDE_EN` reader - No description available
pub type IdDigOverrideEnR = crate::BitReader;
///Field `ID_DIG_OVERRIDE_EN` writer - No description available
pub type IdDigOverrideEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP_MODE_SUSPENDM_ENJ` reader - set op_mode to 2'b01 in suspend, for naneng usbphy
pub type OpModeSuspendmEnjR = crate::BitReader;
///Field `OP_MODE_SUSPENDM_ENJ` writer - set op_mode to 2'b01 in suspend, for naneng usbphy
pub type OpModeSuspendmEnjW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBUS_VALID_OVERRIDE` reader - No description available
pub type VbusValidOverrideR = crate::BitReader;
///Field `VBUS_VALID_OVERRIDE` writer - No description available
pub type VbusValidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SESS_VALID_OVERRIDE` reader - No description available
pub type SessValidOverrideR = crate::BitReader;
///Field `SESS_VALID_OVERRIDE` writer - No description available
pub type SessValidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ID_DIG_OVERRIDE` reader - No description available
pub type IdDigOverrideR = crate::BitReader;
///Field `ID_DIG_OVERRIDE` writer - No description available
pub type IdDigOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_ID_SEL_N` reader - No description available
pub type GpioIdSelNR = crate::BitReader;
///Field `GPIO_ID_SEL_N` writer - No description available
pub type GpioIdSelNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - No description available
    #[inline(always)]
    pub fn vbus_valid_override_en(&self) -> VbusValidOverrideEnR {
        VbusValidOverrideEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - No description available
    #[inline(always)]
    pub fn sess_valid_override_en(&self) -> SessValidOverrideEnR {
        SessValidOverrideEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - No description available
    #[inline(always)]
    pub fn id_dig_override_en(&self) -> IdDigOverrideEnR {
        IdDigOverrideEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 11 - set op_mode to 2'b01 in suspend, for naneng usbphy
    #[inline(always)]
    pub fn op_mode_suspendm_enj(&self) -> OpModeSuspendmEnjR {
        OpModeSuspendmEnjR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - No description available
    #[inline(always)]
    pub fn vbus_valid_override(&self) -> VbusValidOverrideR {
        VbusValidOverrideR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - No description available
    #[inline(always)]
    pub fn sess_valid_override(&self) -> SessValidOverrideR {
        SessValidOverrideR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - No description available
    #[inline(always)]
    pub fn id_dig_override(&self) -> IdDigOverrideR {
        IdDigOverrideR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 25 - No description available
    #[inline(always)]
    pub fn gpio_id_sel_n(&self) -> GpioIdSelNR {
        GpioIdSelNR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - No description available
    #[inline(always)]
    pub fn vbus_valid_override_en(&mut self) -> VbusValidOverrideEnW<'_, PhyCtrl0Spec> {
        VbusValidOverrideEnW::new(self, 0)
    }
    ///Bit 1 - No description available
    #[inline(always)]
    pub fn sess_valid_override_en(&mut self) -> SessValidOverrideEnW<'_, PhyCtrl0Spec> {
        SessValidOverrideEnW::new(self, 1)
    }
    ///Bit 2 - No description available
    #[inline(always)]
    pub fn id_dig_override_en(&mut self) -> IdDigOverrideEnW<'_, PhyCtrl0Spec> {
        IdDigOverrideEnW::new(self, 2)
    }
    ///Bit 11 - set op_mode to 2'b01 in suspend, for naneng usbphy
    #[inline(always)]
    pub fn op_mode_suspendm_enj(&mut self) -> OpModeSuspendmEnjW<'_, PhyCtrl0Spec> {
        OpModeSuspendmEnjW::new(self, 11)
    }
    ///Bit 12 - No description available
    #[inline(always)]
    pub fn vbus_valid_override(&mut self) -> VbusValidOverrideW<'_, PhyCtrl0Spec> {
        VbusValidOverrideW::new(self, 12)
    }
    ///Bit 13 - No description available
    #[inline(always)]
    pub fn sess_valid_override(&mut self) -> SessValidOverrideW<'_, PhyCtrl0Spec> {
        SessValidOverrideW::new(self, 13)
    }
    ///Bit 14 - No description available
    #[inline(always)]
    pub fn id_dig_override(&mut self) -> IdDigOverrideW<'_, PhyCtrl0Spec> {
        IdDigOverrideW::new(self, 14)
    }
    ///Bit 25 - No description available
    #[inline(always)]
    pub fn gpio_id_sel_n(&mut self) -> GpioIdSelNW<'_, PhyCtrl0Spec> {
        GpioIdSelNW::new(self, 25)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`phy_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PhyCtrl0Spec;
impl crate::RegisterSpec for PhyCtrl0Spec {
    type Ux = u32;
}
///`read()` method returns [`phy_ctrl0::R`](R) reader structure
impl crate::Readable for PhyCtrl0Spec {}
///`write(|w| ..)` method takes [`phy_ctrl0::W`](W) writer structure
impl crate::Writable for PhyCtrl0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHY_CTRL0 to value 0
impl crate::Resettable for PhyCtrl0Spec {}
