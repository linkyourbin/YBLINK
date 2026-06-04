///Register `RESET_ENABLE` reader
pub type R = crate::R<ResetEnableSpec>;
///Register `RESET_ENABLE` writer
pub type W = crate::W<ResetEnableSpec>;
///Field `ENABLE` reader - enable of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type EnableR = crate::FieldReader<u32>;
///Field `ENABLE` writer - enable of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - enable of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - enable of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ResetEnableSpec> {
        EnableW::new(self, 0)
    }
}
/**reset source enable

You can [`read`](crate::Reg::read) this register and get [`reset_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResetEnableSpec;
impl crate::RegisterSpec for ResetEnableSpec {
    type Ux = u32;
}
///`read()` method returns [`reset_enable::R`](R) reader structure
impl crate::Readable for ResetEnableSpec {}
///`write(|w| ..)` method takes [`reset_enable::W`](W) writer structure
impl crate::Writable for ResetEnableSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESET_ENABLE to value 0xffff_ffff
impl crate::Resettable for ResetEnableSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
