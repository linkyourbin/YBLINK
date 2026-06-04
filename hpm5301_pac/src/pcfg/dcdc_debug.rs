///Register `DCDC_DEBUG` reader
pub type R = crate::R<DcdcDebugSpec>;
///Register `DCDC_DEBUG` writer
pub type W = crate::W<DcdcDebugSpec>;
///Field `UPDATE_TIME` reader - DCDC voltage change time in 24M clock cycles, default value is 1mS
pub type UpdateTimeR = crate::FieldReader<u32>;
///Field `UPDATE_TIME` writer - DCDC voltage change time in 24M clock cycles, default value is 1mS
pub type UpdateTimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - DCDC voltage change time in 24M clock cycles, default value is 1mS
    #[inline(always)]
    pub fn update_time(&self) -> UpdateTimeR {
        UpdateTimeR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - DCDC voltage change time in 24M clock cycles, default value is 1mS
    #[inline(always)]
    pub fn update_time(&mut self) -> UpdateTimeW<'_, DcdcDebugSpec> {
        UpdateTimeW::new(self, 0)
    }
}
/**DCDC Debug

You can [`read`](crate::Reg::read) this register and get [`dcdc_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcDebugSpec;
impl crate::RegisterSpec for DcdcDebugSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_debug::R`](R) reader structure
impl crate::Readable for DcdcDebugSpec {}
///`write(|w| ..)` method takes [`dcdc_debug::W`](W) writer structure
impl crate::Writable for DcdcDebugSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_DEBUG to value 0x5dbf
impl crate::Resettable for DcdcDebugSpec {
    const RESET_VALUE: u32 = 0x5dbf;
}
