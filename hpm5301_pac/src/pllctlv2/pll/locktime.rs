///Register `LOCKTIME` reader
pub type R = crate::R<LocktimeSpec>;
///Register `LOCKTIME` writer
pub type W = crate::W<LocktimeSpec>;
///Field `LOCKTIME` reader - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting.
pub type LocktimeR = crate::FieldReader<u16>;
///Field `LOCKTIME` writer - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting.
pub type LocktimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting.
    #[inline(always)]
    pub fn locktime(&self) -> LocktimeR {
        LocktimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting.
    #[inline(always)]
    pub fn locktime(&mut self) -> LocktimeW<'_, LocktimeSpec> {
        LocktimeW::new(self, 0)
    }
}
/**PLL0 lock time register

You can [`read`](crate::Reg::read) this register and get [`locktime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locktime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LocktimeSpec;
impl crate::RegisterSpec for LocktimeSpec {
    type Ux = u32;
}
///`read()` method returns [`locktime::R`](R) reader structure
impl crate::Readable for LocktimeSpec {}
///`write(|w| ..)` method takes [`locktime::W`](W) writer structure
impl crate::Writable for LocktimeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKTIME to value 0x09c4
impl crate::Resettable for LocktimeSpec {
    const RESET_VALUE: u32 = 0x09c4;
}
