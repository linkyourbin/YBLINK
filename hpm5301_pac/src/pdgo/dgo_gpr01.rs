///Register `DGO_GPR01` reader
pub type R = crate::R<DgoGpr01Spec>;
///Register `DGO_GPR01` writer
pub type W = crate::W<DgoGpr01Spec>;
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
    pub fn gpr(&mut self) -> GprW<'_, DgoGpr01Spec> {
        GprW::new(self, 0)
    }
}
/**Generic control 1

You can [`read`](crate::Reg::read) this register and get [`dgo_gpr01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_gpr01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoGpr01Spec;
impl crate::RegisterSpec for DgoGpr01Spec {
    type Ux = u32;
}
///`read()` method returns [`dgo_gpr01::R`](R) reader structure
impl crate::Readable for DgoGpr01Spec {}
///`write(|w| ..)` method takes [`dgo_gpr01::W`](W) writer structure
impl crate::Writable for DgoGpr01Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_GPR01 to value 0
impl crate::Resettable for DgoGpr01Spec {}
