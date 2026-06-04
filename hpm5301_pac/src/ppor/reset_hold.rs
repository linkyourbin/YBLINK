///Register `RESET_HOLD` reader
pub type R = crate::R<ResetHoldSpec>;
///Register `RESET_HOLD` writer
pub type W = crate::W<ResetHoldSpec>;
///Field `HOLD` reader - hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type HoldR = crate::FieldReader<u32>;
///Field `HOLD` writer - hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type HoldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<'_, ResetHoldSpec> {
        HoldW::new(self, 0)
    }
}
/**reset hold attribute

You can [`read`](crate::Reg::read) this register and get [`reset_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResetHoldSpec;
impl crate::RegisterSpec for ResetHoldSpec {
    type Ux = u32;
}
///`read()` method returns [`reset_hold::R`](R) reader structure
impl crate::Readable for ResetHoldSpec {}
///`write(|w| ..)` method takes [`reset_hold::W`](W) writer structure
impl crate::Writable for ResetHoldSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESET_HOLD to value 0
impl crate::Resettable for ResetHoldSpec {}
