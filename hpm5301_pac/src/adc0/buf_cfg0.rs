///Register `buf_cfg0` reader
pub type R = crate::R<BufCfg0Spec>;
///Register `buf_cfg0` writer
pub type W = crate::W<BufCfg0Spec>;
///Field `WAIT_DIS` reader - set to disable read waiting, get result immediately but maybe not current conversion result.
pub type WaitDisR = crate::BitReader;
///Field `WAIT_DIS` writer - set to disable read waiting, get result immediately but maybe not current conversion result.
pub type WaitDisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_MODE_EN` reader - bus mode enable
pub type BusModeEnR = crate::BitReader;
///Field `BUS_MODE_EN` writer - bus mode enable
pub type BusModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - set to disable read waiting, get result immediately but maybe not current conversion result.
    #[inline(always)]
    pub fn wait_dis(&self) -> WaitDisR {
        WaitDisR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - bus mode enable
    #[inline(always)]
    pub fn bus_mode_en(&self) -> BusModeEnR {
        BusModeEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - set to disable read waiting, get result immediately but maybe not current conversion result.
    #[inline(always)]
    pub fn wait_dis(&mut self) -> WaitDisW<'_, BufCfg0Spec> {
        WaitDisW::new(self, 0)
    }
    ///Bit 1 - bus mode enable
    #[inline(always)]
    pub fn bus_mode_en(&mut self) -> BusModeEnW<'_, BufCfg0Spec> {
        BusModeEnW::new(self, 1)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`buf_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BufCfg0Spec;
impl crate::RegisterSpec for BufCfg0Spec {
    type Ux = u32;
}
///`read()` method returns [`buf_cfg0::R`](R) reader structure
impl crate::Readable for BufCfg0Spec {}
///`write(|w| ..)` method takes [`buf_cfg0::W`](W) writer structure
impl crate::Writable for BufCfg0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets buf_cfg0 to value 0
impl crate::Resettable for BufCfg0Spec {}
