///Register `MFN` reader
pub type R = crate::R<MfnSpec>;
///Register `MFN` writer
pub type W = crate::W<MfnSpec>;
///Field `MFN` reader - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running.
pub type MfnR = crate::FieldReader<u32>;
///Field `MFN` writer - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running.
pub type MfnW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running.
    #[inline(always)]
    pub fn mfn(&self) -> MfnR {
        MfnR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 0:29 - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running.
    #[inline(always)]
    pub fn mfn(&mut self) -> MfnW<'_, MfnSpec> {
        MfnW::new(self, 0)
    }
}
/**PLL0 fraction numerator register

You can [`read`](crate::Reg::read) this register and get [`mfn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MfnSpec;
impl crate::RegisterSpec for MfnSpec {
    type Ux = u32;
}
///`read()` method returns [`mfn::R`](R) reader structure
impl crate::Readable for MfnSpec {}
///`write(|w| ..)` method takes [`mfn::W`](W) writer structure
impl crate::Writable for MfnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MFN to value 0x0989_6800
impl crate::Resettable for MfnSpec {
    const RESET_VALUE: u32 = 0x0989_6800;
}
