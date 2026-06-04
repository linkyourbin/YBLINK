///Register `DGO_CTR2` reader
pub type R = crate::R<DgoCtr2Spec>;
///Register `DGO_CTR2` writer
pub type W = crate::W<DgoCtr2Spec>;
///Field `WAKEUP_PULLDN_DISABLE` reader - wakeup pin pull down disable
pub type WakeupPulldnDisableR = crate::BitReader;
///Field `WAKEUP_PULLDN_DISABLE` writer - wakeup pin pull down disable
pub type WakeupPulldnDisableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESETN_PULLUP_DISABLE` reader - resetn pin pull up disable
pub type ResetnPullupDisableR = crate::BitReader;
///Field `RESETN_PULLUP_DISABLE` writer - resetn pin pull up disable
pub type ResetnPullupDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - wakeup pin pull down disable
    #[inline(always)]
    pub fn wakeup_pulldn_disable(&self) -> WakeupPulldnDisableR {
        WakeupPulldnDisableR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - resetn pin pull up disable
    #[inline(always)]
    pub fn resetn_pullup_disable(&self) -> ResetnPullupDisableR {
        ResetnPullupDisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - wakeup pin pull down disable
    #[inline(always)]
    pub fn wakeup_pulldn_disable(&mut self) -> WakeupPulldnDisableW<'_, DgoCtr2Spec> {
        WakeupPulldnDisableW::new(self, 16)
    }
    ///Bit 24 - resetn pin pull up disable
    #[inline(always)]
    pub fn resetn_pullup_disable(&mut self) -> ResetnPullupDisableW<'_, DgoCtr2Spec> {
        ResetnPullupDisableW::new(self, 24)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoCtr2Spec;
impl crate::RegisterSpec for DgoCtr2Spec {
    type Ux = u32;
}
///`read()` method returns [`dgo_ctr2::R`](R) reader structure
impl crate::Readable for DgoCtr2Spec {}
///`write(|w| ..)` method takes [`dgo_ctr2::W`](W) writer structure
impl crate::Writable for DgoCtr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_CTR2 to value 0
impl crate::Resettable for DgoCtr2Spec {}
