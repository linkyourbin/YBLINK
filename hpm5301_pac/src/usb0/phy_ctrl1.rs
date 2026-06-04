///Register `PHY_CTRL1` reader
pub type R = crate::R<PhyCtrl1Spec>;
///Register `PHY_CTRL1` writer
pub type W = crate::W<PhyCtrl1Spec>;
///Field `UTMI_OTG_SUSPENDM` reader - OTG suspend, not utmi_suspendm. setting this bit also enable host high-speed disconnect detection logic(check DP/DM voltage at end of SOF, to determine whether 50Ohm are still there on DP/DM) should clear this bit before entering into suspend state(setting portsc.phcd), avoide unwanted glitch on vbus/sess_vld/sess_end
pub type UtmiOtgSuspendmR = crate::BitReader;
///Field `UTMI_OTG_SUSPENDM` writer - OTG suspend, not utmi_suspendm. setting this bit also enable host high-speed disconnect detection logic(check DP/DM voltage at end of SOF, to determine whether 50Ohm are still there on DP/DM) should clear this bit before entering into suspend state(setting portsc.phcd), avoide unwanted glitch on vbus/sess_vld/sess_end
pub type UtmiOtgSuspendmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UTMI_CFG_RST_N` reader - No description available
pub type UtmiCfgRstNR = crate::BitReader;
///Field `UTMI_CFG_RST_N` writer - No description available
pub type UtmiCfgRstNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - OTG suspend, not utmi_suspendm. setting this bit also enable host high-speed disconnect detection logic(check DP/DM voltage at end of SOF, to determine whether 50Ohm are still there on DP/DM) should clear this bit before entering into suspend state(setting portsc.phcd), avoide unwanted glitch on vbus/sess_vld/sess_end
    #[inline(always)]
    pub fn utmi_otg_suspendm(&self) -> UtmiOtgSuspendmR {
        UtmiOtgSuspendmR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 20 - No description available
    #[inline(always)]
    pub fn utmi_cfg_rst_n(&self) -> UtmiCfgRstNR {
        UtmiCfgRstNR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - OTG suspend, not utmi_suspendm. setting this bit also enable host high-speed disconnect detection logic(check DP/DM voltage at end of SOF, to determine whether 50Ohm are still there on DP/DM) should clear this bit before entering into suspend state(setting portsc.phcd), avoide unwanted glitch on vbus/sess_vld/sess_end
    #[inline(always)]
    pub fn utmi_otg_suspendm(&mut self) -> UtmiOtgSuspendmW<'_, PhyCtrl1Spec> {
        UtmiOtgSuspendmW::new(self, 1)
    }
    ///Bit 20 - No description available
    #[inline(always)]
    pub fn utmi_cfg_rst_n(&mut self) -> UtmiCfgRstNW<'_, PhyCtrl1Spec> {
        UtmiCfgRstNW::new(self, 20)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`phy_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PhyCtrl1Spec;
impl crate::RegisterSpec for PhyCtrl1Spec {
    type Ux = u32;
}
///`read()` method returns [`phy_ctrl1::R`](R) reader structure
impl crate::Readable for PhyCtrl1Spec {}
///`write(|w| ..)` method takes [`phy_ctrl1::W`](W) writer structure
impl crate::Writable for PhyCtrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHY_CTRL1 to value 0
impl crate::Resettable for PhyCtrl1Spec {}
