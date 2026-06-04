///Register `SHADOW[%s]` reader
pub type R = crate::R<ShadowSpec>;
///Register `SHADOW[%s]` writer
pub type W = crate::W<ShadowSpec>;
///Field `SHADOW` reader - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128
pub type ShadowR = crate::FieldReader<u32>;
///Field `SHADOW` writer - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128
pub type ShadowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128
    #[inline(always)]
    pub fn shadow(&self) -> ShadowR {
        ShadowR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128
    #[inline(always)]
    pub fn shadow(&mut self) -> ShadowW<'_, ShadowSpec> {
        ShadowW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`shadow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ShadowSpec;
impl crate::RegisterSpec for ShadowSpec {
    type Ux = u32;
}
///`read()` method returns [`shadow::R`](R) reader structure
impl crate::Readable for ShadowSpec {}
///`write(|w| ..)` method takes [`shadow::W`](W) writer structure
impl crate::Writable for ShadowSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHADOW[%s] to value 0
impl crate::Resettable for ShadowSpec {}
