///Register `ADCCLK[%s]` reader
pub type R = crate::R<AdcclkSpec>;
///Register `ADCCLK[%s]` writer
pub type W = crate::W<AdcclkSpec>;
///Field `MUX` reader - current mux 0: ahb0 clock 1: ana clock N
pub type MuxR = crate::BitReader;
///Field `MUX` writer - current mux 0: ahb0 clock 1: ana clock N
pub type MuxW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESERVE` reader - preserve function against global select 0: select global clock setting 1: not select global clock setting
pub type PreserveR = crate::BitReader;
///Field `PRESERVE` writer - preserve function against global select 0: select global clock setting 1: not select global clock setting
pub type PreserveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOC_BUSY` reader - local busy 0: a change is pending for current node 1: current node is changing status
pub type LocBusyR = crate::BitReader;
///Field `GLB_BUSY` reader - global busy 0: no changes pending to any clock 1: any of nodes is changing status
pub type GlbBusyR = crate::BitReader;
impl R {
    ///Bit 8 - current mux 0: ahb0 clock 1: ana clock N
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 28 - preserve function against global select 0: select global clock setting 1: not select global clock setting
    #[inline(always)]
    pub fn preserve(&self) -> PreserveR {
        PreserveR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - local busy 0: a change is pending for current node 1: current node is changing status
    #[inline(always)]
    pub fn loc_busy(&self) -> LocBusyR {
        LocBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - global busy 0: no changes pending to any clock 1: any of nodes is changing status
    #[inline(always)]
    pub fn glb_busy(&self) -> GlbBusyR {
        GlbBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - current mux 0: ahb0 clock 1: ana clock N
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<'_, AdcclkSpec> {
        MuxW::new(self, 8)
    }
    ///Bit 28 - preserve function against global select 0: select global clock setting 1: not select global clock setting
    #[inline(always)]
    pub fn preserve(&mut self) -> PreserveW<'_, AdcclkSpec> {
        PreserveW::new(self, 28)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`adcclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcclkSpec;
impl crate::RegisterSpec for AdcclkSpec {
    type Ux = u32;
}
///`read()` method returns [`adcclk::R`](R) reader structure
impl crate::Readable for AdcclkSpec {}
///`write(|w| ..)` method takes [`adcclk::W`](W) writer structure
impl crate::Writable for AdcclkSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCCLK[%s] to value 0
impl crate::Resettable for AdcclkSpec {}
