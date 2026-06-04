///Register `PHY_STATUS` reader
pub type R = crate::R<PhyStatusSpec>;
///Register `PHY_STATUS` writer
pub type W = crate::W<PhyStatusSpec>;
///Field `VBUS_VALID` reader - No description available
pub type VbusValidR = crate::BitReader;
///Field `VBUS_VALID` writer - No description available
pub type VbusValidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UTMI_SESS_VALID` reader - No description available
pub type UtmiSessValidR = crate::BitReader;
///Field `UTMI_SESS_VALID` writer - No description available
pub type UtmiSessValidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ID_DIG` reader - No description available
pub type IdDigR = crate::BitReader;
///Field `ID_DIG` writer - No description available
pub type IdDigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST_DISCONNECT` reader - No description available
pub type HostDisconnectR = crate::BitReader;
///Field `HOST_DISCONNECT` writer - No description available
pub type HostDisconnectW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_STATE` reader - No description available
pub type LineStateR = crate::FieldReader;
///Field `LINE_STATE` writer - No description available
pub type LineStateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UTMI_CLK_VALID` reader - No description available
pub type UtmiClkValidR = crate::BitReader;
///Field `UTMI_CLK_VALID` writer - No description available
pub type UtmiClkValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - No description available
    #[inline(always)]
    pub fn vbus_valid(&self) -> VbusValidR {
        VbusValidR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - No description available
    #[inline(always)]
    pub fn utmi_sess_valid(&self) -> UtmiSessValidR {
        UtmiSessValidR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - No description available
    #[inline(always)]
    pub fn id_dig(&self) -> IdDigR {
        IdDigR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - No description available
    #[inline(always)]
    pub fn host_disconnect(&self) -> HostDisconnectR {
        HostDisconnectR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - No description available
    #[inline(always)]
    pub fn line_state(&self) -> LineStateR {
        LineStateR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 31 - No description available
    #[inline(always)]
    pub fn utmi_clk_valid(&self) -> UtmiClkValidR {
        UtmiClkValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - No description available
    #[inline(always)]
    pub fn vbus_valid(&mut self) -> VbusValidW<'_, PhyStatusSpec> {
        VbusValidW::new(self, 0)
    }
    ///Bit 2 - No description available
    #[inline(always)]
    pub fn utmi_sess_valid(&mut self) -> UtmiSessValidW<'_, PhyStatusSpec> {
        UtmiSessValidW::new(self, 2)
    }
    ///Bit 4 - No description available
    #[inline(always)]
    pub fn id_dig(&mut self) -> IdDigW<'_, PhyStatusSpec> {
        IdDigW::new(self, 4)
    }
    ///Bit 5 - No description available
    #[inline(always)]
    pub fn host_disconnect(&mut self) -> HostDisconnectW<'_, PhyStatusSpec> {
        HostDisconnectW::new(self, 5)
    }
    ///Bits 6:7 - No description available
    #[inline(always)]
    pub fn line_state(&mut self) -> LineStateW<'_, PhyStatusSpec> {
        LineStateW::new(self, 6)
    }
    ///Bit 31 - No description available
    #[inline(always)]
    pub fn utmi_clk_valid(&mut self) -> UtmiClkValidW<'_, PhyStatusSpec> {
        UtmiClkValidW::new(self, 31)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`phy_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PhyStatusSpec;
impl crate::RegisterSpec for PhyStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`phy_status::R`](R) reader structure
impl crate::Readable for PhyStatusSpec {}
///`write(|w| ..)` method takes [`phy_status::W`](W) writer structure
impl crate::Writable for PhyStatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHY_STATUS to value 0
impl crate::Resettable for PhyStatusSpec {}
