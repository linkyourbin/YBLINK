///Register `SS_STOP` reader
pub type R = crate::R<SsStopSpec>;
///Register `SS_STOP` writer
pub type W = crate::W<SsStopSpec>;
///Field `STOP` reader - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
pub type StopR = crate::FieldReader<u32>;
///Field `STOP` writer - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 0:29 - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, SsStopSpec> {
        StopW::new(self, 0)
    }
}
/**PLL0 spread spectrum stop register

You can [`read`](crate::Reg::read) this register and get [`ss_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsStopSpec;
impl crate::RegisterSpec for SsStopSpec {
    type Ux = u32;
}
///`read()` method returns [`ss_stop::R`](R) reader structure
impl crate::Readable for SsStopSpec {}
///`write(|w| ..)` method takes [`ss_stop::W`](W) writer structure
impl crate::Writable for SsStopSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SS_STOP to value 0
impl crate::Resettable for SsStopSpec {}
