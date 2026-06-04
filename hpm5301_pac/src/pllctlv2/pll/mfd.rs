///Register `MFD` reader
pub type R = crate::R<MfdSpec>;
///Register `MFD` writer
pub type W = crate::W<MfdSpec>;
///Field `MFD` reader - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled.
pub type MfdR = crate::FieldReader<u32>;
///Field `MFD` writer - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled.
pub type MfdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled.
    #[inline(always)]
    pub fn mfd(&self) -> MfdR {
        MfdR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 0:29 - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled.
    #[inline(always)]
    pub fn mfd(&mut self) -> MfdW<'_, MfdSpec> {
        MfdW::new(self, 0)
    }
}
/**PLL0 fraction demoninator register

You can [`read`](crate::Reg::read) this register and get [`mfd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MfdSpec;
impl crate::RegisterSpec for MfdSpec {
    type Ux = u32;
}
///`read()` method returns [`mfd::R`](R) reader structure
impl crate::Readable for MfdSpec {}
///`write(|w| ..)` method takes [`mfd::W`](W) writer structure
impl crate::Writable for MfdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MFD to value 0x0e4e_1c00
impl crate::Resettable for MfdSpec {
    const RESET_VALUE: u32 = 0x0e4e_1c00;
}
