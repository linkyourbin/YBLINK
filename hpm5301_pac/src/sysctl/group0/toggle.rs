///Register `TOGGLE` reader
pub type R = crate::R<ToggleSpec>;
///Register `TOGGLE` writer
pub type W = crate::W<ToggleSpec>;
///Field `LINK` reader - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before
pub type LinkR = crate::FieldReader<u32>;
///Field `LINK` writer - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, ToggleSpec> {
        LinkW::new(self, 0)
    }
}
/**Group setting

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
