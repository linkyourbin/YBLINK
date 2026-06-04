///Register `DCDC_START_TIME` reader
pub type R = crate::R<DcdcStartTimeSpec>;
///Register `DCDC_START_TIME` writer
pub type W = crate::W<DcdcStartTimeSpec>;
///Field `START_TIME` reader - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS
pub type StartTimeR = crate::FieldReader<u32>;
///Field `START_TIME` writer - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS
pub type StartTimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS
    #[inline(always)]
    pub fn start_time(&self) -> StartTimeR {
        StartTimeR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS
    #[inline(always)]
    pub fn start_time(&mut self) -> StartTimeW<'_, DcdcStartTimeSpec> {
        StartTimeW::new(self, 0)
    }
}
/**DCDC ramp time

You can [`read`](crate::Reg::read) this register and get [`dcdc_start_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_start_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcStartTimeSpec;
impl crate::RegisterSpec for DcdcStartTimeSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_start_time::R`](R) reader structure
impl crate::Readable for DcdcStartTimeSpec {}
///`write(|w| ..)` method takes [`dcdc_start_time::W`](W) writer structure
impl crate::Writable for DcdcStartTimeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_START_TIME to value 0x0001_193f
impl crate::Resettable for DcdcStartTimeSpec {
    const RESET_VALUE: u32 = 0x0001_193f;
}
