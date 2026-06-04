///Register `DAC_MATRIX_SEL` reader
pub type R = crate::R<DacMatrixSelSpec>;
///Register `DAC_MATRIX_SEL` writer
pub type W = crate::W<DacMatrixSelSpec>;
///Field `ACMP0_DAC_SEL` reader - No description available
pub type Acmp0DacSelR = crate::FieldReader;
///Field `ACMP0_DAC_SEL` writer - No description available
pub type Acmp0DacSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ACMP1_DAC_SEL` reader - No description available
pub type Acmp1DacSelR = crate::FieldReader;
///Field `ACMP1_DAC_SEL` writer - No description available
pub type Acmp1DacSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DAC0_DAC_SEL` reader - No description available
pub type Dac0DacSelR = crate::FieldReader;
///Field `DAC0_DAC_SEL` writer - No description available
pub type Dac0DacSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DAC1_DAC_SEL` reader - 0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved
pub type Dac1DacSelR = crate::FieldReader;
///Field `DAC1_DAC_SEL` writer - 0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved
pub type Dac1DacSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn acmp0_dac_sel(&self) -> Acmp0DacSelR {
        Acmp0DacSelR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn acmp1_dac_sel(&self) -> Acmp1DacSelR {
        Acmp1DacSelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - No description available
    #[inline(always)]
    pub fn dac0_dac_sel(&self) -> Dac0DacSelR {
        Dac0DacSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - 0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved
    #[inline(always)]
    pub fn dac1_dac_sel(&self) -> Dac1DacSelR {
        Dac1DacSelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn acmp0_dac_sel(&mut self) -> Acmp0DacSelW<'_, DacMatrixSelSpec> {
        Acmp0DacSelW::new(self, 0)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn acmp1_dac_sel(&mut self) -> Acmp1DacSelW<'_, DacMatrixSelSpec> {
        Acmp1DacSelW::new(self, 8)
    }
    ///Bits 16:23 - No description available
    #[inline(always)]
    pub fn dac0_dac_sel(&mut self) -> Dac0DacSelW<'_, DacMatrixSelSpec> {
        Dac0DacSelW::new(self, 16)
    }
    ///Bits 24:31 - 0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved
    #[inline(always)]
    pub fn dac1_dac_sel(&mut self) -> Dac1DacSelW<'_, DacMatrixSelSpec> {
        Dac1DacSelW::new(self, 24)
    }
}
/**dac matrix select register

You can [`read`](crate::Reg::read) this register and get [`dac_matrix_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_matrix_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DacMatrixSelSpec;
impl crate::RegisterSpec for DacMatrixSelSpec {
    type Ux = u32;
}
///`read()` method returns [`dac_matrix_sel::R`](R) reader structure
impl crate::Readable for DacMatrixSelSpec {}
///`write(|w| ..)` method takes [`dac_matrix_sel::W`](W) writer structure
impl crate::Writable for DacMatrixSelSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAC_MATRIX_SEL to value 0
impl crate::Resettable for DacMatrixSelSpec {}
