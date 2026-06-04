///Register `DGO_CTR1` reader
pub type R = crate::R<DgoCtr1Spec>;
///Register `DGO_CTR1` writer
pub type W = crate::W<DgoCtr1Spec>;
///Field `PIN_WAKEUP_STATUS` reader - wakeup pin status
pub type PinWakeupStatusR = crate::BitReader;
///Field `WAKEUP_EN` reader - permit wakeup pin or software wakeup
pub type WakeupEnR = crate::BitReader;
///Field `WAKEUP_EN` writer - permit wakeup pin or software wakeup
pub type WakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AOTO_SYS_WAKEUP` reader - software wakeup： 0 : wakeup once； 1：auto wakeup Continuously
pub type AotoSysWakeupR = crate::BitReader;
///Field `AOTO_SYS_WAKEUP` writer - software wakeup： 0 : wakeup once； 1：auto wakeup Continuously
pub type AotoSysWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - wakeup pin status
    #[inline(always)]
    pub fn pin_wakeup_status(&self) -> PinWakeupStatusR {
        PinWakeupStatusR::new((self.bits & 1) != 0)
    }
    ///Bit 16 - permit wakeup pin or software wakeup
    #[inline(always)]
    pub fn wakeup_en(&self) -> WakeupEnR {
        WakeupEnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - software wakeup： 0 : wakeup once； 1：auto wakeup Continuously
    #[inline(always)]
    pub fn aoto_sys_wakeup(&self) -> AotoSysWakeupR {
        AotoSysWakeupR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - permit wakeup pin or software wakeup
    #[inline(always)]
    pub fn wakeup_en(&mut self) -> WakeupEnW<'_, DgoCtr1Spec> {
        WakeupEnW::new(self, 16)
    }
    ///Bit 31 - software wakeup： 0 : wakeup once； 1：auto wakeup Continuously
    #[inline(always)]
    pub fn aoto_sys_wakeup(&mut self) -> AotoSysWakeupW<'_, DgoCtr1Spec> {
        AotoSysWakeupW::new(self, 31)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoCtr1Spec;
impl crate::RegisterSpec for DgoCtr1Spec {
    type Ux = u32;
}
///`read()` method returns [`dgo_ctr1::R`](R) reader structure
impl crate::Readable for DgoCtr1Spec {}
///`write(|w| ..)` method takes [`dgo_ctr1::W`](W) writer structure
impl crate::Writable for DgoCtr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_CTR1 to value 0
impl crate::Resettable for DgoCtr1Spec {}
