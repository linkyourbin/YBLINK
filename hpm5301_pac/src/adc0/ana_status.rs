///Register `ana_status` reader
pub type R = crate::R<AnaStatusSpec>;
///Register `ana_status` writer
pub type W = crate::W<AnaStatusSpec>;
///Field `CALON` reader - Indicates if the ADC is in calibration mode (Active H).
pub type CalonR = crate::BitReader;
///Field `CALON` writer - Indicates if the ADC is in calibration mode (Active H).
pub type CalonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Indicates if the ADC is in calibration mode (Active H).
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - Indicates if the ADC is in calibration mode (Active H).
    #[inline(always)]
    pub fn calon(&mut self) -> CalonW<'_, AnaStatusSpec> {
        CalonW::new(self, 7)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`ana_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AnaStatusSpec;
impl crate::RegisterSpec for AnaStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`ana_status::R`](R) reader structure
impl crate::Readable for AnaStatusSpec {}
///`write(|w| ..)` method takes [`ana_status::W`](W) writer structure
impl crate::Writable for AnaStatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ana_status to value 0
impl crate::Resettable for AnaStatusSpec {}
