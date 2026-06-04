///Register `DCDC_RESUME_TIME` reader
pub type R = crate::R<DcdcResumeTimeSpec>;
///Register `DCDC_RESUME_TIME` writer
pub type W = crate::W<DcdcResumeTimeSpec>;
///Field `RESUME_TIME` reader - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS
pub type ResumeTimeR = crate::FieldReader<u32>;
///Field `RESUME_TIME` writer - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS
pub type ResumeTimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS
    #[inline(always)]
    pub fn resume_time(&self) -> ResumeTimeR {
        ResumeTimeR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS
    #[inline(always)]
    pub fn resume_time(&mut self) -> ResumeTimeW<'_, DcdcResumeTimeSpec> {
        ResumeTimeW::new(self, 0)
    }
}
/**DCDC resume time

You can [`read`](crate::Reg::read) this register and get [`dcdc_resume_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_resume_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcResumeTimeSpec;
impl crate::RegisterSpec for DcdcResumeTimeSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_resume_time::R`](R) reader structure
impl crate::Readable for DcdcResumeTimeSpec {}
///`write(|w| ..)` method takes [`dcdc_resume_time::W`](W) writer structure
impl crate::Writable for DcdcResumeTimeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_RESUME_TIME to value 0x8c9f
impl crate::Resettable for DcdcResumeTimeSpec {
    const RESET_VALUE: u32 = 0x8c9f;
}
