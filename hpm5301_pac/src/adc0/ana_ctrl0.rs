///Register `ana_ctrl0` reader
pub type R = crate::R<AnaCtrl0Spec>;
///Register `ana_ctrl0` writer
pub type W = crate::W<AnaCtrl0Spec>;
///Field `STARTCAL` reader - set to start the offset calibration cycle (Active H). user need to clear it after setting it.
pub type StartcalR = crate::BitReader;
///Field `STARTCAL` writer - set to start the offset calibration cycle (Active H). user need to clear it after setting it.
pub type StartcalW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CLK_ON` reader - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access
pub type AdcClkOnR = crate::BitReader;
///Field `ADC_CLK_ON` writer - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access
pub type AdcClkOnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOTO_EN` reader - "set to enable moto_soc and moto_valid. Should use AHB clock for adc, this bit can be used avoid async output"
pub type MotoEnR = crate::BitReader;
///Field `MOTO_EN` writer - "set to enable moto_soc and moto_valid. Should use AHB clock for adc, this bit can be used avoid async output"
pub type MotoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it.
    #[inline(always)]
    pub fn startcal(&self) -> StartcalR {
        StartcalR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 12 - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access
    #[inline(always)]
    pub fn adc_clk_on(&self) -> AdcClkOnR {
        AdcClkOnR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 31 - "set to enable moto_soc and moto_valid. Should use AHB clock for adc, this bit can be used avoid async output"
    #[inline(always)]
    pub fn moto_en(&self) -> MotoEnR {
        MotoEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it.
    #[inline(always)]
    pub fn startcal(&mut self) -> StartcalW<'_, AnaCtrl0Spec> {
        StartcalW::new(self, 2)
    }
    ///Bit 12 - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access
    #[inline(always)]
    pub fn adc_clk_on(&mut self) -> AdcClkOnW<'_, AnaCtrl0Spec> {
        AdcClkOnW::new(self, 12)
    }
    ///Bit 31 - "set to enable moto_soc and moto_valid. Should use AHB clock for adc, this bit can be used avoid async output"
    #[inline(always)]
    pub fn moto_en(&mut self) -> MotoEnW<'_, AnaCtrl0Spec> {
        MotoEnW::new(self, 31)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`ana_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AnaCtrl0Spec;
impl crate::RegisterSpec for AnaCtrl0Spec {
    type Ux = u32;
}
///`read()` method returns [`ana_ctrl0::R`](R) reader structure
impl crate::Readable for AnaCtrl0Spec {}
///`write(|w| ..)` method takes [`ana_ctrl0::W`](W) writer structure
impl crate::Writable for AnaCtrl0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ana_ctrl0 to value 0
impl crate::Resettable for AnaCtrl0Spec {}
