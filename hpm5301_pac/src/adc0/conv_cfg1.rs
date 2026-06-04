///Register `conv_cfg1` reader
pub type R = crate::R<ConvCfg1Spec>;
///Register `conv_cfg1` writer
pub type W = crate::W<ConvCfg1Spec>;
///Field `CLOCK_DIVIDER` reader - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk
pub type ClockDividerR = crate::FieldReader;
///Field `CLOCK_DIVIDER` writer - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk
pub type ClockDividerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CONVERT_CLOCK_NUMBER` reader - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz).
pub type ConvertClockNumberR = crate::FieldReader;
///Field `CONVERT_CLOCK_NUMBER` writer - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz).
pub type ConvertClockNumberW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:3 - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk
    #[inline(always)]
    pub fn clock_divider(&self) -> ClockDividerR {
        ClockDividerR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:8 - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz).
    #[inline(always)]
    pub fn convert_clock_number(&self) -> ConvertClockNumberR {
        ConvertClockNumberR::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:3 - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk
    #[inline(always)]
    pub fn clock_divider(&mut self) -> ClockDividerW<'_, ConvCfg1Spec> {
        ClockDividerW::new(self, 0)
    }
    ///Bits 4:8 - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz).
    #[inline(always)]
    pub fn convert_clock_number(&mut self) -> ConvertClockNumberW<'_, ConvCfg1Spec> {
        ConvertClockNumberW::new(self, 4)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`conv_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ConvCfg1Spec;
impl crate::RegisterSpec for ConvCfg1Spec {
    type Ux = u32;
}
///`read()` method returns [`conv_cfg1::R`](R) reader structure
impl crate::Readable for ConvCfg1Spec {}
///`write(|w| ..)` method takes [`conv_cfg1::W`](W) writer structure
impl crate::Writable for ConvCfg1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets conv_cfg1 to value 0
impl crate::Resettable for ConvCfg1Spec {}
