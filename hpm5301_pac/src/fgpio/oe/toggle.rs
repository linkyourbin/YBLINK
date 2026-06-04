///Register `TOGGLE` reader
pub type R = crate::R<ToggleSpec>;
///Register `TOGGLE` writer
pub type W = crate::W<ToggleSpec>;
///Field `DIRECTION` reader - GPIO direction, each bit represents a bus bit 0: input 1: output
pub type DirectionR = crate::FieldReader<u32>;
///Field `DIRECTION` writer - GPIO direction, each bit represents a bus bit 0: input 1: output
pub type DirectionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO direction, each bit represents a bus bit 0: input 1: output
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - GPIO direction, each bit represents a bus bit 0: input 1: output
    #[inline(always)]
    pub fn direction(&mut self) -> DirectionW<'_, ToggleSpec> {
        DirectionW::new(self, 0)
    }
}
/**GPIO direction toggle

You can [`read`](crate::Reg::read) this register and get [`toggle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toggle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ToggleSpec;
impl crate::RegisterSpec for ToggleSpec {
    type Ux = u32;
}
///`read()` method returns [`toggle::R`](R) reader structure
impl crate::Readable for ToggleSpec {}
///`write(|w| ..)` method takes [`toggle::W`](W) writer structure
impl crate::Writable for ToggleSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TOGGLE to value 0
impl crate::Resettable for ToggleSpec {}
