///Register `SS_STEP` reader
pub type R = crate::R<SsStepSpec>;
///Register `SS_STEP` writer
pub type W = crate::W<SsStepSpec>;
///Field `STEP` reader - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
pub type StepR = crate::FieldReader<u32>;
///Field `STEP` writer - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 0:29 - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
    #[inline(always)]
    pub fn step(&mut self) -> StepW<'_, SsStepSpec> {
        StepW::new(self, 0)
    }
}
/**PLL0 spread spectrum step register

You can [`read`](crate::Reg::read) this register and get [`ss_step::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_step::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsStepSpec;
impl crate::RegisterSpec for SsStepSpec {
    type Ux = u32;
}
///`read()` method returns [`ss_step::R`](R) reader structure
impl crate::Readable for SsStepSpec {}
///`write(|w| ..)` method takes [`ss_step::W`](W) writer structure
impl crate::Writable for SsStepSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SS_STEP to value 0
impl crate::Resettable for SsStepSpec {}
