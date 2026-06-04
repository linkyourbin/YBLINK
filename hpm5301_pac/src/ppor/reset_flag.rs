///Register `RESET_FLAG` reader
pub type R = crate::R<ResetFlagSpec>;
///Register `RESET_FLAG` writer
pub type W = crate::W<ResetFlagSpec>;
///Field `FLAG` writer - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, ResetFlagSpec> {
        FlagW::new(self, 0)
    }
}
/**flag indicate reset source

You can [`read`](crate::Reg::read) this register and get [`reset_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResetFlagSpec;
impl crate::RegisterSpec for ResetFlagSpec {
    type Ux = u32;
}
///`read()` method returns [`reset_flag::R`](R) reader structure
impl crate::Readable for ResetFlagSpec {}
///`write(|w| ..)` method takes [`reset_flag::W`](W) writer structure
impl crate::Writable for ResetFlagSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESET_FLAG to value 0
impl crate::Resettable for ResetFlagSpec {}
