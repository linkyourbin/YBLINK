///Register `DGO_RC32K_CFG` reader
pub type R = crate::R<DgoRc32kCfgSpec>;
///Register `DGO_RC32K_CFG` writer
pub type W = crate::W<DgoRc32kCfgSpec>;
///Field `CAP_TRIM` reader - capacitor trim bits
pub type CapTrimR = crate::FieldReader<u16>;
///Field `CAP_TRIM` writer - capacitor trim bits
pub type CapTrimW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CAPEX6_TRIM` reader - IRC32K bit 6
pub type Capex6TrimR = crate::BitReader;
///Field `CAPEX6_TRIM` writer - IRC32K bit 6
pub type Capex6TrimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAPEX7_TRIM` reader - IRC32K bit 7
pub type Capex7TrimR = crate::BitReader;
///Field `CAPEX7_TRIM` writer - IRC32K bit 7
pub type Capex7TrimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRC_TRIMMED` reader - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed
pub type IrcTrimmedR = crate::BitReader;
///Field `IRC_TRIMMED` writer - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed
pub type IrcTrimmedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - capacitor trim bits
    #[inline(always)]
    pub fn cap_trim(&self) -> CapTrimR {
        CapTrimR::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 22 - IRC32K bit 6
    #[inline(always)]
    pub fn capex6_trim(&self) -> Capex6TrimR {
        Capex6TrimR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - IRC32K bit 7
    #[inline(always)]
    pub fn capex7_trim(&self) -> Capex7TrimR {
        Capex7TrimR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed
    #[inline(always)]
    pub fn irc_trimmed(&self) -> IrcTrimmedR {
        IrcTrimmedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - capacitor trim bits
    #[inline(always)]
    pub fn cap_trim(&mut self) -> CapTrimW<'_, DgoRc32kCfgSpec> {
        CapTrimW::new(self, 0)
    }
    ///Bit 22 - IRC32K bit 6
    #[inline(always)]
    pub fn capex6_trim(&mut self) -> Capex6TrimW<'_, DgoRc32kCfgSpec> {
        Capex6TrimW::new(self, 22)
    }
    ///Bit 23 - IRC32K bit 7
    #[inline(always)]
    pub fn capex7_trim(&mut self) -> Capex7TrimW<'_, DgoRc32kCfgSpec> {
        Capex7TrimW::new(self, 23)
    }
    ///Bit 31 - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed
    #[inline(always)]
    pub fn irc_trimmed(&mut self) -> IrcTrimmedW<'_, DgoRc32kCfgSpec> {
        IrcTrimmedW::new(self, 31)
    }
}
/**RC32K CLOCK

You can [`read`](crate::Reg::read) this register and get [`dgo_rc32k_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_rc32k_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoRc32kCfgSpec;
impl crate::RegisterSpec for DgoRc32kCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`dgo_rc32k_cfg::R`](R) reader structure
impl crate::Readable for DgoRc32kCfgSpec {}
///`write(|w| ..)` method takes [`dgo_rc32k_cfg::W`](W) writer structure
impl crate::Writable for DgoRc32kCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_RC32K_CFG to value 0
impl crate::Resettable for DgoRc32kCfgSpec {}
