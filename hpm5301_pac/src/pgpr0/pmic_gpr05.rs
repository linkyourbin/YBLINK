///Register `PMIC_GPR05` reader
pub type R = crate::R<PmicGpr05Spec>;
///Register `PMIC_GPR05` writer
pub type W = crate::W<PmicGpr05Spec>;
///Field `GPR` reader - Generic control
pub type GprR = crate::FieldReader<u32>;
///Field `GPR` writer - Generic control
pub type GprW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Generic control
    #[inline(always)]
    pub fn gpr(&self) -> GprR {
        GprR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Generic control
    #[inline(always)]
    pub fn gpr(&mut self) -> GprW<'_, PmicGpr05Spec> {
        GprW::new(self, 0)
    }
}
/**Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr05::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr05::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmicGpr05Spec;
impl crate::RegisterSpec for PmicGpr05Spec {
    type Ux = u32;
}
///`read()` method returns [`pmic_gpr05::R`](R) reader structure
impl crate::Readable for PmicGpr05Spec {}
///`write(|w| ..)` method takes [`pmic_gpr05::W`](W) writer structure
impl crate::Writable for PmicGpr05Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMIC_GPR05 to value 0
impl crate::Resettable for PmicGpr05Spec {}
