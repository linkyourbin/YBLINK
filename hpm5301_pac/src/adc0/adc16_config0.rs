///Register `adc16_config0` reader
pub type R = crate::R<Adc16Config0Spec>;
///Register `adc16_config0` writer
pub type W = crate::W<Adc16Config0Spec>;
///Field `CONV_PARAM` reader - conversion parameter
pub type ConvParamR = crate::FieldReader<u16>;
///Field `CONV_PARAM` writer - conversion parameter
pub type ConvParamW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `CAL_AVG_CFG` reader - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved
pub type CalAvgCfgR = crate::FieldReader;
///Field `CAL_AVG_CFG` writer - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved
pub type CalAvgCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BANDGAP_EN` reader - set to enable bandgap. user should set reg_en and bandgap_en before use adc16.
pub type BandgapEnR = crate::BitReader;
///Field `BANDGAP_EN` writer - set to enable bandgap. user should set reg_en and bandgap_en before use adc16.
pub type BandgapEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_EN` reader - set to enable regulator
pub type RegEnR = crate::BitReader;
///Field `REG_EN` writer - set to enable regulator
pub type RegEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:13 - conversion parameter
    #[inline(always)]
    pub fn conv_param(&self) -> ConvParamR {
        ConvParamR::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 20:22 - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved
    #[inline(always)]
    pub fn cal_avg_cfg(&self) -> CalAvgCfgR {
        CalAvgCfgR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - set to enable bandgap. user should set reg_en and bandgap_en before use adc16.
    #[inline(always)]
    pub fn bandgap_en(&self) -> BandgapEnR {
        BandgapEnR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - set to enable regulator
    #[inline(always)]
    pub fn reg_en(&self) -> RegEnR {
        RegEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:13 - conversion parameter
    #[inline(always)]
    pub fn conv_param(&mut self) -> ConvParamW<'_, Adc16Config0Spec> {
        ConvParamW::new(self, 0)
    }
    ///Bits 20:22 - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved
    #[inline(always)]
    pub fn cal_avg_cfg(&mut self) -> CalAvgCfgW<'_, Adc16Config0Spec> {
        CalAvgCfgW::new(self, 20)
    }
    ///Bit 23 - set to enable bandgap. user should set reg_en and bandgap_en before use adc16.
    #[inline(always)]
    pub fn bandgap_en(&mut self) -> BandgapEnW<'_, Adc16Config0Spec> {
        BandgapEnW::new(self, 23)
    }
    ///Bit 24 - set to enable regulator
    #[inline(always)]
    pub fn reg_en(&mut self) -> RegEnW<'_, Adc16Config0Spec> {
        RegEnW::new(self, 24)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`adc16_config0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc16_config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adc16Config0Spec;
impl crate::RegisterSpec for Adc16Config0Spec {
    type Ux = u32;
}
///`read()` method returns [`adc16_config0::R`](R) reader structure
impl crate::Readable for Adc16Config0Spec {}
///`write(|w| ..)` method takes [`adc16_config0::W`](W) writer structure
impl crate::Writable for Adc16Config0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets adc16_config0 to value 0
impl crate::Resettable for Adc16Config0Spec {}
