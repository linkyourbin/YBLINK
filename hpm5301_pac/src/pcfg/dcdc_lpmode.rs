///Register `DCDC_LPMODE` reader
pub type R = crate::R<DcdcLpmodeSpec>;
///Register `DCDC_LPMODE` writer
pub type W = crate::W<DcdcLpmodeSpec>;
///Field `STBY_VOLT` reader - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
pub type StbyVoltR = crate::FieldReader<u16>;
///Field `STBY_VOLT` writer - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
pub type StbyVoltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
    #[inline(always)]
    pub fn stby_volt(&self) -> StbyVoltR {
        StbyVoltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
    #[inline(always)]
    pub fn stby_volt(&mut self) -> StbyVoltW<'_, DcdcLpmodeSpec> {
        StbyVoltW::new(self, 0)
    }
}
/**DCDC low power mode

You can [`read`](crate::Reg::read) this register and get [`dcdc_lpmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_lpmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcLpmodeSpec;
impl crate::RegisterSpec for DcdcLpmodeSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_lpmode::R`](R) reader structure
impl crate::Readable for DcdcLpmodeSpec {}
///`write(|w| ..)` method takes [`dcdc_lpmode::W`](W) writer structure
impl crate::Writable for DcdcLpmodeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_LPMODE to value 0x0384
impl crate::Resettable for DcdcLpmodeSpec {
    const RESET_VALUE: u32 = 0x0384;
}
