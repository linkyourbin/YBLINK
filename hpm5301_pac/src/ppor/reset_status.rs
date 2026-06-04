///Register `RESET_STATUS` reader
pub type R = crate::R<ResetStatusSpec>;
///Register `RESET_STATUS` writer
pub type W = crate::W<ResetStatusSpec>;
///Field `STATUS` reader - current status of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type StatusR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - current status of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {}
/**reset source status

You can [`read`](crate::Reg::read) this register and get [`reset_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResetStatusSpec;
impl crate::RegisterSpec for ResetStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`reset_status::R`](R) reader structure
impl crate::Readable for ResetStatusSpec {}
///`write(|w| ..)` method takes [`reset_status::W`](W) writer structure
impl crate::Writable for ResetStatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESET_STATUS to value 0
impl crate::Resettable for ResetStatusSpec {}
