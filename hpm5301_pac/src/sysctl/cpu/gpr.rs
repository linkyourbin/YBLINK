///Register `GPR[%s]` reader
pub type R = crate::R<GprSpec>;
///Register `GPR[%s]` writer
pub type W = crate::W<GprSpec>;
///Field `GPR` reader - register for software to handle resume, can save resume address or status
pub type GprR = crate::FieldReader<u32>;
///Field `GPR` writer - register for software to handle resume, can save resume address or status
pub type GprW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - register for software to handle resume, can save resume address or status
    #[inline(always)]
    pub fn gpr(&self) -> GprR {
        GprR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - register for software to handle resume, can save resume address or status
    #[inline(always)]
    pub fn gpr(&mut self) -> GprW<'_, GprSpec> {
        GprW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`gpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GprSpec;
impl crate::RegisterSpec for GprSpec {
    type Ux = u32;
}
///`read()` method returns [`gpr::R`](R) reader structure
impl crate::Readable for GprSpec {}
///`write(|w| ..)` method takes [`gpr::W`](W) writer structure
impl crate::Writable for GprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPR[%s] to value 0
impl crate::Resettable for GprSpec {}
