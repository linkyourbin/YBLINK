///Register `RESET_TYPE` reader
pub type R = crate::R<ResetTypeSpec>;
///Register `RESET_TYPE` writer
pub type W = crate::W<ResetTypeSpec>;
///Field `TYPE` reader - reset type of reset sources, 0 for cold reset, all system control setting cleared except debug/fuse/ioc; 1 for hot reset, keep system control setting and debug/fuse/ioc setting, only clear some subsystem 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type TypeR = crate::FieldReader<u32>;
///Field `TYPE` writer - reset type of reset sources, 0 for cold reset, all system control setting cleared except debug/fuse/ioc; 1 for hot reset, keep system control setting and debug/fuse/ioc setting, only clear some subsystem 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reset type of reset sources, 0 for cold reset, all system control setting cleared except debug/fuse/ioc; 1 for hot reset, keep system control setting and debug/fuse/ioc setting, only clear some subsystem 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - reset type of reset sources, 0 for cold reset, all system control setting cleared except debug/fuse/ioc; 1 for hot reset, keep system control setting and debug/fuse/ioc setting, only clear some subsystem 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, ResetTypeSpec> {
        TypeW::new(self, 0)
    }
}
/**reset type triggered by reset

You can [`read`](crate::Reg::read) this register and get [`reset_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResetTypeSpec;
impl crate::RegisterSpec for ResetTypeSpec {
    type Ux = u32;
}
///`read()` method returns [`reset_type::R`](R) reader structure
impl crate::Readable for ResetTypeSpec {}
///`write(|w| ..)` method takes [`reset_type::W`](W) writer structure
impl crate::Writable for ResetTypeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESET_TYPE to value 0
impl crate::Resettable for ResetTypeSpec {}
