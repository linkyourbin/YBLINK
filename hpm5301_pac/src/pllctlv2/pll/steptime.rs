///Register `STEPTIME` reader
pub type R = crate::R<SteptimeSpec>;
///Register `STEPTIME` writer
pub type W = crate::W<SteptimeSpec>;
///Field `STEPTIME` reader - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500.
pub type SteptimeR = crate::FieldReader<u16>;
///Field `STEPTIME` writer - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500.
pub type SteptimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500.
    #[inline(always)]
    pub fn steptime(&self) -> SteptimeR {
        SteptimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500.
    #[inline(always)]
    pub fn steptime(&mut self) -> SteptimeW<'_, SteptimeSpec> {
        SteptimeW::new(self, 0)
    }
}
/**PLL0 step time register

You can [`read`](crate::Reg::read) this register and get [`steptime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`steptime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SteptimeSpec;
impl crate::RegisterSpec for SteptimeSpec {
    type Ux = u32;
}
///`read()` method returns [`steptime::R`](R) reader structure
impl crate::Readable for SteptimeSpec {}
///`write(|w| ..)` method takes [`steptime::W`](W) writer structure
impl crate::Writable for SteptimeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STEPTIME to value 0x09c4
impl crate::Resettable for SteptimeSpec {
    const RESET_VALUE: u32 = 0x09c4;
}
