///Register `DGO_CTR4` reader
pub type R = crate::R<DgoCtr4Spec>;
///Register `DGO_CTR4` writer
pub type W = crate::W<DgoCtr4Spec>;
///Field `BANDGAP_LP_MODE` reader - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode
pub type BandgapLpModeR = crate::BitReader;
///Field `BANDGAP_LP_MODE` writer - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode
pub type BandgapLpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BANDGAP_LESS_POWER` reader - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode
pub type BandgapLessPowerR = crate::BitReader;
///Field `BANDGAP_LESS_POWER` writer - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode
pub type BandgapLessPowerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode
    #[inline(always)]
    pub fn bandgap_lp_mode(&self) -> BandgapLpModeR {
        BandgapLpModeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode
    #[inline(always)]
    pub fn bandgap_less_power(&self) -> BandgapLessPowerR {
        BandgapLessPowerR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode
    #[inline(always)]
    pub fn bandgap_lp_mode(&mut self) -> BandgapLpModeW<'_, DgoCtr4Spec> {
        BandgapLpModeW::new(self, 0)
    }
    ///Bit 1 - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode
    #[inline(always)]
    pub fn bandgap_less_power(&mut self) -> BandgapLessPowerW<'_, DgoCtr4Spec> {
        BandgapLessPowerW::new(self, 1)
    }
}
/**control register 4

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoCtr4Spec;
impl crate::RegisterSpec for DgoCtr4Spec {
    type Ux = u32;
}
///`read()` method returns [`dgo_ctr4::R`](R) reader structure
impl crate::Readable for DgoCtr4Spec {}
///`write(|w| ..)` method takes [`dgo_ctr4::W`](W) writer structure
impl crate::Writable for DgoCtr4Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_CTR4 to value 0
impl crate::Resettable for DgoCtr4Spec {}
