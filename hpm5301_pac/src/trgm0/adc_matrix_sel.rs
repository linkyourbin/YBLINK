///Register `ADC_MATRIX_SEL` reader
pub type R = crate::R<AdcMatrixSelSpec>;
///Register `ADC_MATRIX_SEL` writer
pub type W = crate::W<AdcMatrixSelSpec>;
///Field `QEI0_ADC0_SEL` reader - No description available
pub type Qei0Adc0SelR = crate::FieldReader;
///Field `QEI0_ADC0_SEL` writer - No description available
pub type Qei0Adc0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QEI0_ADC1_SEL` reader - No description available
pub type Qei0Adc1SelR = crate::FieldReader;
///Field `QEI0_ADC1_SEL` writer - No description available
pub type Qei0Adc1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QEI1_ADC0_SEL` reader - No description available
pub type Qei1Adc0SelR = crate::FieldReader;
///Field `QEI1_ADC0_SEL` writer - No description available
pub type Qei1Adc0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QEI1_ADC1_SEL` reader - 0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved
pub type Qei1Adc1SelR = crate::FieldReader;
///Field `QEI1_ADC1_SEL` writer - 0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved
pub type Qei1Adc1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn qei0_adc0_sel(&self) -> Qei0Adc0SelR {
        Qei0Adc0SelR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn qei0_adc1_sel(&self) -> Qei0Adc1SelR {
        Qei0Adc1SelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - No description available
    #[inline(always)]
    pub fn qei1_adc0_sel(&self) -> Qei1Adc0SelR {
        Qei1Adc0SelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - 0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved
    #[inline(always)]
    pub fn qei1_adc1_sel(&self) -> Qei1Adc1SelR {
        Qei1Adc1SelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn qei0_adc0_sel(&mut self) -> Qei0Adc0SelW<'_, AdcMatrixSelSpec> {
        Qei0Adc0SelW::new(self, 0)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn qei0_adc1_sel(&mut self) -> Qei0Adc1SelW<'_, AdcMatrixSelSpec> {
        Qei0Adc1SelW::new(self, 8)
    }
    ///Bits 16:23 - No description available
    #[inline(always)]
    pub fn qei1_adc0_sel(&mut self) -> Qei1Adc0SelW<'_, AdcMatrixSelSpec> {
        Qei1Adc0SelW::new(self, 16)
    }
    ///Bits 24:31 - 0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved
    #[inline(always)]
    pub fn qei1_adc1_sel(&mut self) -> Qei1Adc1SelW<'_, AdcMatrixSelSpec> {
        Qei1Adc1SelW::new(self, 24)
    }
}
/**adc matrix select register

You can [`read`](crate::Reg::read) this register and get [`adc_matrix_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_matrix_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcMatrixSelSpec;
impl crate::RegisterSpec for AdcMatrixSelSpec {
    type Ux = u32;
}
///`read()` method returns [`adc_matrix_sel::R`](R) reader structure
impl crate::Readable for AdcMatrixSelSpec {}
///`write(|w| ..)` method takes [`adc_matrix_sel::W`](W) writer structure
impl crate::Writable for AdcMatrixSelSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_MATRIX_SEL to value 0
impl crate::Resettable for AdcMatrixSelSpec {}
