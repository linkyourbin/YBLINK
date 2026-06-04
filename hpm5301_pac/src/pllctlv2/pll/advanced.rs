///Register `ADVANCED` reader
pub type R = crate::R<AdvancedSpec>;
///Register `ADVANCED` writer
pub type W = crate::W<AdvancedSpec>;
///Field `DITHER` reader - Enable dither function
pub type DitherR = crate::BitReader;
///Field `DITHER` writer - Enable dither function
pub type DitherW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOW` reader - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us
pub type SlowR = crate::BitReader;
///Field `SLOW` writer - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us
pub type SlowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24 - Enable dither function
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us
    #[inline(always)]
    pub fn slow(&self) -> SlowR {
        SlowR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - Enable dither function
    #[inline(always)]
    pub fn dither(&mut self) -> DitherW<'_, AdvancedSpec> {
        DitherW::new(self, 24)
    }
    ///Bit 28 - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us
    #[inline(always)]
    pub fn slow(&mut self) -> SlowW<'_, AdvancedSpec> {
        SlowW::new(self, 28)
    }
}
/**PLL0 advance configuration register

You can [`read`](crate::Reg::read) this register and get [`advanced::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`advanced::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdvancedSpec;
impl crate::RegisterSpec for AdvancedSpec {
    type Ux = u32;
}
///`read()` method returns [`advanced::R`](R) reader structure
impl crate::Readable for AdvancedSpec {}
///`write(|w| ..)` method takes [`advanced::W`](W) writer structure
impl crate::Writable for AdvancedSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADVANCED to value 0
impl crate::Resettable for AdvancedSpec {}
