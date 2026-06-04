///Register `SAMPLE_CFG[%s]` reader
pub type R = crate::R<SampleCfgSpec>;
///Register `SAMPLE_CFG[%s]` writer
pub type W = crate::W<SampleCfgSpec>;
///Field `SAMPLE_CLOCK_NUMBER` reader - sample clock number, base on clock_period, default one period
pub type SampleClockNumberR = crate::FieldReader<u16>;
///Field `SAMPLE_CLOCK_NUMBER` writer - sample clock number, base on clock_period, default one period
pub type SampleClockNumberW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SAMPLE_CLOCK_NUMBER_SHIFT` reader - shift for sample clock number
pub type SampleClockNumberShiftR = crate::FieldReader;
///Field `SAMPLE_CLOCK_NUMBER_SHIFT` writer - shift for sample clock number
pub type SampleClockNumberShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:8 - sample clock number, base on clock_period, default one period
    #[inline(always)]
    pub fn sample_clock_number(&self) -> SampleClockNumberR {
        SampleClockNumberR::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:11 - shift for sample clock number
    #[inline(always)]
    pub fn sample_clock_number_shift(&self) -> SampleClockNumberShiftR {
        SampleClockNumberShiftR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    ///Bits 0:8 - sample clock number, base on clock_period, default one period
    #[inline(always)]
    pub fn sample_clock_number(&mut self) -> SampleClockNumberW<'_, SampleCfgSpec> {
        SampleClockNumberW::new(self, 0)
    }
    ///Bits 9:11 - shift for sample clock number
    #[inline(always)]
    pub fn sample_clock_number_shift(&mut self) -> SampleClockNumberShiftW<'_, SampleCfgSpec> {
        SampleClockNumberShiftW::new(self, 9)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`sample_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SampleCfgSpec;
impl crate::RegisterSpec for SampleCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`sample_cfg::R`](R) reader structure
impl crate::Readable for SampleCfgSpec {}
///`write(|w| ..)` method takes [`sample_cfg::W`](W) writer structure
impl crate::Writable for SampleCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SAMPLE_CFG[%s] to value 0x01
impl crate::Resettable for SampleCfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
