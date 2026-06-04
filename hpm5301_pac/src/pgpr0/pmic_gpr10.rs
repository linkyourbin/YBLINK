///Register `PMIC_GPR10` reader
pub type R = crate::R<PmicGpr10Spec>;
///Register `PMIC_GPR10` writer
pub type W = crate::W<PmicGpr10Spec>;
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
    pub fn gpr(&mut self) -> GprW<'_, PmicGpr10Spec> {
        GprW::new(self, 0)
    }
}
/**Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmicGpr10Spec;
impl crate::RegisterSpec for PmicGpr10Spec {
    type Ux = u32;
}
///`read()` method returns [`pmic_gpr10::R`](R) reader structure
impl crate::Readable for PmicGpr10Spec {}
///`write(|w| ..)` method takes [`pmic_gpr10::W`](W) writer structure
impl crate::Writable for PmicGpr10Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMIC_GPR10 to value 0
impl crate::Resettable for PmicGpr10Spec {}
