///Register `feature` reader
pub type R = crate::R<FeatureSpec>;
///Register `feature` writer
pub type W = crate::W<FeatureSpec>;
///Field `PREEMPT` reader - Preemptive priority interrupt enable 0: Disabled 1: Enabled
pub type PreemptR = crate::BitReader;
///Field `PREEMPT` writer - Preemptive priority interrupt enable 0: Disabled 1: Enabled
pub type PreemptW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VECTORED` reader - Vector mode enable 0: Disabled 1: Enabled
pub type VectoredR = crate::BitReader;
///Field `VECTORED` writer - Vector mode enable 0: Disabled 1: Enabled
pub type VectoredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Preemptive priority interrupt enable 0: Disabled 1: Enabled
    #[inline(always)]
    pub fn preempt(&self) -> PreemptR {
        PreemptR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Vector mode enable 0: Disabled 1: Enabled
    #[inline(always)]
    pub fn vectored(&self) -> VectoredR {
        VectoredR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Preemptive priority interrupt enable 0: Disabled 1: Enabled
    #[inline(always)]
    pub fn preempt(&mut self) -> PreemptW<'_, FeatureSpec> {
        PreemptW::new(self, 0)
    }
    ///Bit 1 - Vector mode enable 0: Disabled 1: Enabled
    #[inline(always)]
    pub fn vectored(&mut self) -> VectoredW<'_, FeatureSpec> {
        VectoredW::new(self, 1)
    }
}
/**Feature enable register

You can [`read`](crate::Reg::read) this register and get [`feature::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FeatureSpec;
impl crate::RegisterSpec for FeatureSpec {
    type Ux = u32;
}
///`read()` method returns [`feature::R`](R) reader structure
impl crate::Readable for FeatureSpec {}
///`write(|w| ..)` method takes [`feature::W`](W) writer structure
impl crate::Writable for FeatureSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets feature to value 0
impl crate::Resettable for FeatureSpec {}
