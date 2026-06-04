///Register `PMIC_GPR07` reader
pub type R = crate::R<PmicGpr07Spec>;
///Register `PMIC_GPR07` writer
pub type W = crate::W<PmicGpr07Spec>;
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
    pub fn gpr(&mut self) -> GprW<'_, PmicGpr07Spec> {
        GprW::new(self, 0)
    }
}
/**Generic control

You can [`read`](crate::Reg::read) this register and get [`pmic_gpr07::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_gpr07::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmicGpr07Spec;
impl crate::RegisterSpec for PmicGpr07Spec {
    type Ux = u32;
}
///`read()` method returns [`pmic_gpr07::R`](R) reader structure
impl crate::Readable for PmicGpr07Spec {}
///`write(|w| ..)` method takes [`pmic_gpr07::W`](W) writer structure
impl crate::Writable for PmicGpr07Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMIC_GPR07 to value 0
impl crate::Resettable for PmicGpr07Spec {}
