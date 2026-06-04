///Register `REF_PROT` reader
pub type R = crate::R<RefProtSpec>;
///Register `REF_PROT` writer
pub type W = crate::W<RefProtSpec>;
///Field `REF_UNL_PSD` reader - The password to unlock refreshing
pub type RefUnlPsdR = crate::FieldReader<u16>;
///Field `REF_UNL_PSD` writer - The password to unlock refreshing
pub type RefUnlPsdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The password to unlock refreshing
    #[inline(always)]
    pub fn ref_unl_psd(&self) -> RefUnlPsdR {
        RefUnlPsdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - The password to unlock refreshing
    #[inline(always)]
    pub fn ref_unl_psd(&mut self) -> RefUnlPsdW<'_, RefProtSpec> {
        RefUnlPsdW::new(self, 0)
    }
}
/**refresh protection register

You can [`read`](crate::Reg::read) this register and get [`ref_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RefProtSpec;
impl crate::RegisterSpec for RefProtSpec {
    type Ux = u32;
}
///`read()` method returns [`ref_prot::R`](R) reader structure
impl crate::Readable for RefProtSpec {}
///`write(|w| ..)` method takes [`ref_prot::W`](W) writer structure
impl crate::Writable for RefProtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REF_PROT to value 0
impl crate::Resettable for RefProtSpec {}
